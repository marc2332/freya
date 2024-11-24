/// Generate a Dioxus component rendering the specified SVG.
///
/// Example:
///
/// ```no_run
/// # use freya::prelude::*;
///
/// import_svg!(Ferris, "../../../examples/ferris.svg", "100%", "100%");
/// import_svg!(FerrisWithRequiredSize, "../../../examples/ferris.svg");
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
macro_rules! import_svg {
    ($component_name:ident, $path:expr, $width: expr, $height: expr) => {
        // Generate a function with the name derived from the file name
        #[allow(non_snake_case)]
        #[dioxus::prelude::component]
        pub fn $component_name(
            #[props(default = $width.to_string())] width: String,
            #[props(default = $height.to_string())] height: String,
            color: Option<String>,
            fill: Option<String>,
            stroke: Option<String>,
        ) -> freya::prelude::Element {
            use freya::prelude::*;
            let svg_data = static_bytes(include_bytes!($path));

            rsx!(svg {
                color,
                fill,
                stroke,
                width,
                height,
                svg_data,
            })
        }
    };
    ($component_name:ident, $path:expr) => {
        // Generate a function with the name derived from the file name
        #[allow(non_snake_case)]
        #[dioxus::prelude::component]
        pub fn $component_name(
            width: String,
            height: String,
            color: Option<String>,
            fill: Option<String>,
            stroke: Option<String>,
        ) -> freya::prelude::Element {
            use freya::prelude::*;
            let svg_data = static_bytes(include_bytes!($path));

            rsx!(svg {
                color,
                fill,
                stroke,
                width,
                height,
                svg_data,
            })
        }
    };
}
