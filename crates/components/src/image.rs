/// Generate a Dioxus component rendering the specified image.
///
/// ### Example
///
/// ```no_run
/// # use freya::prelude::*;
///
/// // You can pass as many `image` attributes you need, and these will become the default values and also allowed to be overriden.
/// import_image!(RustLogo, "../../../examples/rust_logo.png", {
///     width: "auto",
///     height: "40%",
///     aspect_ratio: "min",
/// });
///
/// fn app() -> Element {
///     rsx!(RustLogo {
///         width: "150",
///     })
/// }
/// ```
#[macro_export]
macro_rules! import_image {
    ($component_name:ident, $path:expr, { $($key:ident : $value:expr),* $(,)? }) => {
        #[allow(non_snake_case)]
        #[dioxus::prelude::component]
        pub fn $component_name(
            $(#[props(default = $value.to_string())] $key: String,)*
        ) -> freya::prelude::Element {
            use freya::prelude::*;
            let image_data = static_bytes(include_bytes!($path));

            rsx!(image {
                $($key: $key,)*
                image_data,
            })
        }
    };
}
