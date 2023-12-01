use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_focus, use_get_theme, use_platform};
use winit::window::CursorIcon;

/// [`Button`] component properties.
#[derive(Props)]
pub struct ButtonProps<'a> {
    /// Padding for the Button.
    #[props(default = "8 16".to_string(), into)]
    pub padding: String,
    /// Margin for the Button.
    #[props(default = "4".to_string(), into)]
    pub margin: String,
    /// Corner radius for the Button.
    #[props(default = "8".to_string(), into)]
    pub corner_radius: String,
    /// Width size for the Button.
    #[props(default = "auto".to_string(), into)]
    pub width: String,
    /// Inner children for the Button.
    #[props(default = "auto".to_string(), into)]
    pub height: String,
    /// Inner children for the Button.
    pub children: Element<'a>,
    /// Handler for the `onclick` event.
    #[props(optional)]
    pub onclick: Option<EventHandler<'a, MouseEvent>>,
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
/// fn app(cx: Scope) -> Element {
///     render!(
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
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let focus = use_focus(cx);
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);
    let platform = use_platform(cx);

    let focus_id = focus.attribute(cx);

    let onclick = move |ev| {
        focus.focus();
        if let Some(onclick) = &cx.props.onclick {
            onclick.call(ev)
        }
    };

    use_on_unmount(cx, {
        to_owned![status, platform];
        move || {
            if *status.current() == ButtonStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onmouseenter = {
        to_owned![status, platform];
        move |_| {
            platform.set_cursor(CursorIcon::Hand);
            status.set(ButtonStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let background = match *status.get() {
        ButtonStatus::Hovering => theme.button.hover_background,
        ButtonStatus::Idle => theme.button.background,
    };
    let color = theme.button.font_theme.color;
    let border_fill = theme.button.border_fill;
    let ButtonProps {
        width,
        height,
        corner_radius,
        padding,
        margin,
        ..
    } = &cx.props;

    render!(
        rect {
            onclick: onclick,
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            focus_id: focus_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            focusable: "true",
            overflow: "clip",
            role: "button",
            color: "{color}",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            border: "1 solid {border_fill}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            align: "center",
            main_align: "center",
            cross_align: "center",
            &cx.props.children
        }
    )
}
