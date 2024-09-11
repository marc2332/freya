use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::{
        KeyboardEvent,
        PointerEvent,
        PointerType,
    },
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    use_platform,
    ButtonTheme,
    ButtonThemeWith,
};
use winit::{
    event::{
        MouseButton,
        TouchPhase,
    },
    window::CursorIcon,
};

pub enum PressEvent {
    Pointer(PointerEvent),
    Key(KeyboardEvent),
}

impl PressEvent {
    pub fn stop_propagation(&self) {
        match &self {
            Self::Pointer(ev) => ev.stop_propagation(),
            Self::Key(ev) => ev.stop_propagation(),
        }
    }
}

/// Properties for the [`Button`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Theme override.
    pub theme: Option<ButtonThemeWith>,
    /// Inner children for the Button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,
    /// Event handler for when the button is clicked. Not recommended, use `onpress` instead.
    pub onclick: Option<EventHandler<()>>,
}

/// Identifies the current status of the Button.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ButtonStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the button.
    Hovering,
}

/// Clickable button.
///
/// # Styling
/// Inherits the [`ButtonTheme`](freya_hooks::ButtonTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         Button {
///             onpress: |_| println!("clicked"),
///             label {
///                 "Click this"
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Button(
    ButtonProps {
        onpress,
        children,
        theme,
        onclick,
    }: ButtonProps,
) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();

    let ButtonTheme {
        background,
        hover_background,
        border_fill,
        focus_border_fill,
        padding,
        margin,
        corner_radius,
        width,
        height,
        font_theme,
        shadow,
    } = use_applied_theme!(&theme, button);

    let onpointerup = {
        to_owned![onpress, onclick];
        move |ev: PointerEvent| {
            focus.focus();
            if let Some(onpress) = &onpress {
                let is_valid = match ev.data.pointer_type {
                    PointerType::Mouse {
                        trigger_button: Some(MouseButton::Left),
                    } => true,
                    PointerType::Touch { phase, .. } => phase == TouchPhase::Ended,
                    _ => false,
                };
                if is_valid {
                    onpress.call(PressEvent::Pointer(ev))
                }
            } else if let Some(onclick) = onclick {
                if let PointerType::Mouse {
                    trigger_button: Some(MouseButton::Left),
                    ..
                } = ev.data.pointer_type
                {
                    onclick.call(())
                }
            }
        }
    };

    use_drop(move || {
        if *status.read() == ButtonStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if focus.validate_keydown(&ev) {
            if let Some(onpress) = &onpress {
                onpress.call(PressEvent::Key(ev))
            }
        }
    };

    let background = match *status.read() {
        ButtonStatus::Hovering => hover_background,
        ButtonStatus::Idle => background,
    };
    let border = if focus.is_selected() {
        format!("2 solid {focus_border_fill}")
    } else {
        format!("1 solid {border_fill}")
    };

    rsx!(
        rect {
            onpointerup,
            onmouseenter,
            onmouseleave,
            onkeydown,
            focus_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            a11_focusable: "true",
            overflow: "clip",
            a11y_role:"button",
            color: "{font_theme.color}",
            shadow: "{shadow}",
            border: "{border}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_align: "center",
            main_align: "center",
            cross_align: "center",
            line_height: "1.1",
            {&children}
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn button() {
        fn button_app() -> Element {
            let mut state = use_signal(|| false);

            rsx!(
                Button {
                    onpress: move |_| state.toggle(),
                    label {
                        "{state}"
                    }
                }
            )
        }

        let mut utils = launch_test(button_app);
        let root = utils.root();
        let label = root.get(0).get(0);
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("false"));

        utils.click_cursor((15.0, 15.0)).await;

        assert_eq!(label.get(0).text(), Some("true"));

        utils.push_event(PlatformEvent::Touch {
            name: EventName::TouchEnd,
            location: (15.0, 15.0).into(),
            finger_id: 1,
            phase: TouchPhase::Ended,
            force: None,
        });

        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("false"));
    }
}
