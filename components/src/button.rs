use dioxus::{events::MouseEvent, prelude::*};
use fermi::*;
use freya_elements as dioxus_elements;

use crate::THEME;

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let theme = use_atom_ref(&cx, THEME);
    let button_theme = &theme.read().button;

    let border_color = use_state(&cx, || button_theme.border_theme.color.clone());
    let set_border_color = border_color.setter();
    let background = use_state(&cx, || button_theme.background.clone());
    let set_background = background.setter();

    use_effect(&cx, &button_theme.clone(), move |button_theme| async move {
        set_background(button_theme.background);
        set_border_color(button_theme.border_theme.color);
    });

    cx.render(rsx!(
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
                    border_color.set(theme.read().button.border_theme.hover_color);
                },
                onmouseleave: move |_| {
                    background.set(theme.read().button.background);
                    border_color.set(theme.read().button.border_theme.color);
                },
                width: "auto",
                height: "auto",
                background: "{border_color}",
                padding: "3",
                radius: "7",
                direction: "both",
                color: "{button_theme.font_theme.color}",
                shadow: "0 5 15 10 black",
                rect {
                    width: "auto",
                    height: "auto",
                    radius: "5",
                    direction: "both",
                    padding: "15",
                    background: "{background}",
                    &cx.props.children
                }
            }
        }
    ))
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    pub children: Element<'a>,
    #[props(optional)]
    pub on_click: Option<EventHandler<'a, MouseEvent>>,
}
