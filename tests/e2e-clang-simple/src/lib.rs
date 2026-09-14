#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate bnd_macros as windows_link;

#[cfg(test)]
mod tests {
    use crate::bindings::*;

    #[test]
    fn generated_basic_api_compiles_and_runs() {
        assert_eq!(MAX_DEPTH, 42);
        assert_eq!(std::mem::size_of::<Rect>(), 16);
        assert_eq!(std::mem::align_of::<Rect>(), 4);
        assert_eq!(COLOR_RED, 0u32);
        assert_eq!(COLOR_GREEN, 1u32);
        assert_eq!(COLOR_BLUE, 2u32);

        unsafe {
            let bounds = Rect {
                x: 1,
                y: 2,
                width: 640,
                height: 480,
            };
            assert_eq!(bounds.x, 1);
            assert_eq!(bounds.y, 2);
            assert_eq!(bounds.width, 640);
            assert_eq!(bounds.height, 480);

            let mut widget = Widget::default();

            assert_eq!(create_widget(c"simple".as_ptr(), bounds, &mut widget), 0);
            assert_eq!(widget.values, [1, 2, 640, 480]);
            assert!(widget_is_visible(&widget));
            assert!(!widget_is_visible(std::ptr::null()));
            destroy_widget(&mut widget);
        }
    }

    #[test]
    fn generated_record_layouts_match_c() {
        assert_eq!(std::mem::size_of::<Value>(), 4);
        assert_eq!(std::mem::align_of::<Value>(), 4);
        assert_eq!(std::mem::size_of::<NetAddr>(), 20);
        assert_eq!(std::mem::size_of::<NetAddr_0>(), 16);
        assert_eq!(std::mem::size_of::<MacroNestedUnion_0>(), 8);
        assert_eq!(std::mem::align_of::<MacroNestedUnion_0>(), 8);
        assert_eq!(std::mem::size_of::<MacroNestedUnion>(), 8);
        assert_eq!(std::mem::align_of::<MacroNestedUnion>(), 8);
        assert_eq!(std::mem::offset_of!(MacroNestedUnion, dummy), 0);
        assert_eq!(std::mem::size_of::<HasAnonUnion>(), 12);
        assert_eq!(std::mem::size_of::<WithBitfield>(), 16);
        assert_eq!(std::mem::align_of::<WithBitfield>(), 8);
        assert_eq!(std::mem::size_of::<PartialBitfield>(), 4);
        assert_eq!(std::mem::align_of::<PartialBitfield>(), 4);
        assert_eq!(std::mem::offset_of!(PartialBitfield, next), 3);
    }

    #[test]
    fn generated_unions_support_alias_reads() {
        let value = Value { f: 1.0 };
        unsafe {
            assert_eq!(value.f, 1.0);
            assert_eq!(value.i, 0x3f80_0000);
            assert_eq!(value.bytes, 1.0_f32.to_ne_bytes());
        }

        let mut address = NetAddr::default();
        address.addr.dwords = [0x0403_0201, 0x0807_0605, 0x0c0b_0a09, 0x100f_0e0d];
        unsafe {
            assert_eq!(address.addr.bytes[0], 0x01);
            assert_eq!(address.addr.bytes[15], 0x10);
            assert_eq!(address.addr.words[0], 0x0201);
        }
        address.scope_id = 42;
        assert_eq!(address.scope_id, 42);
    }

    #[test]
    fn generated_c11_anonymous_union_matches_c() {
        assert_eq!(std::mem::offset_of!(HasAnonUnion, before), 0);
        assert_eq!(std::mem::offset_of!(HasAnonUnion, after), 8);
        assert_eq!(std::mem::size_of::<HasAnonUnion_0>(), 4);

        let mut value = HasAnonUnion::default();
        value.Anonymous.x = 42;
        assert_eq!(unsafe { value.Anonymous.y }, f32::from_bits(42));
    }

    #[test]
    fn generated_anonymous_arrays_match_c() {
        assert_eq!(std::mem::size_of::<WithAnon2DArrayField_0>(), 4);
        assert_eq!(std::mem::size_of::<WithAnon2DArrayField>(), 132);
        let mut matrix = WithAnon2DArrayField::default();
        matrix.tc_rxq[0][0].base = 1;
        matrix.tc_rxq[3][7].nb_queue = 255;
        matrix.count = 32;
        assert_eq!(matrix.tc_rxq[0][0].base, 1);
        assert_eq!(matrix.tc_rxq[3][7].nb_queue, 255);
        assert_eq!(matrix.count, 32);

        assert_eq!(std::mem::size_of::<WithAnonArrayField_0>(), 8);
        assert_eq!(std::mem::size_of::<WithAnonArrayField>(), 36);
        let mut entries = WithAnonArrayField::default();
        entries.entries[0].id = 42;
        entries.entries[0].mask = 0xdead;
        entries.count = 4;
        assert_eq!(entries.entries[0].id, 42);
        assert_eq!(entries.entries[0].mask, 0xdead);
        assert_eq!(entries.count, 4);
    }

    #[test]
    fn generated_bitfield_enum_values_match_c() {
        assert_eq!(BF_KIND_NONE, 0u32);
        assert_eq!(BF_KIND_FLAG, 1u32);
        assert_eq!(BF_KIND_VALUE, 2u32);
    }

    #[test]
    fn generated_aligned_records_match_c() {
        assert_eq!(std::mem::size_of::<CacheAligned>(), 64);
        assert_eq!(std::mem::align_of::<CacheAligned>(), 64);
        assert_eq!(std::mem::size_of::<AlignedInner>(), 64);
        assert_eq!(std::mem::align_of::<AlignedInner>(), 64);
        assert_eq!(std::mem::size_of::<EmbeddingAligned>(), 192);
        assert_eq!(std::mem::offset_of!(EmbeddingAligned, aligned_member), 64);
        assert_eq!(std::mem::offset_of!(EmbeddingAligned, after), 128);
    }

    #[test]
    fn stored_va_list_layout_matches_c_abi() {
        assert_eq!(std::mem::size_of::<test_va_list>(), 24);
        assert_eq!(std::mem::align_of::<test_va_list>(), 8);
        assert_eq!(std::mem::size_of::<SavedVaList>(), 32);
        assert_eq!(std::mem::align_of::<SavedVaList>(), 8);
        assert_eq!(std::mem::offset_of!(SavedVaList, tail), 24);
    }

    #[test]
    fn generated_callback_uses_c_abi() {
        unsafe extern "C" fn compare(
            left: *const core::ffi::c_void,
            right: *const core::ffi::c_void,
        ) -> i32 {
            unsafe { *(left.cast::<i32>()) - *(right.cast::<i32>()) }
        }

        let callback: CompareFunc = Some(compare);
        assert!(callback.is_some());
    }

    #[test]
    fn unsupported_int128_typedefs_are_not_generated() {
        let bindings = include_str!(concat!(env!("OUT_DIR"), "/bindings.rs"));

        for unsupported in ["__s128", "__u128", "pub type s128", "pub type u128"] {
            assert!(
                !bindings.contains(unsupported),
                "{unsupported} should not be generated"
            );
        }
    }
}
