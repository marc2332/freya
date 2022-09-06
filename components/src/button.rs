use dioxus::{events::MouseEvent, prelude::*};
use elements_namespace as dioxus_elements;
use fermi::*;

use crate::THEME;

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let theme = use_atom_ref(&cx, THEME);

    let background = use_state(&cx, || theme.read().button.background.clone());
    let set_background = background.setter();

    use_effect(&cx, &theme.read().clone(), move |theme| async move {
        set_background(theme.button.background);
    });

    cx.render(rsx!(
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
            background: "{background}",
            padding: "20",
            radius: "5",
            direction: "both",
            &cx.props.child
        }
    ))
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    child: Element<'a>,
    #[props(optional)]
    on_click: Option<EventHandler<'a, MouseEvent>>,
}
