#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use rand::seq::SliceRandom;
use tokio::time::sleep;

fn main() {
    launch(app);
}

fn bar_color(value: u32) -> String {
    let ratio = (value - 5) as f32 / 95.0;
    let r = 0;
    let g = (100.0 + 155.0 * ratio) as u8;
    let b = (255.0 - 155.0 * ratio) as u8;
    format!("rgb({r},{g},{b})")
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);

    let mut bars = use_signal(|| {
        let mut v: Vec<u32> = (5..=100).step_by(1).collect(); // ~96 bars
        v.shuffle(&mut rand::thread_rng());
        v
    });

    let mut is_sorting = use_signal(|| false);

    let start_sort = move |_| {
        if *is_sorting.read() {
            return;
        }

        is_sorting.set(true);

        spawn(async move {
            let mut vec = bars.read().clone();
            let len = vec.len();

            for _ in 0..len {
                for j in 0..vec.windows(2).count() {
                    if vec[j] > vec[j + 1] {
                        vec.swap(j, j + 1);
                        bars.set(vec.clone());
                        sleep(Duration::from_micros(200)).await;
                    }
                }
            }

            is_sorting.set(false);
        });
    };

    rsx!(
        Body {
            rect {
                width: "100%",
                height: "85%",
                padding: "4",
                direction: "horizontal",
                main_align: "center",
                cross_align: "end",
                spacing: "1",

                for (i, value) in bars.read().iter().enumerate() {
                    rect {
                        key: "{i}",
                        width: "6",
                        height: "{value}%",
                        background: bar_color(*value),
                        corner_radius: "2",
                    }
                }
            }

            rect {
                width: "100%",
                height: "15%",
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",

                Button {
                    onpress: start_sort,
                    label { "Start Bubble Sort" }
                }
            }
        }
    )
}
