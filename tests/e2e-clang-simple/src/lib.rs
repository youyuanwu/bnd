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

        unsafe {
            let bounds = Rect {
                x: 1,
                y: 2,
                width: 640,
                height: 480,
            };
            let mut widget = Widget::default();

            assert_eq!(create_widget(c"simple".as_ptr(), bounds, &mut widget), 0);
            assert_eq!(widget.values, [1, 2, 640, 480]);
            assert!(widget_is_visible(&widget));
            destroy_widget(&mut widget);
        }
    }

    #[test]
    fn generated_record_layouts_match_c() {
        assert_eq!(std::mem::size_of::<Value>(), 4);
        assert_eq!(std::mem::size_of::<NetAddr>(), 20);
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
    fn generated_anonymous_arrays_match_c() {
        assert_eq!(std::mem::size_of::<WithAnon2DArrayField>(), 132);
        assert_eq!(std::mem::size_of::<WithAnonArrayField>(), 36);
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
