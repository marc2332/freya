#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let progress = use_state(cx, || 0f64);

    render!(
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.75
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.50
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.35
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.15
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.90
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.70
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.65
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.30
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.85
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.60
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.45
        }
        ProgressBar {
            show_progress: true,
            progress: *progress.get() as f32 * 0.20
        }
        Slider {
            width: 300.0,
            value: *progress.get(),
            onmoved: |v| {
                progress.set(v)
            }
        }
    )
}
