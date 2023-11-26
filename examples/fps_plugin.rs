#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::events::MouseEvent;
use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(500.)
            .with_height(170.)
            .with_plugin(FpsPlugin::default())
            .build(),
    )
}

const TIME: i32 = 500;
const TARGET: f64 = 350.0;

fn app(cx: Scope) -> Element {
    let animation = use_animation(cx, || 0.0);

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
            width: "100%",
            height: "100%",
            offset_x: "{progress}",
            main_align: "center",
            rect {
                height: "150",
                width: "150",
                background: "rgb(7, 102, 173)",
                corner_radius: "100",
                onclick: anim,
            }
        }
    )
}
