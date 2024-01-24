use crate::Button;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{theme_with, ButtonThemeWith};

#[allow(non_snake_case)]
#[component]
pub fn PopupBackground(children: Element) -> Element {
    rsx!(rect {
        height: "100v",
        width: "100v",
        background: "rgb(0, 0, 0, 150)",
        position: "absolute",
        position_top: "0",
        position_left: "0",
        layer: "-1",
        main_align: "center",
        cross_align: "center",
        {children}
    })
}

#[allow(non_snake_case)]
#[component]
pub fn Popup(
    children: Element,
    #[props(default = "325".into(), into)] width: String,
    #[props(default = "225".into(), into)] height: String,

    on_close_request: Option<EventHandler>,
) -> Element {
    rsx!(
        PopupBackground {
            rect {
                padding: "14",
                corner_radius: "8",
                background: "white",
                shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
                width: "{width}",
                height: "{height}",
                rect {
                    position: "absolute",
                    position_right: "0",
                    width: "30",
                    Button {
                        theme: theme_with!(ButtonTheme {
                            padding: "6".into(),
                            margin: "0".into(),
                            width: "30".into(),
                            height: "30".into(),
                            corner_radius: "999".into(),
                        }),
                        onclick: move |_| if let Some(on_close_request) = &on_close_request {
                            on_close_request.call(());
                        },
                        label {
                            font_size: "14",
                            "X"
                        }
                    }
                }
                {children}
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
pub fn PopupTitle(children: Element) -> Element {
    rsx!(
        rect {
            font_size: "18",
            margin: "4 2 8 2",
            font_weight: "bold",
            {children}
        }
    )
}

#[allow(non_snake_case)]
#[component]
pub fn PopupContent(children: Element) -> Element {
    rsx!(
        rect {
            font_size: "15",
            margin: "6 2",
            color: "rgb(20, 20, 20)",
            {children}
        }
    )
}
