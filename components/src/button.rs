use dioxus::{events::MouseEvent, prelude::*};
use fermi::*;
use freya_elements as dioxus_elements;

use crate::THEME;

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let theme = use_atom_ref(&cx, THEME);
    let button_theme = &theme.read().button;

    let background = use_state(&cx, || button_theme.background.clone());
    let set_background = background.setter();

    use_effect(&cx, &button_theme.clone(), move |button_theme| async move {
        set_background(button_theme.background);
    });

    render!(
        container {
            width: "auto",
            height: "auto",
            direction: "both",
            padding: "3",
            container {
                onclick: move |ev| {
                    if let Some(on_click) = &cx.props.on_click {
                        on_click.call(ev)
                    }
                },
                onmouseover: move |_| {
                    background.set(theme.read().button.hover_background);
                },
                onmouseleave: move |_| {
                    background.set(theme.read().button.background);
                },
                width: "auto",
                height: "auto",
                direction: "both",
                color: "{button_theme.font_theme.color}",
                shadow: "0 5 15 10 black",
                radius: "5",
                padding: "17",
                background: "{background}",
                &cx.props.children
            }
        }
    )
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    pub children: Element<'a>,
    #[props(optional)]
    pub on_click: Option<EventHandler<'a, MouseEvent>>,
}
