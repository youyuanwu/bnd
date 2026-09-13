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
    use crate::bindings::MultiTest::Types::*;
    use crate::bindings::MultiTest::Widgets::*;

    #[test]
    fn generated_multi_constants_and_types_compile() {
        assert_eq!(MAX_WIDGETS, 256);
        assert_eq!(DEFAULT_WIDTH, 800);
        assert_eq!(DEFAULT_HEIGHT, 600);

        let bounds = Rect {
            x: 10,
            y: 20,
            width: 100,
            height: 200,
        };
        assert_eq!(std::mem::size_of_val(&bounds), 16);

        let widget = Widget {
            name: std::ptr::null(),
            values: [0; 4],
            color: COLOR_GREEN,
        };
        assert_eq!(widget.color, COLOR_GREEN);
    }

    #[test]
    fn generated_callback_compiles() {
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
    fn generated_functions_link_and_run() {
        unsafe {
            assert_eq!(widget_count(), 0);

            let name = c"clang".as_ptr();
            let bounds = Rect {
                x: 1,
                y: 2,
                width: 640,
                height: 480,
            };
            let mut widget = Widget::default();

            assert_eq!(create_widget(name, bounds, &mut widget), 0);
            assert_eq!(widget_count(), 1);
            assert_eq!(widget.values, [1, 2, 640, 480]);
            assert_eq!(widget.color, COLOR_RED);

            destroy_widget(&mut widget);
            assert_eq!(widget_count(), 0);
        }
    }
}
