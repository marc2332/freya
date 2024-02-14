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

const TARGET: f32 = 650.0;

fn app() -> Element {
    let animation = use_animation(|ctx| {
        ctx.with(
            AnimNum::new(15., TARGET)
                .time(400)
                .ease(Ease::InOut)
                .function(Function::Sine),
        )
    });

    let progress = animation.read().get().read().as_f32();

    if !animation.read().is_running() {
        if progress == 15.0 {
            animation.read().start();
        } else if progress == TARGET {
            animation.read().reverse();
        }
    }

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            for i in 0..32 {
                rect {
                    offset_x: "{progress - i as f32 * 10.0}",
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
            }
        }
    )
}
