use std::time::Duration;

use crate::Loader;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{
    use_applied_theme, use_asset_cacher, use_focus, AssetConfiguration, NetworkImageTheme,
    NetworkImageThemeWith,
};
use freya_node_state::bytes_to_data;
use reqwest::Url;

/// [`NetworkImage`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct NetworkImageProps {
    /// Theme override.
    pub theme: Option<NetworkImageThemeWith>,

    /// URL of the image
    pub url: Url,

    /// Fallback element
    pub fallback: Option<Element>,

    /// Loading element
    pub loading: Option<Element>,

    /// Information about the image.
    pub alt: Option<String>,
}

/// Image status.
#[derive(PartialEq)]
pub enum ImageStatus {
    /// Image is being fetched.
    Loading,

    /// Image fetching threw an error.
    Errored,

    /// Image has been fetched.
    Loaded(Signal<Vec<u8>>),
}

/// `NetworkImage` component.
///
/// # Props
/// See [`NetworkImageProps`].
///
/// # Example
///  
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         NetworkImage {
///             url: "https://raw.githubusercontent.com/jigsawpieces/dog-api-images/main/greyhound/Cordelia.jpg".parse().unwrap()
///         }
///     )
/// }
///
#[allow(non_snake_case)]
pub fn NetworkImage(props: NetworkImageProps) -> Element {
    let asset_cacher = use_asset_cacher();
    let focus = use_focus();
    let mut status = use_signal(|| ImageStatus::Loading);

    let focus_id = focus.attribute();
    let NetworkImageTheme { width, height } = use_applied_theme!(&props.theme, network_image);
    let alt = props.alt.as_deref();

    // TODO: Waiting for a dependency-based use_effect
    let _ = use_memo_with_dependencies(&props.url, move |url| {
        let asset_configuration = AssetConfiguration {
            duration: Duration::from_secs(3600),
            id: url.to_string(),
        };

        // Loading image
        status.set(ImageStatus::Loading);
        if let Some(asset) = asset_cacher.get(&asset_configuration) {
            // Image loaded from cache
            status.set(ImageStatus::Loaded(asset))
        } else {
            spawn(async move {
                let asset = fetch_image(url).await;
                if let Ok(asset) = asset {
                    let asset_signal = asset_cacher.insert(asset_configuration, asset);
                    // Image loaded
                    status.set(ImageStatus::Loaded(asset_signal))
                } else if let Err(_err) = asset {
                    // Image errored
                    status.set(ImageStatus::Errored)
                }
            });
        }
    });

    if let ImageStatus::Loaded(bytes) = &*status.read() {
        let image_data = bytes_to_data(&bytes.read());
        rsx!(image {
            height: "{height}",
            width: "{width}",
            focus_id,
            image_data,
            role: "image",
            alt
        })
    } else if *status.read() == ImageStatus::Loading {
        if let Some(loading_element) = &props.loading {
            rsx!({ loading_element })
        } else {
            rsx!(
                rect {
                    height: "{height}",
                    width: "{width}",
                    main_align: "center",
                    cross_align: "center",
                    Loader {}
                }
            )
        }
    } else {
        if let Some(fallback_element) = &props.fallback {
            rsx!({ fallback_element })
        } else {
            rsx!(
                rect {
                    height: "{height}",
                    width: "{width}",
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

async fn fetch_image(url: Url) -> Result<Vec<u8>, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let data = res.bytes().await?;
    Ok(data.to_vec())
}
