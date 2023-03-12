use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_node_state::bytes_to_data;
use reqwest::Url;

/// [`NetworkImage`] component properties.
#[derive(Props)]
pub struct NetworkImageProps<'a> {
    /// URL of the image
    pub url: Url,

    // Fallback element
    #[props(optional)]
    pub fallback: Option<Element<'a>>,

    // Loading element
    #[props(optional)]
    pub loading: Option<Element<'a>>,
}

#[derive(PartialEq)]
pub enum ImageStatus {
    Loading,
    Errored,
    Loaded,
}

/// `NetworkImage` component.
///
/// # Props
/// See [`NetworkImageProps`].
///
#[allow(non_snake_case)]
pub fn NetworkImage<'a>(cx: Scope<'a, NetworkImageProps<'a>>) -> Element<'a> {
    let status = use_state(cx, || ImageStatus::Loading);
    let image_bytes = use_state::<Option<Vec<u8>>>(cx, || None);

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

    render!(
        image {
            if *status.get() == ImageStatus::Loading {
                if let Some(loading_element) =  &cx.props.loading {
                    rsx!(
                        loading_element
                    )
                } else {
                    rsx!(
                        rect {
                            height: "100%",
                            width: "100%",
                            display: "center",
                            direction: "both",
                            label {
                                align: "center",
                                "..."
                            }
                        }
                    )
                }
            } else if *status.get() == ImageStatus::Errored {
                if let Some(fallback_element) =  &cx.props.fallback {
                    rsx!(
                        fallback_element
                    )
                } else {
                    rsx!(
                        rect {
                            height: "100%",
                            width: "100%",
                            display: "center",
                            label {
                                align: "center",
                                "Error"
                            }
                        }
                    )
                }
            } else if *status.get() == ImageStatus::Loaded {
                rsx!{
                    image_bytes.as_ref().map(|bytes| {
                        let image_data = bytes_to_data(cx, bytes);
                        rsx!(
                            image {
                                width: "100%",
                                height: "100%",
                                image_data: image_data
                            }
                        )
                    })
                }
            }
        }
    )
}

async fn fetch_image(url: Url) -> Result<Vec<u8>, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let data = res.bytes().await?;
    Ok(data.to_vec())
}
