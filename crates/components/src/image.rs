/// Generate a Dioxus component rendering the specified image.
///
/// Example:
///
/// ```no_run
/// # use freya::prelude::*;
///
/// import_svg!(Ferris, "../../../examples/rust_logo.png", "100%", "100%");
/// import_svg!(FerrisWithRequiredSize, "../../../examples/rust_logo.png");
///
/// fn app() -> Element {
///     rsx!(Ferris {})
/// }
///
/// fn another_app() -> Element {
///     rsx!(FerrisWithRequiredSize {
///         width: "150",
///         height: "40%",
///     })
/// }
/// ```
#[macro_export]
macro_rules! import_image {
    ($component_name:ident, $path:expr, $width: expr, $height: expr) => {
        // Generate a function with the name derived from the file name
        #[allow(non_snake_case)]
        #[dioxus::prelude::component]
        pub fn $component_name(
            #[props(default = $width.to_string())] width: String,
            #[props(default = $height.to_string())] height: String,
        ) -> freya::prelude::Element {
            use freya::prelude::*;
            let image_data = static_bytes(include_bytes!($path));

            rsx!(image {
                width,
                height,
                image_data
            })
        }
    };
    ($component_name:ident, $path:expr) => {
        // Generate a function with the name derived from the file name
        #[allow(non_snake_case)]
        #[dioxus::prelude::component]
        pub fn $component_name(width: String, height: String) -> freya::prelude::Element {
            use freya::prelude::*;
            let image_data = static_bytes(include_bytes!($path));

            rsx!(image {
                width,
                height,
                image_data
            })
        }
    };
}
