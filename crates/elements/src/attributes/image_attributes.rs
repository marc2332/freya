use crate::def_attribute;

def_attribute!(
    image_data,
    image_reference,

    /// `aspect_ratio` controls how an `image` element is rendered when facing unexpected dimensions.
    ///
    /// Accepted values:
    /// - `none` (default): The image will be rendered with its original dimensions.
    /// - `min`: The image will be rendered with the minimum dimensions possible.
    /// - `max`: The image will be rendered with the maximum dimensions possible.
    ///
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
    ///             width: "100%", // You must specify size otherwise it will default to 0
    ///             height: "100%",
    ///         }
    ///     )
    /// }
    /// ```
    aspect_ratio,

    /// `cache_key` lets you specify an unique identifier for the given image.
    /// This will help Freya cache the image decoding, if the cache_key changes the old
    /// cache will be pruned and the image (changed or not) will be decoded again.
    /// `cache_key` is optinal but its recommended to be used, specialy for high quality images.
    /// You can pass any value that can be transformed into a string. Like a URL.
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
);
