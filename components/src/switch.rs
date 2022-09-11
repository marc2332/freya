use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;
use fermi::use_atom_ref;

use crate::THEME;

#[derive(Props)]
pub struct SwitchProps<'a> {
    enabled: &'a bool,
    ontoggled: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
    let theme = use_atom_ref(&cx, THEME);
    let theme = theme.read();
    let hovering = use_state(&cx, || false);
    let clicking = use_state(&cx, || false);

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = |_: UiEvent<MouseData>| {
        hovering.set(true);
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
        cx.props.ontoggled.call(());
    };

    let (scroll_x, border, circle) = {
        if *cx.props.enabled {
            (
                if *clicking.get() { 20 } else { 25 },
                theme.switch.enabled_background,
                theme.switch.enabled_thumb_background,
            )
        } else {
            (
                if *clicking.get() { 5 } else { 0 },
                theme.switch.background,
                theme.switch.thumb_background,
            )
        }
    };

    cx.render(rsx!(
        container {
            width: "50",
            height: "25",
            padding: "2",
            radius: "50",
            shadow: "0 0 60 35 white",
            background: "{border}",
            onmousedown: onmousedown,
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            onclick: onclick,
            rect {
                width: "100%",
                height: "100%",
                scroll_x: "{scroll_x}",
                padding: "5",
                radius: "50",
                rect {
                    background: "{circle}",
                    direction: "both",
                    width: "18",
                    height: "18",
                    radius: "50",
                }
            }
        }
    ))
}
