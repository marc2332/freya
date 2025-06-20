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
    ButtonSegmentTheme,
    ButtonSegmentThemeWith,
};

use crate::PressEvent;

/// Properties for the [`ButtonSegment`].
#[derive(Props, Clone, PartialEq)]
pub struct ButtonSegmentProps {
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,
}

/// Properties for the [`ButtonSegment`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BaseButtonSegmentProps {
    pub theme: Option<ButtonSegmentThemeWith>,
    /// Inner children for the button.
    pub children: Element,
    /// Event handler for when the button is pressed.
    ///
    /// This will fire upon **mouse click**, **touch** or pressing the **enter key** when it is focused.
    pub onpress: Option<EventHandler<PressEvent>>,

    #[props(default = true)]
    pub enabled: bool,
}

/// Identifies the current status of the [`ButtonSegment`s].
#[derive(Debug, Default, PartialEq, Clone, Copy)]
enum ButtonSegmentStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the button.
    Hovering,
}

#[allow(non_snake_case)]
pub fn ButtonSegment(
    BaseButtonSegmentProps {
        theme,
        onpress,
        children,
        enabled,
    }: BaseButtonSegmentProps,
) -> Element {
    let theme = use_applied_theme!(&theme, button_segment);
    let mut focus = use_focus();
    let mut status = use_signal(ButtonSegmentStatus::default);
    let platform = use_platform();

    let a11y_id = focus.attribute();

    let ButtonSegmentTheme {
        background,
        hover_background,
        disabled_background,
        padding,
        width,
        height,
        font_theme,
        shadow,
    } = theme;

    let onpointerup = move |ev: PointerEvent| {
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
    };

    use_effect(use_reactive!(|enabled| {
        if *status.peek() == ButtonSegmentStatus::Hovering && !enabled {
            platform.set_cursor(CursorIcon::default());
        }
    }));

    use_drop(move || {
        if *status.read() == ButtonSegmentStatus::Hovering && enabled {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        if enabled {
            platform.set_cursor(CursorIcon::Pointer);
            status.set(ButtonSegmentStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonSegmentStatus::default());
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
        ButtonSegmentStatus::Hovering => hover_background,
        ButtonSegmentStatus::Idle => background,
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
            overflow: "clip",
            a11y_role: "button",
            a11y_focusable,
            color: "{font_theme.color}",
            shadow: "{shadow}",
            background: "{background}",
            text_height: "disable-least-ascent",
            main_align: "center",
            cross_align: "center",
            {&children}
        }
    )
}
