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

fn app(cx: Scope) -> Element {
    let font_size = use_state(&cx, || 20.0);

    cx.render(rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            padding: "20",
            label {
                font_size: "{font_size}",
                font_family: "Inter",
                height: "150",
                "Hello World"
            }
            Slider {
                onmoved: |e| {
                    font_size.set(e + 20.0); // Minimum is 20
                }
            }
        }
    ))
}

#[derive(Props)]
struct SliderProps<'a> {
    onmoved: EventHandler<'a, f64>,
}

#[allow(non_snake_case)]
fn Slider<'a>(cx: Scope<'a, SliderProps>) -> Element<'a> {
    let hovering = use_state(&cx, || false);
    let progress = use_state(&cx, || 0.0f64);
    let clicking = use_state(&cx, || false);

    let onmouseleave = |_: UiEvent<MouseData>| {
        if *clicking.get() == false {
            hovering.set(false);
        }
    };

    let onmouseover = |e: UiEvent<MouseData>| {
        hovering.set(true);
        if *clicking.get() {
            let coordinates = e.coordinates().element();
            let mut x = coordinates.x - 11.0;
            if x < 0.0 {
                x = 0.0;
            }
            if x > 80.0 {
                x = 80.0;
            }
            progress.set(x);
            cx.props.onmoved.call(x);
        }
    };

    let onmousedown = |_: UiEvent<MouseData>| {
        clicking.set(true);
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(false);
    };

    cx.render(rsx!(
        container {
            background: "white",
            width: "100",
            height: "20",
            scroll_x: "{progress}",
            padding: "5",
            onmousedown: onmousedown,
            onclick: onclick,
            shadow: "0 0 60 35 white",
            radius: "25",
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            rect {
                background: "rgb(255, 166, 0)",
                width: "15",
                height: "15",
                radius: "15",
            }
        }

    ))
}
