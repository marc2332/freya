use crate::Loader;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, use_focus, NetworkImageTheme, NetworkImageThemeWith};
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
    Loaded,
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
    let focus = use_focus();
    let mut status = use_signal(|| ImageStatus::Loading);
    let mut image_bytes = use_signal::<Option<Vec<u8>>>(|| None);

    let focus_id = focus.attribute();
    let NetworkImageTheme { width, height } = use_applied_theme!(&props.theme, network_image);
    let alt = props.alt.as_deref();

    // TODO: Waiting for a dependency-based use_effect
    let _ = use_memo_with_dependencies(&props.url, move |url| {
        spawn(async move {
            // Loading image
            status.set(ImageStatus::Loading);
            let img = fetch_image(url).await;
            if let Ok(img) = img {
                // Image loaded
                image_bytes.set(Some(img));
                status.set(ImageStatus::Loaded)
            } else if let Err(_err) = img {
                // Image errored
                image_bytes.set(None);
                status.set(ImageStatus::Errored)
            }
        });
    });

    if *status.read() == ImageStatus::Loading {
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
    } else if *status.read() == ImageStatus::Errored {
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
    } else {
        rsx!({
            image_bytes.as_ref().map(|bytes| {
                let image_data = bytes_to_data(&bytes);
                rsx!(image {
                    height: "{height}",
                    width: "{width}",
                    focus_id,
                    image_data: image_data,
                    role: "image",
                    alt: alt
                })
            })
        })
    }
}

async fn fetch_image(url: Url) -> Result<Vec<u8>, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let data = res.bytes().await?;
    Ok(data.to_vec())
}
