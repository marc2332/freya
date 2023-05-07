use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::use_get_theme;

/// [`Button`] component properties.
#[derive(Props)]
pub struct ButtonProps<'a> {
    /// Inner children for the Button.
    pub children: Element<'a>,
    /// Handler for the `onclick` event.
    #[props(optional)]
    pub onclick: Option<EventHandler<'a, MouseEvent>>,
}

/// Identifies the current status of the Button.
#[derive(Debug, Default)]
enum ButtonStatus {
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
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);

    let onclick = move |ev| {
        if let Some(onclick) = &cx.props.onclick {
            onclick.call(ev)
        }
    };

    let onmouseover = move |_| {
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.get() {
        ButtonStatus::Hovering => theme.button.hover_background,
        ButtonStatus::Idle => theme.button.background,
    };
    let color = theme.button.font_theme.color;

    render!(
        container {
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "2",
            container {
                onclick: onclick,
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                width: "auto",
                height: "auto",
                direction: "both",
                color: "{color}",
                shadow: "0 5 15 10 black",
                radius: "5",
                padding: "8",
                background: "{background}",
                &cx.props.children
            }
        }
    )
}
