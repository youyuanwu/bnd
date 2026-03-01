mod bindings;
pub use bindings::*;

extern crate bnd_macros as windows_link;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MAX_WIDGETS, 256);
        assert_eq!(DEFAULT_WIDTH, 800);
        assert_eq!(DEFAULT_HEIGHT, 600);
    }

    #[test]
    fn test_global_clang_args() {
        // MAX_DEPTH is defined via `#ifdef CUSTOM_DEPTH` in simple.h,
        // which is only active when `-DCUSTOM_DEPTH=42` is passed to clang.
        // This flag comes from the top-level `clang_args` in simple.toml.
        assert_eq!(MAX_DEPTH, 42);
    }

    #[test]
    fn test_enum_values() {
        assert_eq!(COLOR_RED, 0u32);
        assert_eq!(COLOR_GREEN, 1u32);
        assert_eq!(COLOR_BLUE, 2u32);
    }

    #[test]
    fn test_struct_layout() {
        // Rect should be 16 bytes (4 × i32/u32)
        assert_eq!(std::mem::size_of::<Rect>(), 16);
        assert_eq!(std::mem::align_of::<Rect>(), 4);

        let r = Rect {
            x: 10,
            y: 20,
            width: 100,
            height: 200,
        };
        assert_eq!(r.x, 10);
        assert_eq!(r.width, 100);
    }

    #[test]
    fn test_create_and_destroy_widget() {
        unsafe {
            assert_eq!(widget_count(), 0);

            let name = c"hello".as_ptr();
            let bounds = Rect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            };
            let mut widget: Widget = std::mem::zeroed();

            let result = create_widget(name, bounds, &mut widget as *mut Widget);
            assert_eq!(result, 0);
            assert_eq!(widget_count(), 1);
            assert_eq!(widget.color, COLOR_RED);
            assert_eq!(widget.values[2], 800); // width

            // Verify pointer round-trip
            let returned_name = std::ffi::CStr::from_ptr(widget.name);
            assert_eq!(returned_name.to_str().unwrap(), "hello");

            destroy_widget(&mut widget as *mut Widget);
            assert_eq!(widget_count(), 0);
        }
    }

    #[test]
    fn test_create_widget_null_returns_error() {
        unsafe {
            let bounds = Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            let result = create_widget(std::ptr::null(), bounds, std::ptr::null_mut());
            assert_eq!(result, -1);
        }
    }

    #[test]
    fn test_union_layout() {
        // Value union should be 4 bytes (max of i32, f32, [u8; 4])
        assert_eq!(std::mem::size_of::<Value>(), 4);
        assert_eq!(std::mem::align_of::<Value>(), 4);

        let mut v = Value { i: 0x41424344 };
        assert_eq!(unsafe { v.i }, 0x41424344);

        // Writing via one field and reading via another — classic union behavior
        v.f = 1.0_f32;
        assert_eq!(unsafe { v.f }, 1.0);
        // IEEE 754: 1.0f == 0x3F800000
        assert_eq!(unsafe { v.i }, 0x3F80_0000_i32);
        assert_eq!(unsafe { v.bytes }, 1.0_f32.to_ne_bytes());
    }

    #[test]
    fn test_anonymous_nested_union() {
        // NetAddr contains an anonymous union field 'addr' extracted as NetAddr_addr
        assert_eq!(std::mem::size_of::<NetAddr>(), 20);
        assert_eq!(std::mem::size_of::<NetAddr_addr>(), 16);

        let mut na = NetAddr::default();
        // Write via dwords and read back via bytes/words
        na.addr.dwords = [0x04030201, 0x08070605, 0x0C0B0A09, 0x100F0E0D];
        unsafe {
            assert_eq!(na.addr.bytes[0], 0x01);
            assert_eq!(na.addr.bytes[15], 0x10);
            assert_eq!(na.addr.words[0], 0x0201);
        }
        na.scope_id = 42;
        assert_eq!(na.scope_id, 42);
    }

    #[test]
    fn test_delegate_type_exists() {
        // Verify CompareFunc delegate compiles and has the right signature.
        unsafe extern "system" fn cmp(
            a: *const core::ffi::c_void,
            b: *const core::ffi::c_void,
        ) -> i32 {
            unsafe {
                let a = *(a as *const i32);
                let b = *(b as *const i32);
                a - b
            }
        }
        let _f: CompareFunc = Some(cmp);
    }

    #[test]
    fn test_bool_typedef_not_recursive() {
        // `typedef _Bool bool;` in simple.h must be suppressed — if it
        // leaked through, the generated code would contain the recursive
        // `pub type bool = bool;` and fail to compile.
        //
        // Verify the function that uses `bool` return type compiles and works.
        unsafe {
            let w = Widget {
                name: c"test".as_ptr().cast_mut(),
                values: [0; 4],
                color: COLOR_RED,
            };
            assert!(widget_is_visible(&w as *const Widget));
            assert!(!widget_is_visible(std::ptr::null()));
        }
    }

    #[test]
    fn test_int128_typedefs_skipped() {
        // `typedef __int128 __s128;` and `typedef unsigned __int128 __u128;`
        // in simple.h must be silently skipped — WinMD has no 128-bit integer
        // type. Typedef chains through them (`typedef __s128 s128;`) must also
        // be skipped recursively. Verify none appear in the generated bindings.
        let bindings = include_str!("bindings.rs");
        assert!(
            !bindings.contains("__s128"),
            "__s128 should not appear in generated bindings"
        );
        assert!(
            !bindings.contains("__u128"),
            "__u128 should not appear in generated bindings"
        );
        assert!(
            !bindings.contains("pub type s128"),
            "s128 (chained typedef) should not appear in generated bindings"
        );
        assert!(
            !bindings.contains("pub type u128"),
            "u128 (chained typedef) should not appear in generated bindings"
        );
    }

    // -----------------------------------------------------------------------
    // Injected types (from [[inject_type]] in simple.toml)
    // -----------------------------------------------------------------------

    #[test]
    fn test_injected_enum() {
        // Priority enum injected via TOML, not extracted from C headers.
        assert_eq!(PRIORITY_LOW, 0u32);
        assert_eq!(PRIORITY_MEDIUM, 1u32);
        assert_eq!(PRIORITY_HIGH, 2u32);
    }

    #[test]
    fn test_injected_typedef() {
        // handle_t injected as a u64 typedef.
        let _h: handle_t = 42u64;
        assert_eq!(std::mem::size_of::<handle_t>(), 8);
    }

    #[test]
    fn test_injected_opaque_struct() {
        // OpaqueCtx injected as an opaque struct with size=32, align=8.
        assert_eq!(std::mem::size_of::<OpaqueCtx>(), 32);
        assert_eq!(std::mem::align_of::<OpaqueCtx>(), 8);
    }

    #[test]
    fn test_injected_conflict_extracted_wins() {
        // Color is both extracted from simple.h (3 variants: RED=0, GREEN=1, BLUE=2)
        // and injected in TOML (1 variant: INJECTED=99). Extracted should win.
        assert_eq!(COLOR_RED, 0u32);
        assert_eq!(COLOR_GREEN, 1u32);
        assert_eq!(COLOR_BLUE, 2u32);
        // COLOR_INJECTED should NOT exist in bindings.
        let bindings = include_str!("bindings.rs");
        assert!(
            !bindings.contains("COLOR_INJECTED"),
            "injected Color should be skipped when extracted version exists"
        );
    }

    #[test]
    fn test_c11_anonymous_union_member() {
        // HasAnonUnion has a C11 anonymous union (no field name):
        //   struct { int before; union { int x; float y; }; int after; }
        // The union must not be dropped — struct size and field offsets
        // must match the C layout.
        assert_eq!(std::mem::size_of::<HasAnonUnion>(), 12);
        assert_eq!(
            std::mem::offset_of!(HasAnonUnion, before),
            0,
            "before should be at offset 0"
        );
        assert_eq!(
            std::mem::offset_of!(HasAnonUnion, after),
            8,
            "after should be at offset 8 (not 4)"
        );

        // The anonymous union is extracted as HasAnonUnion__anon_0.
        assert_eq!(std::mem::size_of::<HasAnonUnion__anon_0>(), 4);
        let mut h = HasAnonUnion::default();
        h.HasAnonUnion__anon_0.x = 42;
        assert_eq!(unsafe { h.HasAnonUnion__anon_0.y }, f32::from_bits(42));
    }

    #[test]
    #[ignore = "requires windows-bindgen fork with nested ArrayFixed support; upstream emits [[T; 8]; 1] (wrong outer dim)"]
    #[allow(unconditional_panic)]
    #[allow(clippy::out_of_bounds_indexing)]
    fn test_anon_struct_2d_array_field() {
        // `struct { ... } tc_rxq[4][8]` — anonymous struct as 2D array element.
        // Extracted as WithAnon2DArrayField_tc_rxq, field emitted as [[T; 8]; 4].
        // Requires the local windows-bindgen fork — upstream does not handle
        // nested ArrayFixed and silently produces the wrong outer dimension.
        assert_eq!(std::mem::size_of::<WithAnon2DArrayField_tc_rxq>(), 4);
        assert_eq!(
            std::mem::size_of::<WithAnon2DArrayField>(),
            132, // 4 * 8 * 4 (tc_rxq) + 4 (count) = 132
        );
        let mut s = WithAnon2DArrayField::default();
        s.tc_rxq[0][0].base = 1;
        s.tc_rxq[3][7].nb_queue = 255;
        assert_eq!(s.tc_rxq[0][0].base, 1);
        assert_eq!(s.tc_rxq[3][7].nb_queue, 255);
    }

    #[test]
    fn test_anon_struct_array_field() {
        // `struct { ... } entries[4]` — anonymous struct used as array element.
        // The element type must be extracted as WithAnonArrayField_entries and
        // the field emitted as [WithAnonArrayField_entries; 4].
        assert_eq!(std::mem::size_of::<WithAnonArrayField_entries>(), 8);
        assert_eq!(
            std::mem::size_of::<WithAnonArrayField>(),
            36, // 4 * 8 (entries) + 4 (count) = 36
        );
        let mut s = WithAnonArrayField::default();
        s.entries[0].id = 42;
        s.entries[0].mask = 0xDEAD;
        assert_eq!(s.entries[0].id, 42);
        assert_eq!(s.entries[0].mask, 0xDEAD);
        s.count = 4;
        assert_eq!(s.count, 4);
    }

    #[test]
    fn test_bitfield_enum_extracted() {
        // BitfieldKind enum (used in a bitfield context) must be discovered
        // by sonar and emitted with the correct variants.
        assert_eq!(BF_KIND_NONE, 0u32);
        assert_eq!(BF_KIND_FLAG, 1u32);
        assert_eq!(BF_KIND_VALUE, 2u32);

        // WithBitfield has `kind:8` + `flags:24` which pack into a single
        // u32 via bitfield flattening. Struct size must match C (16 bytes).
        assert_eq!(std::mem::size_of::<WithBitfield>(), 16);
        assert_eq!(std::mem::align_of::<WithBitfield>(), 8); // pointer alignment
    }

    #[test]
    fn test_cacheline_aligned_struct() {
        // CacheAligned has __attribute__((aligned(64))), so sizeof == 64
        // even though it only has 8 bytes of fields (x: i32, y: i32).
        assert_eq!(std::mem::size_of::<CacheAligned>(), 64);
    }
}
