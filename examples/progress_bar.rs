#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut start_origin = use_signal(|| 50.);
    let animation = use_animation(move |ctx| {
        ctx.with(
            AnimNum::new(*start_origin.read(), 100.)
                .time(400)
                .ease(Ease::InOut)
                .function(Function::Sine),
        )
    });
    let progress = animation.read().get().read().as_f32();

    let set_to_max = move |_| {
        animation.read().start();
    };

    let onmoved = move |value: f64| {
        start_origin.set(value as f32);
    };

    rsx!(
        ProgressBar {
            show_progress: true,
            progress: progress
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.75
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.50
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.35
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.15
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.90
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.70
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.65
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.30
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.85
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.60
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.45
        }
        ProgressBar {
            show_progress: true,
            progress: progress * 0.20
        }
        Slider {
            width: "300",
            value: progress as f64,
            onmoved: onmoved
        }
        Button {
            onclick: set_to_max,
            label {
                "Set to 100%"
            }
        }
    )
}
