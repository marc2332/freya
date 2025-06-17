use dioxus::prelude::*;
use freya_core::platform::CursorIcon;
use freya_elements::{
    self as dioxus_elements,
    events::{
        KeyboardEvent,
        PointerEvent,
        PointerType,
    },
    MouseButton,
    TouchPhase,
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    use_platform,
    ChipTheme,
    ChipThemeWith,
};

use crate::PressEvent;

/// Properties for the [`Chip`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ChipProps {
    /// Theme override.
    pub theme: Option<ChipThemeWith>,
    /// Inner children for the chip.
    pub children: Element,
    /// Event handler for when the chip is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,
}

#[allow(non_snake_case)]
pub fn Chip(props: ChipProps) -> Element {
    let theme = use_applied_theme!(&props.theme, chip);
    ChipBase(BaseChipProps {
        theme,
        children: props.children,
        onpress: props.onpress,
        enabled: props.enabled,
    })
}

/// Properties for the [`Chip`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BaseChipProps {
    /// Theme.
    pub theme: ChipTheme,
    /// Inner children for the chip.
    pub children: Element,
    /// Event handler for when the chip is pressed.
    ///
    /// This will fire upon **mouse click** or pressing the **enter key**.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,
}

/// Identifies the current status of the Chip.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ChipStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the chip.
    Hovering,
}

#[allow(non_snake_case)]
pub fn ChipBase(
    BaseChipProps {
        onpress,
        children,
        theme,
        enabled,
    }: BaseChipProps,
) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ChipStatus::default);
    let platform = use_platform();

    let a11y_id = focus.attribute();

    let ChipTheme {
        background,
        hover_background,
        disabled_background,
        border_fill,
        focus_border_fill,
        padding,
        margin,
        corner_radius,
        width,
        height,
        font_theme,
        shadow,
    } = theme;

    let onpointerup = {
        move |ev: PointerEvent| {
            if !enabled {
                return;
            }
            focus.request_focus();
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
            }
        }
    };

    use_effect(use_reactive!(|enabled| {
        if *status.peek() == ChipStatus::Hovering && !enabled {
            platform.set_cursor(CursorIcon::default());
        }
    }));

    use_drop(move || {
        if *status.read() == ChipStatus::Hovering && enabled {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        if enabled {
            platform.set_cursor(CursorIcon::Pointer);
            status.set(ChipStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ChipStatus::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if focus.validate_keydown(&ev) && !enabled {
            if let Some(onpress) = &onpress {
                onpress.call(PressEvent::Key(ev))
            }
        }
    };

    let a11y_focusable = if enabled { "true" } else { "false" };
    let background = match *status.read() {
        _ if !enabled => disabled_background,
        ChipStatus::Hovering => hover_background,
        ChipStatus::Idle => background,
    };
    let border = if focus.is_focused_with_keyboard() {
        format!("2 inner {focus_border_fill}")
    } else {
        format!("1 inner {border_fill}")
    };

    rsx!(
        rect {
            onpointerup,
            onmouseenter,
            onmouseleave,
            onkeydown,
            a11y_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            overflow: "clip",
            a11y_role: "button",
            a11y_focusable,
            color: "{font_theme.color}",
            shadow: "{shadow}",
            border,
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_height: "disable-least-ascent",
            main_align: "center",
            cross_align: "center",
            {&children}
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn chip() {
        fn chip_app() -> Element {
            let mut state = use_signal(|| false);

            rsx!(
                Chip {
                    onpress: move |_| state.toggle(),
                    label {
                        "{state}"
                    }
                }
            )
        }

        let mut utils = launch_test(chip_app);
        let root = utils.root();
        let label = root.get(0).get(0);
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("false"));

        utils.click_cursor((15.0, 15.0)).await;

        assert_eq!(label.get(0).text(), Some("true"));

        utils.push_event(TestEvent::Touch {
            name: EventName::TouchStart,
            location: (15.0, 15.0).into(),
            finger_id: 1,
            phase: TouchPhase::Started,
            force: None,
        });
        utils.wait_for_update().await;

        utils.push_event(TestEvent::Touch {
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
