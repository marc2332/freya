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

    /// `sampling` controls how an `image` element is resized when scaling from its original size to smaller or larger sizes.
    ///
    /// Accepted values:
    /// - `nearest` or `none` (default): The image will be resized using nearest-neighbor interpolation.
    /// - `bilinear`: The image will be resized using bilinear interpolation.
    /// - `trilinear`: The image will be resized using trilinear interpolation.
    /// - `mitchell`: The image will be resized using Mitchell-Netravali interpolation, also known as Bicubic.
    /// - `catmull-rom`: The image will be resized using Catmull-Rom interpolation.
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
    ///             width: "96",
    ///             height: "96",
    ///             sampling: "trilinear",
    ///         }
    ///     )
    /// }
    /// ```
    sampling,
);
