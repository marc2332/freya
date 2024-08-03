#[macro_export]
macro_rules! import_svg {
    ($component_name:ident, $path:expr, $width: expr, $height: expr) => {
        use dioxus::prelude::component;
        // Generate a function with the name derived from the file name
        #[allow(non_snake_case)]
        #[component]
        pub fn $component_name(
            #[props(default = $width.to_string())] width: String,
            #[props(default = $height.to_string())] height: String,
        ) -> freya::prelude::Element {
            use freya::prelude::*;
            let svg_content = include_str!($path);

            rsx!(svg {
                width,
                height,
                svg_content
            })
        }
    };
}
