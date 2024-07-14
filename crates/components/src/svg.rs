#[macro_export]
macro_rules! import_svg {
    ($component_name:ident, $path:expr, $width: expr, $height: expr) => {
        // Generate a function with the name derived from the file name
        #[allow(non_snake_case)]
        pub fn $component_name() -> freya::prelude::Element {
            use freya::prelude::*;
            let svg_content = include_str!($path);

            rsx!(svg {
                width: $width,
                height: $height,
                svg_content
            })
        }
    };
}
