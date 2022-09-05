use dioxus::{events::MouseEvent, prelude::*};
use elements_namespace as dioxus_elements;

use crate::{ButtonTheme, Theme};

const BACKGROUND_COLOR: &str = "black";
const HOVER_BACKGROUND_COLOR: &str = "gray";

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let theme = cx.consume_context::<Theme>();

    let button_theme = if let Some(ref theme) = theme {
        theme.button.clone()
    } else {
        ButtonTheme {
            background: BACKGROUND_COLOR,
            hover_background: HOVER_BACKGROUND_COLOR,
        }
    };

    let background = use_state(&cx, || button_theme.background);
    let set_background = background.setter();

    use_effect(&cx, &theme, move |theme| async move {
        if let Some(ref theme) = theme {
            set_background(theme.button.background);
        }
    });

    cx.render(rsx!(
        container {
            onclick: move |ev| {
                if let Some(on_click) = &cx.props.on_click {
                    on_click.call(ev)
                }
            },
            onmouseover: move |_| {
                background.set(button_theme.hover_background);
            },
            onmouseleave: move |_| {
                background.set(button_theme.background);
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
