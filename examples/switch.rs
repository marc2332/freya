#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;

use freya::launch;

fn main() {
    launch(app);
}

#[derive(Props)]
struct SwitchProps<'a> {
    enabled: &'a bool,
    ontoggled: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
fn Switch<'a>(cx: Scope<'a, SwitchProps<'a>>) -> Element<'a> {
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
                "rgb(103, 80, 164)",
                "rgb(234, 221, 255)",
            )
        } else {
            (
                if *clicking.get() { 5 } else { 0 },
                "rgb(121, 116, 126)",
                "rgb(231, 224, 236)",
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

fn app(cx: Scope) -> Element {
    let enabled = use_state(&cx, || false);

    let is_enabled = if *enabled.get() { "Yes" } else { "No" };

    cx.render(rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            label {
                color: "black",
                "Is enabled? {is_enabled}"
            }
            Switch {
                enabled: enabled,
                ontoggled: |_| {
                    enabled.set(!enabled.get());
                }
            }
        }
    ))
}
