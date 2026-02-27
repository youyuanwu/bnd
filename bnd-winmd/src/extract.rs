//! Extraction — clang `Entity`/`Type` → intermediate model types.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use std::collections::HashSet;

use clang::{
    CallingConvention, Entity, EntityKind, Index, Type as ClangType, TypeKind,
    sonar::{self, Declaration, DefinitionValue},
};
use tracing::{debug, trace, warn};

use crate::config::{self, PartitionConfig};
use crate::model::*;

/// Extract all declarations from a single partition into model types.
pub fn extract_partition(
    index: &Index,
    partition: &PartitionConfig,
    base_dir: &Path,
    include_paths: &[PathBuf],
    global_clang_args: &[String],
    namespace_overrides: &std::collections::HashMap<String, String>,
) -> Result<Partition> {
    let _ = namespace_overrides; // reserved for future per-API namespace overrides
    let header_path = partition.wrapper_header(base_dir, include_paths);
    debug!(header = %header_path.display(), namespace = %partition.namespace, "parsing partition");

    // Build clang arguments: global args + per-partition args + -I flags.
    // Include base_dir so that wrapper files (in /tmp/) can find headers
    // via angle-bracket includes relative to the TOML config directory.
    let mut all_args: Vec<String> = global_clang_args.to_vec();
    for arg in &partition.clang_args {
        if !all_args.contains(arg) {
            all_args.push(arg.clone());
        }
    }
    let base_flag = format!("-I{}", base_dir.display());
    if !all_args.contains(&base_flag) {
        all_args.push(base_flag);
    }
    for inc in include_paths {
        let flag = format!("-I{}", inc.display());
        if !all_args.contains(&flag) {
            all_args.push(flag);
        }
    }

    let tu = index
        .parser(header_path.to_str().unwrap())
        .arguments(&all_args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        .detailed_preprocessing_record(true)
        .parse()
        .map_err(|e| anyhow::anyhow!("failed to parse {}: {:?}", header_path.display(), e))?;

    // Resolve traverse files through include_paths so relative names work
    let resolved_traverse: Vec<PathBuf> = partition
        .traverse_files()
        .iter()
        .map(|t| config::resolve_header(t, base_dir, include_paths))
        .collect();
    let entities = tu.get_entity().get_children();

    let in_scope = |e: &Entity| should_emit(e, &resolved_traverse, base_dir);

    let structs = collect_structs(&entities, &in_scope);
    let (enums, anon_enum_constants) = collect_enums(&entities, &in_scope);
    let functions = collect_functions(&entities, &in_scope);
    let typedefs = collect_typedefs(&entities, &in_scope);
    let mut constants = collect_constants(&entities, &in_scope);

    // Merge in constants extracted from anonymous enums
    constants.extend(anon_enum_constants);

    tracing::info!(
        namespace = %partition.namespace,
        structs = structs.len(),
        enums = enums.len(),
        functions = functions.len(),
        typedefs = typedefs.len(),
        constants = constants.len(),
        "partition extraction complete"
    );

    Ok(Partition {
        namespace: partition.namespace.clone(),
        library: partition.library.clone(),
        structs,
        enums,
        functions,
        typedefs,
        constants,
    })
}

// ---------------------------------------------------------------------------
// Collection helpers — one per declaration kind
// ---------------------------------------------------------------------------

/// Collect structs via sonar, then run a supplemental pass for StructDecl
/// entities that sonar missed (e.g. structs that only have a pointer typedef).
fn collect_structs(entities: &[Entity], in_scope: &impl Fn(&Entity) -> bool) -> Vec<StructDef> {
    let mut structs = Vec::new();
    let mut seen = HashSet::new();

    // Primary: sonar-discovered structs (via typedef patterns)
    for decl in sonar::find_structs(entities.to_vec()) {
        if !in_scope(&decl.entity) {
            trace_out_of_scope(&decl.entity, "struct");
            continue;
        }
        seen.insert(decl.name.clone());
        match extract_struct(&decl) {
            Ok((s, nested)) => {
                debug!(name = %s.name, fields = s.fields.len(), size = s.size, "extracted struct");
                for ns in nested {
                    seen.insert(ns.name.clone());
                    debug!(name = %ns.name, fields = ns.fields.len(), "  nested anonymous type");
                    structs.push(ns);
                }
                structs.push(s);
            }
            Err(e) => warn!(name = %decl.name, err = %e, "skipping struct"),
        }
    }

    // Supplemental: StructDecl/UnionDecl entities with full definitions that
    // sonar missed (e.g. `struct gzFile_s` which only has a pointer typedef,
    // or any union — sonar has no find_unions).
    for entity in entities {
        let is_union = match entity.get_kind() {
            EntityKind::StructDecl => false,
            EntityKind::UnionDecl => true,
            _ => continue,
        };
        if !in_scope(entity) {
            trace_out_of_scope(entity, "struct");
            continue;
        }
        let name = match entity.get_name() {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };
        if seen.contains(&name) || !entity.is_definition() {
            continue;
        }
        seen.insert(name.clone());
        match extract_struct_from_entity(entity, &name, is_union) {
            Ok((s, nested)) => {
                let kind = if is_union { "union" } else { "struct" };
                debug!(name = %s.name, fields = s.fields.len(), size = s.size, "extracted {kind} (supplemental)");
                for ns in nested {
                    seen.insert(ns.name.clone());
                    debug!(name = %ns.name, fields = ns.fields.len(), "  nested anonymous type");
                    structs.push(ns);
                }
                structs.push(s);
            }
            Err(e) => warn!(name = %name, err = %e, "skipping struct/union"),
        }
    }

    structs
}

/// Collect enums via sonar, then run a supplemental pass for EnumDecl
/// entities that sonar missed (e.g. enums with forward declarations that
/// poison sonar's `seen` set).
fn collect_enums(
    entities: &[Entity],
    in_scope: &impl Fn(&Entity) -> bool,
) -> (Vec<EnumDef>, Vec<ConstantDef>) {
    let mut enums = Vec::new();
    let mut anon_constants = Vec::new();
    let mut seen = HashSet::new();

    // Primary: sonar-discovered enums
    for decl in sonar::find_enums(entities.to_vec()) {
        if !in_scope(&decl.entity) {
            trace_out_of_scope(&decl.entity, "enum");
            continue;
        }
        // Detect anonymous enums (e.g. `enum { DT_UNKNOWN = 0, ... }`).
        // clang gives them names like "enum (unnamed at /usr/include/dirent.h:97:1)".
        // These are just collections of integer constants in C — emit their
        // variants as standalone ConstantDef entries instead of a named enum.
        if decl.entity.is_anonymous() || decl.name.contains("(unnamed") {
            match extract_enum(&decl) {
                Ok(en) => {
                    debug!(
                        name = %decl.name,
                        variants = en.variants.len(),
                        "anonymous enum → emitting variants as constants"
                    );
                    for variant in en.variants {
                        let value = if variant.signed_value < 0 {
                            ConstantValue::Signed(variant.signed_value)
                        } else {
                            ConstantValue::Unsigned(variant.unsigned_value)
                        };
                        anon_constants.push(ConstantDef {
                            name: variant.name,
                            value,
                        });
                    }
                }
                Err(e) => warn!(name = %decl.name, err = %e, "skipping anonymous enum"),
            }
            continue;
        }
        seen.insert(decl.name.clone());
        match extract_enum(&decl) {
            Ok(en) => {
                debug!(name = %en.name, variants = en.variants.len(), "extracted enum");
                enums.push(en);
            }
            Err(e) => warn!(name = %decl.name, err = %e, "skipping enum"),
        }
    }

    // Supplemental: EnumDecl entities with full definitions that sonar
    // missed. This catches enums whose forward declaration poisoned
    // sonar's `seen` set (see SonarForwardDeclSkipsEnum.md).
    for entity in entities {
        if entity.get_kind() != EntityKind::EnumDecl {
            continue;
        }
        if !in_scope(entity) || !entity.is_definition() {
            if !entity.is_definition() {
                // forward decl — silent, not interesting
            } else {
                trace_out_of_scope(entity, "enum");
            }
            continue;
        }
        let name = match entity.get_name() {
            Some(n) if !n.is_empty() && !n.contains("(unnamed") => n,
            _ => continue,
        };
        if seen.contains(&name) {
            continue;
        }
        seen.insert(name.clone());
        match extract_enum_from_entity(entity, &name) {
            Ok(en) => {
                debug!(name = %en.name, variants = en.variants.len(), "extracted enum (supplemental)");
                enums.push(en);
            }
            Err(e) => warn!(name = %name, err = %e, "skipping enum"),
        }
    }

    (enums, anon_constants)
}

/// Collect functions via sonar.
fn collect_functions(entities: &[Entity], in_scope: &impl Fn(&Entity) -> bool) -> Vec<FunctionDef> {
    let mut functions = Vec::new();
    let mut seen = HashSet::new();
    for decl in sonar::find_functions(entities.to_vec()) {
        if !in_scope(&decl.entity) {
            trace_out_of_scope(&decl.entity, "function");
            continue;
        }
        // Skip variadic functions — P/Invoke metadata cannot represent `...`
        if decl.entity.is_variadic() {
            warn!(name = %decl.name, "skipping variadic function");
            continue;
        }
        match extract_function(&decl) {
            Ok(f) => {
                // Deduplicate by name — glibc __REDIRECT macros can produce
                // multiple declarations of the same function (e.g. lockf / lockf64).
                if !seen.insert(f.name.clone()) {
                    trace!(name = %f.name, "skipping duplicate function");
                    continue;
                }
                debug!(name = %f.name, params = f.params.len(), "extracted function");
                functions.push(f);
            }
            Err(e) => warn!(name = %decl.name, err = %e, "skipping function"),
        }
    }
    functions
}

/// Collect typedefs via custom discovery (not sonar, which drops typedef-to-
/// typedef aliases like `typedef Byte Bytef`).
fn collect_typedefs(entities: &[Entity], in_scope: &impl Fn(&Entity) -> bool) -> Vec<TypedefDef> {
    let mut typedefs = Vec::new();
    let mut seen = HashSet::new();
    for entity in entities {
        if entity.get_kind() != EntityKind::TypedefDecl {
            continue;
        }
        if !in_scope(entity) {
            trace_out_of_scope(entity, "typedef");
            continue;
        }
        let name = match entity.get_name() {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };
        if !seen.insert(name.clone()) {
            continue;
        }
        let underlying = match entity.get_typedef_underlying_type() {
            Some(ut) => ut,
            None => continue,
        };
        // Skip trivial struct/enum/union pass-throughs like `typedef struct foo foo;`
        if is_struct_passthrough(&underlying, &name) {
            trace!(name = %name, "skipping struct/enum passthrough typedef");
            continue;
        }
        // Skip typedefs whose name collides with a Rust primitive (e.g.
        // `typedef _Bool bool;` from linux/types.h would produce the
        // recursive `pub type bool = bool;`).
        if is_primitive_name(&name) {
            trace!(name = %name, "skipping typedef that shadows a Rust primitive");
            continue;
        }
        match extract_typedef_from_entity(entity, &name) {
            Ok(td) => {
                debug!(name = %td.name, "extracted typedef");
                typedefs.push(td);
            }
            Err(e) => warn!(name = %name, err = %e, "skipping typedef"),
        }
    }
    typedefs
}

/// Collect `#define` constants via sonar + supplemental hex parsing.
fn collect_constants(entities: &[Entity], in_scope: &impl Fn(&Entity) -> bool) -> Vec<ConstantDef> {
    let mut constants = Vec::new();
    let mut seen = HashSet::new();

    // Primary: sonar-discovered constants (decimal integers + floats)
    for def in sonar::find_definitions(entities.to_vec()) {
        if !in_scope(&def.entity) {
            continue;
        }
        let value = match def.value {
            DefinitionValue::Integer(negated, val) => {
                if negated {
                    ConstantValue::Signed(-(val as i64))
                } else if val <= i64::MAX as u64 {
                    ConstantValue::Signed(val as i64)
                } else {
                    ConstantValue::Unsigned(val)
                }
            }
            DefinitionValue::Real(val) => ConstantValue::Float(val),
        };
        debug!(name = %def.name, "extracted #define constant");
        seen.insert(def.name.clone());
        constants.push(ConstantDef {
            name: def.name,
            value,
        });
    }

    // Supplemental: hex constants that sonar's u64::from_str misses.
    // sonar only parses decimal; `#define PROT_READ 0x1` is silently skipped.
    for entity in entities {
        if entity.get_kind() != EntityKind::MacroDefinition {
            continue;
        }
        if !in_scope(entity) {
            continue;
        }
        let name = match entity.get_name() {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };
        if seen.contains(&name) {
            continue;
        }
        if let Some(range) = entity.get_range() {
            let mut tokens: Vec<String> =
                range.tokenize().iter().map(|t| t.get_spelling()).collect();
            // Strip trailing "#" that clang sometimes appends
            if tokens.last().is_some_and(|t| t == "#") {
                tokens.pop();
            }
            let (negated, number) = if tokens.len() == 2 {
                (false, &tokens[1])
            } else if tokens.len() == 3 && tokens[1] == "-" {
                (true, &tokens[2])
            } else {
                continue;
            };
            if let Some(val) = parse_hex_or_suffixed_int(number) {
                let value = if negated {
                    ConstantValue::Signed(-(val as i64))
                } else if val <= i64::MAX as u64 {
                    ConstantValue::Signed(val as i64)
                } else {
                    ConstantValue::Unsigned(val)
                };
                debug!(name = %name, "extracted #define hex constant");
                seen.insert(name.clone());
                constants.push(ConstantDef { name, value });
            }
        }
    }

    constants
}

/// Parse a hex literal (`0x1F`) or a suffixed integer (`1U`, `0x10UL`, etc.)
/// that `u64::from_str` can't handle. Returns None if not parseable.
fn parse_hex_or_suffixed_int(s: &str) -> Option<u64> {
    // Strip trailing integer suffixes: U, L, LL, UL, ULL (case-insensitive)
    let s = s.trim_end_matches(['u', 'U', 'l', 'L']);

    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else if let Some(octal) = s.strip_prefix("0") {
        if octal.is_empty() {
            Some(0) // "0" with suffixes stripped
        } else if octal.chars().all(|c| c.is_ascii_digit()) {
            u64::from_str_radix(octal, 8).ok()
        } else {
            None
        }
    } else {
        // Try decimal (handles cases where suffix stripping exposed a plain decimal)
        s.parse::<u64>().ok()
    }
}

// ---------------------------------------------------------------------------
// Struct extraction
// ---------------------------------------------------------------------------

fn extract_struct(decl: &Declaration) -> Result<(StructDef, Vec<StructDef>)> {
    extract_struct_from_entity(&decl.entity, &decl.name, false)
}

fn extract_struct_from_entity(
    entity: &Entity,
    name: &str,
    is_union: bool,
) -> Result<(StructDef, Vec<StructDef>)> {
    let ty = entity.get_type().context("struct has no type")?;
    let size = ty.get_sizeof().unwrap_or(0);
    let align = ty.get_alignof().unwrap_or(0);

    let mut fields = Vec::new();
    let mut nested_types = Vec::new();
    let mut anon_counter = 0u32;
    let children: Vec<_> = entity.get_children();

    // Collect entity IDs of anonymous record decls that have an explicit
    // named FieldDecl (e.g. `union { ... } addr;`). These are handled by
    // the existing try_extract_anonymous_field path on the FieldDecl, so
    // we must NOT also extract them via the C11 anonymous member path.
    let named_anon_decls: HashSet<_> = children
        .iter()
        .filter(|c| c.get_kind() == EntityKind::FieldDecl && c.get_name().is_some())
        .filter_map(|c| {
            let ft = c.get_type()?.get_canonical_type();
            if ft.get_kind() != TypeKind::Record {
                return None;
            }
            let decl = ft.get_declaration()?;
            if decl.is_anonymous() {
                Some(decl.get_usr())
            } else {
                None
            }
        })
        .flatten()
        .collect();

    for child in &children {
        match child.get_kind() {
            EntityKind::FieldDecl => {}
            // C11 anonymous struct/union member (no field name, no tag).
            // These appear as bare UnionDecl/StructDecl children rather
            // than FieldDecl children with an anonymous record type.
            EntityKind::UnionDecl | EntityKind::StructDecl
                if child.is_anonymous()
                    && !child
                        .get_usr()
                        .is_none_or(|usr| named_anon_decls.contains(&usr)) =>
            {
                let is_nested_union = child.get_kind() == EntityKind::UnionDecl;
                let synthetic_name = format!("{name}__anon_{anon_counter}");
                anon_counter += 1;
                match extract_struct_from_entity(child, &synthetic_name, is_nested_union) {
                    Ok((nested, mut more)) => {
                        let kind = if is_nested_union { "union" } else { "struct" };
                        debug!(
                            parent = %name,
                            synthetic = %synthetic_name,
                            "extracted C11 anonymous {kind} member"
                        );
                        let ctype = CType::Named {
                            name: synthetic_name.clone(),
                            resolved: None,
                        };
                        fields.push(FieldDef {
                            name: synthetic_name,
                            ty: ctype,
                            bitfield_width: None,
                            bitfield_offset: None,
                        });
                        nested_types.push(nested);
                        nested_types.append(&mut more);
                    }
                    Err(e) => {
                        warn!(
                            parent = %name,
                            err = %e,
                            "failed to extract C11 anonymous member"
                        );
                    }
                }
                continue;
            }
            _ => {
                continue;
            }
        }

        let field_name = child.get_name().unwrap_or_default();
        let field_type = child.get_type().context("field has no type")?;

        // Check for anonymous record type (unnamed struct/union used as a field type).
        // Clang gives these names like "union (unnamed at file.h:37:5)" which can't
        // be resolved. We extract them as separate TypeDefs with synthetic names.
        let ctype =
            match try_extract_anonymous_field(&field_type, name, &field_name, &mut nested_types) {
                Some(synthetic_name) => CType::Named {
                    name: synthetic_name,
                    resolved: None,
                },
                None => map_clang_type(&field_type)
                    .with_context(|| format!("unsupported type for field '{}'", field_name))?,
            };

        let bitfield_width = if child.is_bit_field() {
            child.get_bit_field_width()
        } else {
            None
        };
        let bitfield_offset = if child.is_bit_field() {
            child.get_offset_of_field().ok()
        } else {
            None
        };

        trace!(field = %field_name, ty = ?ctype, bitfield_width, bitfield_offset, "  field");
        fields.push(FieldDef {
            name: field_name,
            ty: ctype,
            bitfield_width,
            bitfield_offset,
        });
    }

    // Flatten bitfield fields: replace each bitfield group with a single
    // integer field sized to cover the group's total bit span. Adjacent
    // bitfields that pack into the same storage unit (determined by
    // bitfield_offset continuity) are merged into one field.
    if !is_union {
        fields = flatten_bitfields(fields, name);
    }

    // Append trailing padding if clang's sizeof exceeds what repr(C)
    // layout would produce from the fields alone. This handles
    // `__attribute__((aligned(N)))` which rounds the struct size up
    // beyond the natural field-based padding that windows-bindgen
    // computes from packed(N).
    //
    // windows-bindgen computes size from fields, rounding up to the
    // maximum field alignment (not the struct's declared alignment).
    // We approximate this by rounding last_field_end up to the largest
    // individual field's natural alignment.
    if size > 0 && !fields.is_empty() && !is_union {
        let mut last_field_end: usize = 0;
        let mut max_field_align: usize = 1;
        for child in &children {
            if child.get_kind() != EntityKind::FieldDecl {
                continue;
            }
            if let Ok(offset_bits) = child.get_offset_of_field() {
                let offset_bytes = offset_bits / 8;
                let field_ty = child.get_type();
                let field_size = field_ty
                    .as_ref()
                    .and_then(|t| t.get_sizeof().ok())
                    .unwrap_or(0);
                let field_align = field_ty
                    .as_ref()
                    .and_then(|t| t.get_alignof().ok())
                    .unwrap_or(1);
                let end = offset_bytes + field_size;
                if end > last_field_end {
                    last_field_end = end;
                }
                if field_align > max_field_align {
                    max_field_align = field_align;
                }
            }
        }
        if last_field_end > 0 && max_field_align > 0 {
            let natural_size = (last_field_end + max_field_align - 1) & !(max_field_align - 1);
            if size > natural_size {
                let trailing_pad = size - last_field_end;
                debug!(
                    name = %name,
                    struct_size = size,
                    natural_size,
                    last_field_end,
                    trailing_pad,
                    "appending trailing padding for alignment attribute"
                );
                fields.push(FieldDef {
                    name: "_padding".to_string(),
                    ty: CType::Array {
                        element: Box::new(CType::U8),
                        len: trailing_pad,
                    },
                    bitfield_width: None,
                    bitfield_offset: None,
                });
            }
        }
    }

    Ok((
        StructDef {
            name: name.to_string(),
            size,
            align,
            fields,
            is_union,
        },
        nested_types,
    ))
}

/// Flatten bitfield fields into correctly-sized integer fields.
///
/// Adjacent bitfields are grouped by checking whether each field's
/// `bitfield_offset` is contiguous with the previous one (offset ==
/// prev_offset + prev_width). Each group is replaced by a single
/// integer field sized to cover the group's total bit span.
///
/// Non-bitfield fields pass through unchanged.
fn flatten_bitfields(fields: Vec<FieldDef>, struct_name: &str) -> Vec<FieldDef> {
    if !fields.iter().any(|f| f.bitfield_width.is_some()) {
        return fields;
    }

    let mut result: Vec<FieldDef> = Vec::new();
    // Accumulator for the current group of adjacent bitfields.
    let mut group: Vec<&FieldDef> = Vec::new();
    let mut group_index = 0u32;

    let flush_group = |group: &mut Vec<&FieldDef>,
                       result: &mut Vec<FieldDef>,
                       group_index: &mut u32,
                       struct_name: &str| {
        if group.is_empty() {
            return;
        }
        let first = group[0];
        let group_start = first.bitfield_offset.unwrap_or(0);
        let last = group[group.len() - 1];
        let group_end = last.bitfield_offset.unwrap_or(0) + last.bitfield_width.unwrap_or(0);
        let total_bits = group_end - group_start;

        let (name, ty) = if group.len() == 1 {
            // Solo bitfield: keep original name, replace type.
            (first.name.clone(), smallest_int_for_bits(total_bits))
        } else {
            // Merged group: synthetic name, covering type.
            let names: Vec<&str> = group.iter().map(|f| f.name.as_str()).collect();
            debug!(
                struct_name = %struct_name,
                fields = ?names,
                total_bits,
                "merged adjacent bitfield group"
            );
            (
                format!("_bitfield_{}", *group_index),
                smallest_int_for_bits(total_bits),
            )
        };
        *group_index += 1;

        result.push(FieldDef {
            name,
            ty,
            bitfield_width: None,
            bitfield_offset: None,
        });
        group.clear();
    };

    for field in &fields {
        if let (Some(offset), Some(width)) = (field.bitfield_offset, field.bitfield_width) {
            // Check if this field is contiguous with the current group.
            if let Some(last) = group.last() {
                let prev_end = last.bitfield_offset.unwrap_or(0) + last.bitfield_width.unwrap_or(0);
                if offset != prev_end {
                    // Gap — flush the current group and start a new one.
                    flush_group(&mut group, &mut result, &mut group_index, struct_name);
                }
            }
            let _ = (offset, width); // used above via field
            group.push(field);
        } else {
            // Non-bitfield: flush any pending group, then pass through.
            flush_group(&mut group, &mut result, &mut group_index, struct_name);
            result.push(FieldDef {
                name: field.name.clone(),
                ty: field.ty.clone(),
                bitfield_width: None,
                bitfield_offset: None,
            });
        }
    }
    // Flush any trailing group.
    flush_group(&mut group, &mut result, &mut group_index, struct_name);
    result
}

/// Return the smallest unsigned integer CType that can hold `bits` bits.
fn smallest_int_for_bits(bits: usize) -> CType {
    match bits {
        0..=8 => CType::U8,
        9..=16 => CType::U16,
        17..=32 => CType::U32,
        _ => CType::U64,
    }
}

/// Try to extract an anonymous record field type as a synthetic named type.
///
/// When a struct/union contains a field whose type is an anonymous record
/// (e.g. `union { int a; float b; } field;`), clang gives it a non-portable
/// name like `"union (unnamed at file.h:37:5)"`. This function detects that
/// case, recursively extracts the anonymous record as a separate `StructDef`
/// with a synthetic name `ParentName_FieldName`, and returns the synthetic
/// name so the caller can reference it.
fn try_extract_anonymous_field(
    field_type: &ClangType,
    parent_name: &str,
    field_name: &str,
    nested_types: &mut Vec<StructDef>,
) -> Option<String> {
    let canonical = field_type.get_canonical_type();
    if canonical.get_kind() != TypeKind::Record {
        return None;
    }
    let decl = canonical.get_declaration()?;
    if !decl.is_anonymous() {
        return None;
    }
    let is_nested_union = decl.get_kind() == EntityKind::UnionDecl;
    let synthetic_name = format!("{}_{}", parent_name, field_name);

    match extract_struct_from_entity(&decl, &synthetic_name, is_nested_union) {
        Ok((nested, mut more)) => {
            let kind = if is_nested_union { "union" } else { "struct" };
            debug!(
                parent = %parent_name,
                field = %field_name,
                synthetic = %synthetic_name,
                "extracted anonymous {kind} as synthetic type"
            );
            nested_types.push(nested);
            nested_types.append(&mut more); // handle deeply nested anonymous types
            Some(synthetic_name)
        }
        Err(e) => {
            warn!(
                parent = %parent_name,
                field = %field_name,
                err = %e,
                "failed to extract anonymous nested type"
            );
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Enum extraction
// ---------------------------------------------------------------------------

fn extract_enum(decl: &Declaration) -> Result<EnumDef> {
    extract_enum_from_entity(&decl.entity, &decl.name)
}

/// Extract an enum directly from a clang Entity (used by the supplemental pass).
fn extract_enum_from_entity(entity: &Entity, name: &str) -> Result<EnumDef> {
    let underlying = entity
        .get_enum_underlying_type()
        .context("enum has no underlying type")?;
    let underlying_ctype = map_clang_type(&underlying).unwrap_or(CType::I32);

    let mut variants = Vec::new();
    for child in entity.get_children() {
        if child.get_kind() != EntityKind::EnumConstantDecl {
            continue;
        }
        let vname = child.get_name().unwrap_or_default();
        let (signed, unsigned) = child.get_enum_constant_value().unwrap_or((0, 0));
        variants.push(EnumVariant {
            name: vname,
            signed_value: signed,
            unsigned_value: unsigned,
        });
    }

    Ok(EnumDef {
        name: name.to_string(),
        underlying_type: underlying_ctype,
        variants,
    })
}

// ---------------------------------------------------------------------------
// Function extraction
// ---------------------------------------------------------------------------

fn extract_function(decl: &Declaration) -> Result<FunctionDef> {
    let fn_type = decl.entity.get_type().context("function has no type")?;

    let ret_type = fn_type
        .get_result_type()
        .context("function has no return type")?;
    let return_ctype = map_clang_type(&ret_type).unwrap_or(CType::Void);

    let calling_convention = fn_type
        .get_calling_convention()
        .map(map_calling_convention)
        .unwrap_or(CallConv::Cdecl);

    let args = decl.entity.get_arguments().unwrap_or_default();
    let arg_types = fn_type.get_argument_types().unwrap_or_default();

    let mut params = Vec::new();
    for (i, arg_entity) in args.iter().enumerate() {
        let name = arg_entity
            .get_name()
            .unwrap_or_else(|| format!("param{}", i));
        let ty = if i < arg_types.len() {
            map_clang_type(&arg_types[i]).unwrap_or(CType::Void)
        } else {
            CType::Void
        };
        // C array parameters decay to pointers (e.g. `const struct timespec t[2]` → `*timespec`).
        // We must do this here because ELEMENT_TYPE_ARRAY blobs in method signatures can confuse
        // windows-bindgen's reader which doesn't consume all ArrayShape fields.
        let ty = match ty {
            CType::Array { element, .. } => CType::Ptr {
                pointee: element,
                is_const: false,
            },
            other => other,
        };
        params.push(ParamDef { name, ty });
    }

    Ok(FunctionDef {
        name: decl.name.clone(),
        return_type: return_ctype,
        params,
        calling_convention,
    })
}

// ---------------------------------------------------------------------------
// Typedef extraction
// ---------------------------------------------------------------------------

fn extract_typedef_from_entity(entity: &Entity, name: &str) -> Result<TypedefDef> {
    let underlying = entity
        .get_typedef_underlying_type()
        .context("typedef has no underlying type")?;
    let ctype = map_clang_type(&underlying)?;
    trace!(name = %name, ty = ?ctype, "typedef underlying type");

    Ok(TypedefDef {
        name: name.to_string(),
        underlying_type: ctype,
    })
}

// ---------------------------------------------------------------------------
// Type mapping: clang TypeKind → CType
// ---------------------------------------------------------------------------

fn map_clang_type(ty: &ClangType) -> Result<CType> {
    match ty.get_kind() {
        TypeKind::Void => Ok(CType::Void),
        TypeKind::Bool => Ok(CType::Bool),
        TypeKind::CharS | TypeKind::SChar => Ok(CType::I8),
        TypeKind::CharU | TypeKind::UChar => Ok(CType::U8),
        TypeKind::Short => Ok(CType::I16),
        TypeKind::UShort => Ok(CType::U16),
        TypeKind::Int => Ok(CType::I32),
        TypeKind::UInt => Ok(CType::U32),
        // C `long` is 64-bit on Linux x86-64 (LP64 ABI)
        TypeKind::Long => Ok(CType::I64),
        TypeKind::ULong => Ok(CType::U64),
        TypeKind::LongLong => Ok(CType::I64),
        TypeKind::ULongLong => Ok(CType::U64),
        TypeKind::Float => Ok(CType::F32),
        TypeKind::Double => Ok(CType::F64),

        // __int128 / unsigned __int128: no WinMD ELEMENT_TYPE for 128-bit
        // integers and windows-bindgen cannot emit i128/u128. Bail so the
        // caller can skip the containing type with a warning.
        TypeKind::Int128 => {
            anyhow::bail!("__int128 not supported (no WinMD 128-bit integer type)")
        }
        TypeKind::UInt128 => {
            anyhow::bail!("unsigned __int128 not supported (no WinMD 128-bit integer type)")
        }

        TypeKind::Pointer => {
            let pointee = ty
                .get_pointee_type()
                .context("pointer has no pointee type")?;
            let is_const = pointee.is_const_qualified();
            let inner = map_clang_type(&pointee)?;
            Ok(CType::Ptr {
                pointee: Box::new(inner),
                is_const,
            })
        }

        TypeKind::ConstantArray => {
            let elem = ty.get_element_type().context("array has no element type")?;
            let len = ty.get_size().unwrap_or(0);
            let inner = map_clang_type(&elem)?;
            Ok(CType::Array {
                element: Box::new(inner),
                len,
            })
        }

        TypeKind::IncompleteArray => {
            // Treat as pointer
            let elem = ty
                .get_element_type()
                .context("incomplete array has no element type")?;
            let inner = map_clang_type(&elem)?;
            Ok(CType::Ptr {
                pointee: Box::new(inner),
                is_const: false,
            })
        }

        TypeKind::Elaborated => {
            let inner = ty
                .get_elaborated_type()
                .context("elaborated type has no inner type")?;
            map_clang_type(&inner)
        }

        TypeKind::Typedef => {
            let decl = ty.get_declaration();
            if let Some(decl) = decl {
                let name = decl.get_name().unwrap_or_default();
                if !name.is_empty() {
                    // va_list is a compiler built-in with no portable canonical type
                    if matches!(
                        name.as_str(),
                        "va_list" | "__builtin_va_list" | "__gnuc_va_list"
                    ) {
                        return Ok(CType::Ptr {
                            pointee: Box::new(CType::Void),
                            is_const: false,
                        });
                    }
                    // Resolve the canonical type — if it's unsupported (e.g.
                    // __int128), bail so any typedef chain referencing it is
                    // also skipped (e.g. `typedef __s128 s128`).
                    let canonical = ty.get_canonical_type();
                    let resolved = map_clang_type(&canonical).map(Box::new)?;
                    return Ok(CType::Named {
                        name,
                        resolved: Some(resolved),
                    });
                }
            }
            // Unnamed or unresolvable typedef — resolve to canonical primitive
            let canonical = ty.get_canonical_type();
            map_clang_type(&canonical)
        }

        TypeKind::Record => {
            let decl = ty.get_declaration();
            if let Some(decl) = decl
                && let Some(name) = decl.get_name()
            {
                // __va_list_tag is a compiler built-in struct backing va_list on
                // x86-64.  It has no header file location and must not leak into
                // the winmd.  Map it to Void so pointers become `*mut c_void`.
                if name == "__va_list_tag" {
                    return Ok(CType::Void);
                }

                // Check if the type is complete (has a definition, not just forward-declared).
                // Incomplete/opaque types (like `struct internal_state` in zlib) are
                // mapped to Void so that pointers to them become `*mut c_void`.
                if ty.get_sizeof().is_ok() {
                    return Ok(CType::Named {
                        name,
                        resolved: None,
                    });
                } else {
                    debug!(name = %name, "incomplete record type, mapping to Void");
                    return Ok(CType::Void);
                }
            }
            anyhow::bail!("anonymous record type without name")
        }

        TypeKind::Enum => {
            let decl = ty.get_declaration();
            if let Some(decl) = decl
                && let Some(name) = decl.get_name()
            {
                return Ok(CType::Named {
                    name,
                    resolved: None,
                });
            }
            anyhow::bail!("anonymous enum type without name")
        }

        TypeKind::FunctionPrototype => {
            let ret = ty
                .get_result_type()
                .context("function prototype has no return type")?;
            let ret_ctype = map_clang_type(&ret)?;
            let arg_types = ty.get_argument_types().unwrap_or_default();
            let mut params = Vec::new();
            for at in &arg_types {
                params.push(map_clang_type(at)?);
            }
            let cc = ty
                .get_calling_convention()
                .map(map_calling_convention)
                .unwrap_or(CallConv::Cdecl);
            Ok(CType::FnPtr {
                return_type: Box::new(ret_ctype),
                params,
                calling_convention: cc,
            })
        }

        TypeKind::FunctionNoPrototype => {
            // K&R-style function — treat as void() for now
            Ok(CType::FnPtr {
                return_type: Box::new(CType::Void),
                params: vec![],
                calling_convention: CallConv::Cdecl,
            })
        }

        other => {
            anyhow::bail!("unsupported clang TypeKind: {:?}", other)
        }
    }
}

// ---------------------------------------------------------------------------
// Calling convention mapping
// ---------------------------------------------------------------------------

fn map_calling_convention(cc: CallingConvention) -> CallConv {
    match cc {
        CallingConvention::Cdecl => CallConv::Cdecl,
        CallingConvention::Stdcall => CallConv::Stdcall,
        CallingConvention::Fastcall => CallConv::Fastcall,
        // Everything else → Cdecl (platform default)
        _ => CallConv::Cdecl,
    }
}

// ---------------------------------------------------------------------------
// Typedef filtering helpers
// ---------------------------------------------------------------------------

/// Returns true if this typedef is a trivial struct/enum/union pass-through,
/// i.e. `typedef struct foo foo;` or `typedef enum bar bar;`.
/// These are handled by sonar's find_structs/find_enums and should NOT also
/// appear as typedefs.
fn is_struct_passthrough(underlying: &ClangType, typedef_name: &str) -> bool {
    let display = underlying.get_display_name();
    for prefix in &["struct ", "enum ", "union "] {
        if display.starts_with(prefix) && &display[prefix.len()..] == typedef_name {
            return true;
        }
    }
    false
}

/// Returns `true` if `name` is a Rust primitive type name.  Typedefs with
/// these names (e.g. `typedef _Bool bool;`) would produce a recursive type
/// alias like `pub type bool = bool;`.
fn is_primitive_name(name: &str) -> bool {
    matches!(
        name,
        "bool"
            | "i8"
            | "u8"
            | "i16"
            | "u16"
            | "i32"
            | "u32"
            | "i64"
            | "u64"
            | "f32"
            | "f64"
            | "isize"
            | "usize"
    )
}

// ---------------------------------------------------------------------------
// Source-location filtering (partition traversal)
// ---------------------------------------------------------------------------

fn should_emit(entity: &Entity, traverse_files: &[PathBuf], base_dir: &Path) -> bool {
    should_emit_by_location(entity, traverse_files, base_dir)
}

/// Emit a trace log when an entity is skipped because it falls outside the
/// traverse scope. Helps diagnose missing types when authoring partitions.
fn trace_out_of_scope(entity: &Entity, kind: &str) {
    let file = entity
        .get_location()
        .and_then(|loc| loc.get_file_location().file)
        .map(|f| f.get_path().display().to_string())
        .unwrap_or_else(|| "<unknown>".into());
    let name = entity.get_name().unwrap_or_else(|| "<unnamed>".into());
    trace!(kind, name = %name, file = %file, "skipping out-of-scope type");
}

fn should_emit_by_location(entity: &Entity, traverse_files: &[PathBuf], _base_dir: &Path) -> bool {
    let location = match entity.get_location() {
        Some(loc) => loc,
        None => return false,
    };
    let file_location = location.get_file_location();
    let file = match file_location.file {
        Some(f) => f,
        None => return false,
    };
    let file_path = file.get_path();

    // traverse_files are already resolved to absolute paths by the caller,
    // so we just compare directly (or by suffix for robustness).
    traverse_files
        .iter()
        .any(|tf| file_path == *tf || file_path.ends_with(tf))
}

/// Build a type registry from all partitions' extracted data.
///
/// Typedefs use first-writer-wins: the first partition to register a typedef
/// name owns it. This means a dedicated "types" partition should come first
/// in the TOML so it claims shared types like `uid_t`, `pid_t`, etc. before
/// other partitions can. Structs and enums still use last-writer-wins (they
/// rarely overlap across partitions).
pub fn build_type_registry(
    partitions: &[Partition],
    namespace_overrides: &std::collections::HashMap<String, String>,
) -> TypeRegistry {
    let mut registry = TypeRegistry::default();
    for partition in partitions {
        for s in &partition.structs {
            let ns = namespace_overrides
                .get(&s.name)
                .unwrap_or(&partition.namespace);
            registry.register(&s.name, ns);
        }
        for e in &partition.enums {
            let ns = namespace_overrides
                .get(&e.name)
                .unwrap_or(&partition.namespace);
            registry.register(&e.name, ns);
        }
        for td in &partition.typedefs {
            // First-writer-wins for typedefs: if already registered by an
            // earlier partition (e.g. a shared types partition), skip.
            if registry.contains(&td.name) {
                continue;
            }
            let ns = namespace_overrides
                .get(&td.name)
                .unwrap_or(&partition.namespace);
            registry.register(&td.name, ns);
        }
    }
    registry
}
