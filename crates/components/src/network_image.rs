use bytes::Bytes;
use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_asset_cacher,
    use_focus,
    AssetAge,
    AssetConfiguration,
};
use freya_node_state::dynamic_bytes;
use reqwest::Url;

use crate::Loader;

/// Properties for the [`NetworkImage`] component.
#[derive(Props, Clone, PartialEq)]
pub struct NetworkImageProps {
    /// Width of the image container. Default to `fill`.
    #[props(default = "fill".into())]
    pub width: String,
    /// Height of the image container. Default to `fill`.
    #[props(default = "fill".into())]
    pub height: String,
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
/// # import_image!(Rust, "../../../examples/rust_logo.png", "100%", "100%");
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           Rust { }
/// #       }
/// #   )
/// # }, (185., 185.).into(), "./images/gallery_network_image.png");
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
        url,
        fallback,
        loading,
        alt,
        aspect_ratio,
        cover,
    }: NetworkImageProps,
) -> Element {
    let mut asset_cacher = use_asset_cacher();
    let focus = use_focus();
    let mut status = use_signal(|| ImageState::Loading);
    let mut cached_assets = use_signal::<Vec<AssetConfiguration>>(Vec::new);
    let mut assets_tasks = use_signal::<Vec<Task>>(Vec::new);

    let a11y_id = focus.attribute();

    use_effect(move || {
        let url = url.read().clone();
        // Cancel previous asset fetching requests
        for asset_task in assets_tasks.write().drain(..) {
            asset_task.cancel();
        }

        // Stop using previous assets
        for cached_asset in cached_assets.write().drain(..) {
            asset_cacher.unuse_asset(cached_asset);
        }

        let asset_configuration = AssetConfiguration {
            age: AssetAge::default(),
            id: url.to_string(),
        };

        // Loading image
        status.set(ImageState::Loading);
        if let Some(asset) = asset_cacher.use_asset(&asset_configuration) {
            // Image loaded from cache
            status.set(ImageState::Loaded(asset));
            cached_assets.write().push(asset_configuration);
        } else {
            let asset_task = spawn(async move {
                let asset = fetch_image(url).await;
                if let Ok(asset_bytes) = asset {
                    asset_cacher.cache_asset(
                        asset_configuration.clone(),
                        asset_bytes.clone(),
                        true,
                    );
                    // Image loaded
                    status.set(ImageState::Loaded(asset_bytes));
                    cached_assets.write().push(asset_configuration);
                } else if let Err(_err) = asset {
                    // Image errored
                    status.set(ImageState::Errored);
                }
            });

            assets_tasks.write().push(asset_task);
        }
    });

    match &*status.read_unchecked() {
        ImageState::Loaded(bytes) => {
            let image_data = dynamic_bytes(bytes.clone());
            rsx!(image {
                height,
                width,
                a11y_id,
                image_data,
                a11y_role: "image",
                a11y_name: alt,
                aspect_ratio,
                cover,
            })
        }
        ImageState::Loading => {
            if let Some(loading_element) = loading {
                rsx!({ loading_element })
            } else {
                rsx!(
                    rect {
                        height,
                        width,
                        main_align: "center",
                        cross_align: "center",
                        Loader {}
                    }
                )
            }
        }
        _ => {
            if let Some(fallback_element) = fallback {
                rsx!({ fallback_element })
            } else {
                rsx!(
                    rect {
                        height,
                        width,
                        main_align: "center",
                        cross_align: "center",
                        label {
                            text_align: "center",
                            "Error"
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
