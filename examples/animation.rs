#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::events::MouseEvent;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Animation", (700.0, 400.0));
}

const TARGET: f64 = 500.0;

fn app(cx: Scope) -> Element {
    let animation =
        use_animation_transition(cx, TransitionAnimation::new_sine_in_out(700), (), |_| {
            vec![
                Transition::new_size(0.0, TARGET),
                Transition::new_color("rgb(33, 158, 188)", "white"),
                Transition::new_color("rgb(30, 15, 25)", "orange"),
            ]
        });

    let size = animation.get(0).unwrap().as_size();
    let background = animation.get(1).unwrap().as_color();
    let text = animation.get(2).unwrap().as_color();

    let anim = move |_: MouseEvent| {
        if size == 0.0 {
            animation.start();
        } else if size == TARGET {
            animation.reverse();
        }
    };

    render!(
        container {
            background: "black",
            display: "center",
            width: "100%",
            height: "100%",
            scroll_x: "{size}",
            rect {
                height: "200",
                width: "200",
                background: "{background}",
                padding: "25",
                radius: "100",
                display: "center",
                onclick: anim,
                label {
                    width: "100%",
                    font_size: "30",
                    align: "center",
                    color: "{text}",
                    "Click to move"
                }
            }
        }
    )
}
