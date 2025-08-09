use crate::def_attribute;

def_attribute!(
    /// Specify the image data for the image element.
    ///
    /// Provides two main functions for handling image data:
    /// - `static_bytes`: Takes a static byte slice (`&'static [u8]`) for static images.
    /// - `dynamic_bytes`: Takes any value that can be converted into `Bytes` for dynamically loaded images.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// static RUST_LOGO: &[u8] = include_bytes!("../_docs/rust_logo.png");
    ///
    /// fn app() -> Element {
    ///     rsx!(
    ///         image {
    ///             image_data: static_bytes(RUST_LOGO),
    ///             width: "200",
    ///             height: "200",
    ///         }
    ///     )
    /// }
    /// ```
    ///
    /// Using with dynamic bytes:
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// # use std::fs;
    ///
    /// #[component]
    /// fn DynamicImage(image_data: ReadOnlySignal<Vec<u8>>) -> Element {
    ///     rsx!(
    ///         image {
    ///             image_data: dynamic_bytes(image_data.read().clone()),
    ///             width: "200",
    ///             height: "200",
    ///         }
    ///     )
    /// }
    /// ```
    image_data,

    /// Specify an image reference handle for the image element.
    ///
    /// This attribute is primarily used in conjunction with the `use_camera` hook to display camera output.
    image_reference,

    /// `cover` controls how an `image` element position is rendered inside the given dimensions.
    ///
    /// Accepted values:
    /// - `fill` (default): The image will be rendered from the start of the given dimensions.
    /// - `center`: The image will be rendered in the center of the given dimensions.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// static RUST_LOGO: &[u8] = include_bytes!("../_docs/rust_logo.png");
    ///
    /// fn app() -> Element {
    ///     let image_data = static_bytes(RUST_LOGO);
    ///     rsx!(
    ///         image {
    ///             image_data,
    ///             width: "100%",
    ///             height: "100%",
    ///             cover: "center"
    ///         }
    ///     )
    /// }
    /// ```
    cover,

    /// `cache_key` lets you specify an unique identifier for the given image.
    /// This will help Freya cache the image decoding, if the cache_key changes the old
    /// cache will be pruned and the image (changed or not) will be decoded again.
    /// `cache_key` is optinal but its recommended to be used, specialy for high quality images.
    /// You can pass any value that can be transformed into a string. Like a URL.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// static RUST_LOGO: &[u8] = include_bytes!("../_docs/rust_logo.png");
    ///
    /// fn app() -> Element {
    ///     let image_data = static_bytes(RUST_LOGO);
    ///     rsx!(
    ///         image {
    ///             image_data,
    ///             width: "100%",
    ///             height: "100%",
    ///             cache_key: "rust-logo"
    ///         }
    ///     )
    /// }
    /// ```
    cache_key,

    /// `sampling` controls how an `image` element is resized when scaling from its original size to smaller or larger sizes.
    ///
    /// Accepted values:
    /// - `nearest` or `none` (default): The image will be resized using nearest-neighbor interpolation.
    /// - `bilinear`: The image will be resized using bilinear interpolation.
    /// - `trilinear`: The image will be resized using trilinear interpolation.
    /// - `mitchell`: The image will be resized using Mitchell-Netravali interpolation, also known as Bicubic.
    /// - `catmull-rom`: The image will be resized using Catmull-Rom interpolation.
    ///
    /// ### Example
    ///
    /// ```rust, no_run
    /// # use freya::prelude::*;
    /// static RUST_LOGO: &[u8] = include_bytes!("../_docs/rust_logo.png");
    ///
    /// fn app() -> Element {
    ///     let image_data = static_bytes(RUST_LOGO);
    ///     rsx!(
    ///         image {
    ///             image_data: image_data,
    ///             width: "96",
    ///             height: "96",
    ///             sampling: "trilinear",
    ///         }
    ///     )
    /// }
    /// ```
    sampling,
);
