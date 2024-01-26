#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let progress_anim = use_animation(cx, || 0.0);
    let progress = progress_anim.value() as f32;

    let set_to_max = {
        to_owned![progress_anim];
        move |_| {
            progress_anim.start(Animation::new_linear(progress_anim.value()..=100.0, 400));
        }
    };

    let onmoved = move |value: f64| {
        progress_anim.set_value(value);
    };

    render!(
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
