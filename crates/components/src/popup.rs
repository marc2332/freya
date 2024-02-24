use crate::{Button, CrossIcon};
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{theme_with, use_applied_theme, ButtonThemeWith, PopupTheme, PopupThemeWith};

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
        layer: "-99",
        main_align: "center",
        cross_align: "center",
        {children}
    })
}

#[allow(non_snake_case)]
#[component]
pub fn Popup(
    /// Theme override.
    theme: Option<PopupThemeWith>,
    /// Popup inner content.
    children: Element,
    /// Optional close request handler.
    oncloserequest: Option<EventHandler>,
    /// Whether to show or no the cross button in the top right corner.
    #[props(default = true)]
    show_close_button: bool,
) -> Element {
    let PopupTheme {
        background,
        color,
        cross_fill,
        width,
        height,
    } = use_applied_theme!(&theme, popup);
    rsx!(
        PopupBackground {
            rect {
                padding: "14",
                corner_radius: "8",
                background: "{background}",
                color: "{color}",
                shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
                width: "{width}",
                height: "{height}",
                if show_close_button {
                    rect {
                        height: "0",
                        width: "fill",
                        cross_align: "end",
                        Button {
                            theme: theme_with!(ButtonTheme {
                                padding: "6".into(),
                                margin: "0".into(),
                                width: "30".into(),
                                height: "30".into(),
                                corner_radius: "999".into(),
                                shadow: "none".into()
                            }),
                            onclick: move |_| if let Some(oncloserequest) = &oncloserequest {
                                oncloserequest.call(());
                            },
                            CrossIcon {
                                fill: cross_fill
                             }
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
            {children}
        }
    )
}
