use bytes::Bytes;
use dioxus::prelude::*;
use freya_elements::{
    self as dioxus_elements,
};
use freya_hooks::{
    use_asset,
    use_asset_cacher,
    AssetAge,
    AssetBytes,
    AssetConfiguration,
};
pub use reqwest::Url;
use tracing::info;

use crate::{
    Gif,
    Loader,
};

#[component]
pub fn RemoteGif(
    url: ReadOnlySignal<Url>,
    /// Information about the gif.
    alt: Option<String>,
    /// Aspect ratio of the gif.
    aspect_ratio: Option<String>,
    /// Cover of the gif.
    cover: Option<String>,
    /// Width of the gif container. Default to `auto`.
    #[props(default = "auto".into())]
    width: String,
    /// Height of the gif container. Default to `auto`.
    #[props(default = "auto".into())]
    height: String,
    /// Min width of the gif container.
    min_width: Option<String>,
    /// Min height of the gif container.
    min_height: Option<String>,
    /// Fallback element.
    fallback: Option<Element>,
    /// Loading element.
    loading: Option<Element>,
) -> Element {
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
                info!("Fetching gif {url}");
                let asset = fetch_gif(url).await;
                if let Ok(asset_bytes) = asset {
                    // GIF loaded
                    asset_cacher
                        .update_asset(asset_config.clone(), AssetBytes::Cached(asset_bytes));
                } else if let Err(err) = asset {
                    // GIF errored asset_cacher
                    asset_cacher.update_asset(asset_config, AssetBytes::Error(err.to_string()));
                }
            });

            assets_tasks.write().push(asset_task);
        }
    });

    match asset_bytes {
        AssetBytes::Cached(data) => {
            rsx!(Gif {
                data,
                width,
                height,
                min_width,
                min_height
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

async fn fetch_gif(url: Url) -> reqwest::Result<Bytes> {
    let res = reqwest::get(url).await?;
    res.bytes().await
}
