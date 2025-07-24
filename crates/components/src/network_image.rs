use bytes::Bytes;
use dioxus::prelude::*;
use freya_core::custom_attributes::dynamic_bytes;
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_asset,
    use_asset_cacher,
    use_focus,
    AssetAge,
    AssetBytes,
    AssetConfiguration,
};
use reqwest::Url;
use tracing::info;

use crate::Loader;

/// Properties for the [`NetworkImage`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NetworkImageProps {
    /// Width of the image container. Default to `auto`.
    #[props(default = "auto".into())]
    pub width: String,
    /// Height of the image container. Default to `auto`.
    #[props(default = "auto".into())]
    pub height: String,
    /// Min width of the image container.
    pub min_width: Option<String>,
    /// Min height of the image container.
    pub min_height: Option<String>,
    /// URL of the image.
    pub url: ReadOnlySignal<Url>,
    /// Fallback element.
    pub fallback: Option<Element>,
    /// Loading element.
    pub loading: Option<Element>,
    /// Information about the image.
    pub alt: Option<String>,
    /// Aspect ratio of the image.
    pub aspect_ratio: Option<String>,
    /// Cover of the image.
    pub cover: Option<String>,
    /// Image sampling algorithm.
    pub sampling: Option<String>,
}

/// Image status.
#[doc(hidden)]
#[derive(PartialEq)]
pub enum ImageState {
    /// Image is being fetched.
    Loading,

    /// Image fetching threw an error.
    Errored,

    /// Image has been fetched.
    Loaded(Bytes),
}

/// Image component that automatically fetches and caches remote (HTTP) images.
///
/// Requires the `network_image` feature.
///
/// # Example
///
/// ```rust
/// # use reqwest::Url;
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         NetworkImage {
///             width: "100%",
///             height: "100%",
///             url: "https://raw.githubusercontent.com/marc2332/freya/refs/heads/main/examples/rust_logo.png".parse::<Url>().unwrap()
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # import_image!(Rust, "../../../examples/rust_logo.png", {
/// #   width: "100%",
/// #   height: "100%"
/// # });
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           Rust { }
/// #       }
/// #   )
/// # }, (250., 250.).into(), "./images/gallery_network_image.png");
/// ```
///
/// # Preview
/// ![NetworkImage Preview][network_image]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("network_image", "images/gallery_network_image.png")
)]
#[allow(non_snake_case)]
pub fn NetworkImage(
    NetworkImageProps {
        width,
        height,
        min_width,
        min_height,
        url,
        fallback,
        loading,
        alt,
        aspect_ratio,
        cover,
        sampling,
    }: NetworkImageProps,
) -> Element {
    let focus = use_focus();
    let asset_config = AssetConfiguration {
        age: AssetAge::default(),
        id: url.to_string(),
    };
    let asset_bytes = use_asset(asset_config.clone());
    let mut asset_cacher = use_asset_cacher();
    let mut assets_tasks = use_signal::<Vec<Task>>(Vec::new);

    use_effect(move || {
        let url = url();

        // Cancel previous asset fetching requests
        for asset_task in assets_tasks.write().drain(..) {
            asset_task.cancel();
        }

        // Fetch asset if still pending or errored
        if matches!(
            asset_cacher.read_asset(&asset_config),
            Some(AssetBytes::Pending) | Some(AssetBytes::Error(_))
        ) {
            // Mark asset as loading
            asset_cacher.update_asset(asset_config.clone(), AssetBytes::Loading);

            let asset_config = asset_config.clone();
            let asset_task = spawn(async move {
                info!("Fetching image {url}");
                let asset = fetch_image(url).await;
                if let Ok(asset_bytes) = asset {
                    // Image loaded
                    asset_cacher
                        .update_asset(asset_config.clone(), AssetBytes::Cached(asset_bytes));
                } else if let Err(err) = asset {
                    // Image errored asset_cacher
                    asset_cacher.update_asset(asset_config, AssetBytes::Error(err.to_string()));
                }
            });

            assets_tasks.write().push(asset_task);
        }
    });

    let a11y_id = focus.attribute();

    match asset_bytes {
        AssetBytes::Cached(bytes) => {
            let image_data = dynamic_bytes(bytes);
            rsx!(image {
                height,
                width,
                min_width,
                min_height,
                a11y_id,
                image_data,
                a11y_role: "image",
                a11y_name: alt,
                aspect_ratio,
                cover,
                cache_key: "{url}",
                sampling,
            })
        }
        AssetBytes::Pending | AssetBytes::Loading => {
            if let Some(loading_element) = loading {
                rsx!({ loading_element })
            } else {
                rsx!(
                    rect {
                        height,
                        width,
                        min_width,
                        min_height,
                        main_align: "center",
                        cross_align: "center",
                        Loader {}
                    }
                )
            }
        }
        AssetBytes::Error(err) => {
            if let Some(fallback_element) = fallback {
                rsx!({ fallback_element })
            } else {
                rsx!(
                    rect {
                        height,
                        width,
                        min_width,
                        min_height,
                        main_align: "center",
                        cross_align: "center",
                        overflow: "clip",
                        label {
                            text_align: "center",
                            "Error: '{err}'"
                        }
                    }
                )
            }
        }
    }
}

async fn fetch_image(url: Url) -> reqwest::Result<Bytes> {
    let res = reqwest::get(url).await?;
    res.bytes().await
}
