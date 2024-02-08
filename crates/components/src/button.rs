use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{KeyboardEvent, MouseEvent};

use freya_hooks::{use_applied_theme, use_focus, use_platform, ButtonTheme, ButtonThemeWith};
use winit::window::CursorIcon;

/// [`Button`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Theme override.
    pub theme: Option<ButtonThemeWith>,
    /// Inner children for the Button.
    pub children: Element,
    /// Handler for the `onclick` event.
    pub onclick: Option<EventHandler<Option<MouseEvent>>>,
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

/// `Button` component.
///
/// # Props
/// See [`ButtonProps`].
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
///             onclick: |_| println!("clicked"),
///             label {
///                 "Click this"
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Button(props: ButtonProps) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();
    let click = &props.onclick;

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
    } = use_applied_theme!(&props.theme, button);

    let onclick = {
        to_owned![click];
        move |ev| {
            focus.focus();
            if let Some(onclick) = &click {
                onclick.call(Some(ev))
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

    let onkeydown = move |e: KeyboardEvent| {
        if focus.validate_keydown(e) {
            if let Some(onclick) = &props.onclick {
                onclick.call(None)
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
            onclick,
            onmouseenter,
            onmouseleave,
            onkeydown,
            focus_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            focusable: "true",
            overflow: "clip",
            role: "button",
            color: "{font_theme.color}",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            border: "{border}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_align: "center",
            main_align: "center",
            cross_align: "center",
            {&props.children}
        }
    )
}
