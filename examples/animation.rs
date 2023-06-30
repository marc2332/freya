#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::events::MouseEvent;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Animation", (700.0, 250.0));
}

const TIME: i32 = 500;
const TARGET: f64 = 500.0;

fn app(cx: Scope) -> Element {
    let animation = use_animation(cx, 0.0);

    let progress = animation.value();

    let anim = move |_: MouseEvent| {
        if animation.is_animating() {
            return;
        }
        if progress == 0.0 {
            animation.start(Animation::new_sine_in_out(0.0..=TARGET, TIME));
        } else if progress == TARGET {
            animation.start(Animation::new_sine_in_out(TARGET..=0.0, TIME));
        }
    };

    render!(
        rect {
            overflow: "clip",
            background: "black",
            direction: "both",
            width: "100%",
            height: "100%",
            offset_x: "{progress}",
            rect {
                display: "center",
                direction: "both",
                height: "100%",
                width: "200",
                rect {
                    height: "200",
                    width: "100%",
                    background: "rgb(100, 100, 100)",
                    padding: "25",
                    corner_radius: "100",
                    display: "center",
                    direction: "both",
                    onclick: anim,
                    label {
                        font_size: "30",
                        align: "center",
                        color: "white",
                        "Click to move"
                    }
                }
            }
        }
    )
}
