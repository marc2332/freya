use crate::{Button, CrossIcon};
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{theme_with, use_applied_theme, ButtonThemeWith, PopupTheme, PopupThemeWith};

/// The background of the [`Popup`] component.
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

/// Floating window intended for quick interactions. Also called `Dialog` in other frameworks.
///
/// # Styling
/// Inherits the [`PopupTheme`](freya_hooks::PopupTheme) theme.
/// ```rust, no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut show_popup = use_signal(|| false);
///
///     rsx!(
///         if *show_popup.read() {
///              Popup {
///                  oncloserequest: move |_| {
///                      show_popup.set(false)
///                  },
///                  PopupTitle {
///                      label {
///                          "Awesome Popup"
///                      }
///                  }
///                  PopupContent {
///                      label {
///                          "Some content"
///                      }
///                  }
///              }
///          }
///          Button {
///              onclick: move |_| show_popup.set(true),
///              label {
///                  "Open"
///              }
///          }
///     )
/// }
/// ```
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

/// Optionally use a styled title inside a [`Popup`].
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

/// Optionally wrap the content of your [`Popup`] in a styled container.
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

#[cfg(test)]
mod test {
    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn popup() {
        fn popup_app() -> Element {
            let mut show_popup = use_signal(|| false);

            rsx!(
                if *show_popup.read() {
                    Popup {
                        oncloserequest: move |_| {
                            show_popup.set(false)
                        },
                        label {
                            "Hello, World!"
                        }
                    }
                }
                Button {
                    onclick: move |_| show_popup.set(true),
                    label {
                        "Open"
                    }
                }
            )
        }

        let mut utils = launch_test(popup_app);
        utils.wait_for_update().await;

        // Check the popup is closed
        assert_eq!(utils.sdom().get().layout().size(), 4);

        // Open the popup
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Check the popup is opened
        assert_eq!(utils.sdom().get().layout().size(), 10);

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (395.0, 180.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Check the popup is closed
        assert_eq!(utils.sdom().get().layout().size(), 4);
    }
}
