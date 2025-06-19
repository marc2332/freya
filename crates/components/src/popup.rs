use dioxus::prelude::*;
use freya_elements::{
    self as dioxus_elements,
    events::{
        Key,
        KeyboardEvent,
    },
    MouseEvent,
};
use freya_hooks::{
    theme_with,
    use_animation,
    use_applied_theme,
    AnimNum,
    ButtonThemeWith,
    Ease,
    Function,
    OnCreation,
    PopupTheme,
    PopupThemeWith,
};

use crate::{
    BaseButtonProps,
    ButtonBase,
    ButtonProps,
};

/// The background of the [`Popup`] component.
#[allow(non_snake_case)]
#[component]
pub fn PopupBackground(children: Element, onclick: EventHandler<MouseEvent>) -> Element {
    rsx!(rect {
        layer: "-2000",
        rect {
            onclick,
            height: "100v",
            width: "100v",
            background: "rgb(0, 0, 0, 150)",
            position: "global",
            position_top: "0",
            position_left: "0",
        }
        rect {
            height: "100v",
            width: "100v",
            position: "global",
            position_top: "0",
            position_left: "0",
            main_align: "center",
            cross_align: "center",
            {children}
        }
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
///                      text: "Awesome Popup"
///                  }
///                  PopupContent {
///                      label {
///                          "Some content"
///                      }
///                  }
///              }
///          }
///          Button {
///              onpress: move |_| show_popup.set(true),
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
    /// Whether to show or no the cross button in the top right corner. Default to `false`.
    #[props(default = false)]
    show_close_button: bool,
    /// Whether to trigger close request handler when the Escape key is pressed.
    #[props(default = true)]
    close_on_escape_key: bool,
) -> Element {
    let animations = use_animation(|conf| {
        conf.on_creation(OnCreation::Run);
        (
            AnimNum::new(0.85, 1.)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo),
            AnimNum::new(0.2, 1.)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo),
        )
    });
    let PopupTheme {
        background,
        color,
        width,
        height,
    } = use_applied_theme!(&theme, popup);

    let scale = animations.get();
    let (scale, opacity) = &*scale.read();

    let request_to_close = move || {
        if let Some(oncloserequest) = &oncloserequest {
            oncloserequest.call(());
        }
    };

    let onglobalkeydown = move |event: KeyboardEvent| {
        if close_on_escape_key && event.key == Key::Escape {
            request_to_close()
        }
    };

    rsx!(
        PopupBackground {
            onclick: move |_| request_to_close(),
            rect {
                scale: "{scale.read()} {scale.read()}",
                opacity: "{opacity.read()}",
                corner_radius: "12",
                background: "{background}",
                color: "{color}",
                shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
                width: "{width}",
                height: "{height}",
                overflow: "clip",
                spacing: "20",
                onglobalkeydown,
                {children}
            }
        }
    )
}

/// Optionally use a styled title inside a [`Popup`].
#[allow(non_snake_case)]
#[component]
pub fn PopupTitle(text: ReadOnlySignal<String>) -> Element {
    rsx!(
        rect {
            font_size: "18",
            font_weight: "bold",
            text_align: "center",
            padding: "20 0 0 0",
            label {
                width: "fill",
                {text}
            }
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
            padding: "0 16",
            {children}
        }
    )
}

#[allow(non_snake_case)]
#[component]
pub fn PopupButtons(children: Element) -> Element {
    rsx!(
        rect {
            height: "40",
            content: "flex",
            direction: "horizontal",
            {children}
        }
    )
}

#[allow(non_snake_case)]
pub fn PopupButton(props: ButtonProps) -> Element {
    let theme = theme_with!(ButtonTheme {
        width: "flex(1)".into(),
        height: "100%".into(),
        padding: "8".into(),
        border_fill: "none".into(),
        corner_radius: "0".into(),
    });
    let theme = use_applied_theme!(&Some(theme), button);
    ButtonBase(BaseButtonProps {
        theme,
        children: props.children,
        onpress: props.onpress,
        onclick: props.onclick,
        enabled: props.enabled,
    })
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_elements::events::keyboard::{
        Code,
        Key,
        Modifiers,
    };
    use freya_testing::prelude::*;
    use tokio::time::sleep;

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
                    onpress: move |_| show_popup.set(true),
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
        utils.click_cursor((15., 15.)).await;
        sleep(Duration::from_millis(150)).await;
        utils.wait_for_update().await;

        // Check the popup is opened
        assert_eq!(utils.sdom().get().layout().size(), 9);

        utils.click_cursor((25., 25.)).await;

        // Check the popup is closed
        assert_eq!(utils.sdom().get().layout().size(), 4);

        // Open the popup
        utils.click_cursor((15., 15.)).await;

        // Send a random globalkeydown event
        utils.push_event(TestEvent::Keyboard {
            name: EventName::KeyDown,
            key: Key::ArrowDown,
            code: Code::ArrowDown,
            modifiers: Modifiers::empty(),
        });
        utils.wait_for_update().await;
        // Check the popup is still open
        assert_eq!(utils.sdom().get().layout().size(), 9);

        // Send a ESC globalkeydown event
        utils.push_event(TestEvent::Keyboard {
            name: EventName::KeyDown,
            key: Key::Escape,
            code: Code::Escape,
            modifiers: Modifiers::empty(),
        });
        utils.wait_for_update().await;
        // Check the popup is closed
        assert_eq!(utils.sdom().get().layout().size(), 4);
    }
}
