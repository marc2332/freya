use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_focus, use_get_theme};

/// [`Button`] component properties.
#[derive(Props)]
pub struct ButtonProps<'a> {
    /// Inner children for the Button.
    pub children: Element<'a>,
    #[props(optional)]
    /// Handler for the `onclick` event.
    pub onclick: Option<EventHandler<'a, MouseEvent>>,
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
    let button_theme = &theme.button;
    let focus_id = focus.attribute(cx);

    let background = use_state(cx, || <&str>::clone(&button_theme.background));
    let set_background = background.setter();

    use_effect(cx, &button_theme.clone(), move |button_theme| async move {
        set_background(button_theme.background);
    });

    let onclick = move |ev: MouseEvent| {
        focus.focus();
        if let Some(onclick) = &cx.props.onclick {
            onclick.call(ev)
        }
    };

    render!(
        container {
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "2",
            focus_id: focus_id,
            role: "button",
            container {
                onclick: onclick,
                onmouseover: move |_| {
                    background.set(theme.button.hover_background);
                },
                onmouseleave: move |_| {
                    background.set(theme.button.background);
                },
                width: "auto",
                height: "auto",
                direction: "both",
                color: "{button_theme.font_theme.color}",
                shadow: "0 5 15 10 black",
                radius: "5",
                padding: "8",
                background: "{background}",
                &cx.props.children
            }
        }
    )
}
