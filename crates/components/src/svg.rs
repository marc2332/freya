/// Generate a Dioxus component rendering the specified SVG.
///
/// ### Example
///
/// ```no_run
/// # use freya::prelude::*;
///
/// import_svg!(Ferris, "../../../examples/ferris.svg",
///     width: "auto",
///     height: "40%",
/// });
///
/// fn app() -> Element {
///     rsx!(
///         Ferris {
///            width: "150",
///         }
///     )
/// }
/// ```
#[macro_export]
macro_rules! import_svg {
    ($component_name:ident, $path:expr, { $($key:ident : $value:expr),* $(,)? }) => {
        #[allow(non_snake_case)]
        #[dioxus::prelude::component]
        pub fn $component_name(
            $(#[props(default = $value.to_string())] $key: String,)*
        ) -> freya::prelude::Element {
            use freya::prelude::*;
            let svg_data = static_bytes(include_bytes!($path));

            rsx!(svg {
                $($key: $key,)*
                svg_data,
            })
        }
    };
}
