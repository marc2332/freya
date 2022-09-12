#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use elements_namespace as dioxus_elements;

use freya::launch;
use hooks::{use_animation, AnimationMode};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let (start, restart, progress) =
        use_animation(&cx, || AnimationMode::new_sine_in_out(0.0..=700.0, 1000));

    let start_animation = move |_: UiEvent<MouseData>| {
        start();
    };

    let restart_animation = move |_: UiEvent<MouseData>| {
        restart();
    };

    cx.render(rsx!(
        rect {
            background: "black",
            direction: "both",
            width: "100%",
            height: "100%",
            scroll_x: "{progress}",
            rect {
                height: "100%",
                width: "200",
                background: "blue",
                rect {
                    padding: "20",
                    height: "50",
                    width: "100%",
                    background: "green",
                    onclick: start_animation,
                    label {
                        "Start"
                    }
                }
                rect {
                    padding: "20",
                    height: "50",
                    width: "100%",
                    background: "red",
                    onclick: restart_animation,
                    label {
                        "Restart"
                    }
                }
            }
        }
    ))
}
