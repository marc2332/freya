use crate::Loader;
use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_node_state::bytes_to_data;
use reqwest::Url;

/// [`NetworkImage`] component properties.
#[derive(Props)]
pub struct NetworkImageProps<'a> {
    /// URL of the image
    pub url: Url,

    /// Fallback element
    #[props(optional)]
    pub fallback: Option<Element<'a>>,

    /// Loading element
    #[props(optional)]
    pub loading: Option<Element<'a>>,

    /// Width of image, default is 100%
    #[props(default = "100%".to_string())]
    pub width: String,

    /// Height of image, default is 100%
    #[props(default = "100%".to_string())]
    pub height: String,
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
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
///         NetworkImage {
///             url: "https://raw.githubusercontent.com/jigsawpieces/dog-api-images/main/greyhound/Cordelia.jpg".parse().unwrap()
///         }
///     )
/// }
///
#[allow(non_snake_case)]
pub fn NetworkImage<'a>(cx: Scope<'a, NetworkImageProps<'a>>) -> Element<'a> {
    let status = use_state(cx, || ImageStatus::Loading);
    let image_bytes = use_state::<Option<Vec<u8>>>(cx, || None);

    let NetworkImageProps { width, height, .. } = cx.props;

    use_effect(cx, &cx.props.url, move |url| {
        to_owned![image_bytes, status];
        async move {
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
        }
    });

    if *status.get() == ImageStatus::Loading {
        if let Some(loading_element) = &cx.props.loading {
            render!(loading_element)
        } else {
            render!(
                rect {
                    height: "{height}",
                    width: "{width}",
                    display: "center",
                    direction: "both",
                    Loader {

                    }
                }
            )
        }
    } else if *status.get() == ImageStatus::Errored {
        if let Some(fallback_element) = &cx.props.fallback {
            render!(fallback_element)
        } else {
            render!(
                rect {
                    height: "{height}",
                    width: "{width}",
                    display: "center",
                    label {
                        align: "center",
                        "Error"
                    }
                }
            )
        }
    } else {
        render! {
            image_bytes.as_ref().map(|bytes| {
                let image_data = bytes_to_data(cx, bytes);
                rsx!(
                    image {
                        height: "{height}",
                        width: "{width}",
                        image_data: image_data
                    }
                )
            })
        }
    }
}

async fn fetch_image(url: Url) -> Result<Vec<u8>, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let data = res.bytes().await?;
    Ok(data.to_vec())
}
