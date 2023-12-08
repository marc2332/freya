#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("Performance Overlay Plugin")
            .with_width(700.)
            .with_height(500.)
            .with_plugin(PerformanceOverlayPlugin::default())
            .build(),
    )
}

const TIME: i32 = 400;
const TARGET: f64 = 650.0;

fn app(cx: Scope) -> Element {
    let animation = use_animation(cx, || 15.0);

    let progress = animation.value();

    if !animation.is_animating() {
        if progress == 15.0 {
            animation.start(Animation::new_sine_in_out(15.0..=TARGET, TIME));
        } else if progress == TARGET {
            animation.start(Animation::new_sine_in_out(TARGET..=15.0, TIME));
        }
    }

    render!(
        rect { width: "100%", height: "100%", main_align: "center",
            for i in 0..32 {
                rsx!(
                    rect {
                        offset_x: "{progress - i as f64 * 10.0}",
                        key: "{i}",
                        direction: "horizontal",
                        rect {
                            height: "25",
                            width: "45",
                            background: "rgb(7, 102, 173)",
                            corner_radius: "100",
                        }
                        rect {
                            height: "25",
                            width: "45",
                            background: "rgb(166, 207, 152)",
                            corner_radius: "100",
                        }
                        rect {
                            height: "25",
                            width: "45",
                            background: "rgb(179, 19, 18)",
                            corner_radius: "100",
                        }
                        rect {
                            height: "25",
                            width: "45",
                            background: "rgb(255, 108, 34)",
                            corner_radius: "100",
                        }
                    }
                )
            }
        }
    )
}
