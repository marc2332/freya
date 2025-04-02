use std::ops::Deref;

use freya::prelude::*;
use freya_testing::prelude::NodeReferenceLayout;
use reqwest::Url;

fn main() {
    launch(ThumbnailCreator);
}

#[component]
pub fn ThumbnailCreator() -> Element {
    let (reference, size) = use_node_signal();

    rsx! {
        rect { width: "100%", height: "100%",
            rect { height: "flex(1)", width: "100%",
                Overlay {
                    element: rsx! {
                        rect { reference, width: "1000", height: "300" }
                    },
                    overlay: rsx! {
                        ImageOverlay { }
                    },
                    size,
                }
            }

        }
    }
}

#[component]
fn ImageOverlay() -> Element {
    rsx! {
        rect {
            NetworkImage {
                url: Url::parse("https://cdn.beatsaver.com/120d40922751fed49e1743473a3d388a50ddcdc4.jpg?quality=lossless").unwrap(),
            }
            label {
                max_width: "100%",
                color: "black",
                "BBB extra  text! dddd 3333  text! dddd 3333  text! dddd 3333"
            }
        }
    }
}

#[component]
#[allow(irrefutable_let_patterns)]
fn Overlay(
    element: Element,
    overlay: Element,
    size: ReadOnlySignal<NodeReferenceLayout>,
) -> Element {
    rsx! {
        rect {
            {element}
            if let size = size.read().deref() {
                rect {
                    position: "absolute",
                    width: "{size.area.width()}",
                    height: "{size.area.height()}",
                    layer: "-1",
                    {overlay}
                }
            }
        }
    }
}
