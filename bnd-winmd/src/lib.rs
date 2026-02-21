//! bnd-winmd — C header → WinMD metadata generator.
//!
//! Parses C headers via libclang and emits ECMA-335 `.winmd` files using the
//! `windows-metadata` writer crate.
//!
//! # Quick start
//!
//! Generate a `.winmd` file from a config (suitable for `build.rs`):
//!
//! ```no_run
//! use std::path::Path;
//!
//! // Reads config TOML, parses headers, writes the .winmd file.
//! bnd_winmd::run(Path::new("bnd-winmd.toml"), None).unwrap();
//! ```
//!
//! Or get the raw bytes without writing to disk:
//!
//! ```no_run
//! use std::path::Path;
//!
//! let winmd_bytes = bnd_winmd::generate(Path::new("bnd-winmd.toml")).unwrap();
//! ```

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tracing::{info, warn};

pub mod config;
pub mod emit;
pub mod extract;
pub mod model;

/// Run the full pipeline: load config, parse C headers, emit WinMD, and write
/// the output file.
///
/// `config_path` is the path to a `bnd-winmd.toml` configuration file.  
/// `output` optionally overrides the output file path from the config.
///
/// This is the top-level entry point intended for use in `build.rs` scripts
/// or other programmatic callers that want the complete generate-and-write
/// workflow in a single call.
///
/// Returns the path the `.winmd` file was written to.
pub fn run(config_path: &Path, output: Option<&Path>) -> Result<PathBuf> {
    let cfg = config::load_config(config_path)
        .with_context(|| format!("loading config from {}", config_path.display()))?;

    let base_dir = config_path.parent().unwrap_or_else(|| Path::new("."));

    let winmd_bytes = generate_from_config(&cfg, base_dir)?;

    let output_path = match output {
        Some(p) => p.to_path_buf(),
        None => base_dir.join(&cfg.output.file),
    };
    std::fs::write(&output_path, &winmd_bytes)
        .with_context(|| format!("writing output to {}", output_path.display()))?;

    info!(
        path = %output_path.display(),
        size = winmd_bytes.len(),
        "wrote winmd"
    );

    Ok(output_path)
}

/// Parse a `bnd-winmd.toml` config file, extract declarations from the
/// referenced C headers, and return the generated WinMD bytes without
/// writing to disk.
pub fn generate(config_path: &Path) -> Result<Vec<u8>> {
    let cfg = config::load_config(config_path)
        .with_context(|| format!("loading config from {}", config_path.display()))?;

    let base_dir = config_path.parent().unwrap_or_else(|| Path::new("."));

    generate_from_config(&cfg, base_dir)
}

/// Generate WinMD bytes from an already-loaded [`config::Config`].
///
/// `base_dir` is the directory relative to which header paths in the config
/// are resolved (typically the parent directory of the TOML file).
pub fn generate_from_config(cfg: &config::Config, base_dir: &Path) -> Result<Vec<u8>> {
    info!(
        assembly = %cfg.output.name,
        partitions = cfg.partition.len(),
        "loaded configuration"
    );

    // Initialize clang
    let clang =
        clang::Clang::new().map_err(|e| anyhow::anyhow!("failed to initialize libclang: {e}"))?;
    let index = clang::Index::new(&clang, false, false);

    // Extract all partitions
    let mut partitions = Vec::new();
    for partition_cfg in &cfg.partition {
        let partition = extract::extract_partition(
            &index,
            partition_cfg,
            base_dir,
            &cfg.include_paths,
            &cfg.namespace_overrides,
        )?;
        partitions.push(partition);
    }

    // Build global type registry
    let mut registry = extract::build_type_registry(&partitions, &cfg.namespace_overrides);

    // Pre-seed the registry with types from external winmd files
    // (cross-winmd references). This must happen after build_type_registry
    // so that locally-extracted types take priority (first-writer-wins in
    // the registry), but imported types fill in names that are referenced
    // by function signatures but not extracted locally.
    for ti in &cfg.type_import {
        let winmd_path = config::resolve_header(&ti.winmd, base_dir, &cfg.include_paths);
        seed_registry_from_winmd(&mut registry, &winmd_path, &ti.namespace);
    }

    // Deduplicate typedefs and structs: when the same type appears in
    // multiple partitions (e.g. `uid_t` or `__sigset_t` in signal, pthread,
    // stat, etc.), keep it only in the partition the registry maps it to.
    // The registry uses first-writer-wins, so the partition listed first in
    // the TOML claims shared names. Other partitions drop their local copy;
    // any function/struct that references the type will use a cross-partition
    // TypeRef instead.
    for partition in &mut partitions {
        partition.typedefs.retain(|td| {
            let canonical_ns = registry.namespace_for(&td.name, &partition.namespace);
            let dominated = canonical_ns != partition.namespace;
            if dominated {
                warn!(
                    name = td.name,
                    canonical = canonical_ns,
                    duplicate = partition.namespace,
                    "dropping duplicate typedef (canonical partition wins)"
                );
            }
            !dominated
        });
        partition.structs.retain(|sd| {
            let canonical_ns = registry.namespace_for(&sd.name, &partition.namespace);
            let dominated = canonical_ns != partition.namespace;
            if dominated {
                warn!(
                    name = sd.name,
                    canonical = canonical_ns,
                    duplicate = partition.namespace,
                    "dropping duplicate struct (canonical partition wins)"
                );
            }
            !dominated
        });
    }

    // Validate that all referenced types are resolvable before emitting.
    // This catches missing traverse headers early with actionable diagnostics
    // instead of a cryptic windows-bindgen "type not found" panic later.
    validate_type_references(&partitions, &registry)?;

    // Emit winmd
    let winmd_bytes = emit::emit_winmd(&cfg.output.name, &partitions, &registry)?;

    info!(size = winmd_bytes.len(), "generated winmd");

    Ok(winmd_bytes)
}

/// Pre-seed the [`TypeRegistry`](model::TypeRegistry) with types from an
/// external `.winmd` file.  Only types whose namespace starts with
/// `ns_filter` are imported.
fn seed_registry_from_winmd(
    registry: &mut model::TypeRegistry,
    winmd_path: &Path,
    ns_filter: &str,
) {
    let bytes = std::fs::read(winmd_path).unwrap_or_else(|e| {
        panic!(
            "failed to read external winmd {}: {e}\n\
             Hint: run the upstream gen crate first (e.g. `cargo run -p bnd-posix-gen`)",
            winmd_path.display()
        )
    });
    let file = windows_metadata::reader::File::new(bytes)
        .unwrap_or_else(|| panic!("failed to parse external winmd: {}", winmd_path.display()));
    let index = windows_metadata::reader::TypeIndex::new(vec![file]);
    let mut count = 0usize;
    for td in index.types() {
        let ns = td.namespace();
        let name = td.name();
        // Skip the synthetic <Module> and Apis classes, and filter by namespace.
        if ns.is_empty() || name == "<Module>" || name == "Apis" {
            continue;
        }
        if !ns.starts_with(ns_filter) {
            continue;
        }
        // Only insert if not already registered (local types win).
        // When two external namespaces define the same type name (e.g.
        // __sigset_t in posix.signal and posix.pthread), keep the
        // lexicographically smallest namespace for determinism.
        if !registry.contains(name) {
            registry.register(name, ns);
            count += 1;
        } else if registry.namespace_for(name, "").as_str() < ns {
            // Already have a smaller namespace — keep it.
        } else {
            registry.register(name, ns);
        }
    }
    info!(
        path = %winmd_path.display(),
        namespace = ns_filter,
        imported = count,
        "pre-seeded type registry from external winmd"
    );
}

// ---------------------------------------------------------------------------
// Type-reference validation
// ---------------------------------------------------------------------------

/// A single unresolved type reference with context about where it was found.
struct UnresolvedRef {
    type_name: String,
    partition: String,
    context: String,
}

/// Walk all CType trees in every partition and verify that each
/// `Named { resolved: None }` type is present in the registry.
///
/// Types with `resolved: Some(_)` are fine — they fall back to the canonical
/// primitive at emit time. Only `resolved: None` (records, enums, anonymous
/// nested types) must be registered.
fn validate_type_references(
    partitions: &[model::Partition],
    registry: &model::TypeRegistry,
) -> Result<()> {
    let mut unresolved: Vec<UnresolvedRef> = Vec::new();

    for partition in partitions {
        let ns = &partition.namespace;

        for s in &partition.structs {
            for field in &s.fields {
                collect_unresolved(
                    &field.ty,
                    registry,
                    ns,
                    &format!("field `{}` of struct `{}`", field.name, s.name),
                    &mut unresolved,
                );
            }
        }

        for f in &partition.functions {
            collect_unresolved(
                &f.return_type,
                registry,
                ns,
                &format!("return type of function `{}`", f.name),
                &mut unresolved,
            );
            for param in &f.params {
                collect_unresolved(
                    &param.ty,
                    registry,
                    ns,
                    &format!("param `{}` of function `{}`", param.name, f.name),
                    &mut unresolved,
                );
            }
        }

        for td in &partition.typedefs {
            collect_unresolved(
                &td.underlying_type,
                registry,
                ns,
                &format!("typedef `{}`", td.name),
                &mut unresolved,
            );
        }
    }

    if unresolved.is_empty() {
        return Ok(());
    }

    // Deduplicate by type name for a concise summary, but keep the first
    // usage context for each name.
    let mut seen = std::collections::HashSet::new();
    let mut unique: Vec<&UnresolvedRef> = Vec::new();
    for r in &unresolved {
        if seen.insert(&r.type_name) {
            unique.push(r);
        }
    }

    let mut msg = format!(
        "{} unresolved type reference(s) found — these will cause \
         windows-bindgen to fail with \"type not found\".\n\
         Hint: add the header that defines each type to the partition's \
         `traverse` list, or add a `[[type_import]]` for an external winmd.\n",
        unique.len()
    );
    for r in &unique {
        msg.push_str(&format!(
            "\n  • `{}` — referenced in {} (partition `{}`)",
            r.type_name, r.context, r.partition,
        ));
    }

    anyhow::bail!("{msg}");
}

/// Recursively walk a CType and collect any `Named { resolved: None }` that
/// is not in the registry.
fn collect_unresolved(
    ctype: &model::CType,
    registry: &model::TypeRegistry,
    partition_ns: &str,
    context: &str,
    out: &mut Vec<UnresolvedRef>,
) {
    match ctype {
        model::CType::Named { name, resolved } => {
            if resolved.is_none() && !registry.contains(name) {
                out.push(UnresolvedRef {
                    type_name: name.clone(),
                    partition: partition_ns.to_string(),
                    context: context.to_string(),
                });
            }
        }
        model::CType::Ptr { pointee, .. } => {
            collect_unresolved(pointee, registry, partition_ns, context, out);
        }
        model::CType::Array { element, .. } => {
            collect_unresolved(element, registry, partition_ns, context, out);
        }
        model::CType::FnPtr {
            return_type,
            params,
            ..
        } => {
            collect_unresolved(return_type, registry, partition_ns, context, out);
            for p in params {
                collect_unresolved(p, registry, partition_ns, context, out);
            }
        }
        // Primitives, Void, etc. — nothing to check.
        _ => {}
    }
}
