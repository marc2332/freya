#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "SnackBar", (400.0, 350.0));
}

fn app() -> Element {
    let animation = use_animation(move |ctx| {
        ctx.with(
            AnimNum::new(0., 100.)
                .time(1650)
                .ease(Ease::Out)
                .function(Function::Sine),
        )
    });
    let progress = animation.get().read().as_f32();
    let mut show = use_signal(|| false);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onpress: move |_| {
                    show.toggle();
                    if *show.read() {
                        animation.start();
                    } else {
                        animation.reset();
                    }
                },
                label { "Install" }
            }
            SnackBar {
                show,
                ProgressBar {
                    show_progress: true,
                    progress: progress
                }
            }
        }
    )
}
