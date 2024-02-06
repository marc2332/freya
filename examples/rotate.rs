#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::time::interval;

fn main() {
    launch_with_props(app, "Rotate", (350.0, 350.0));
}

const SPEEDS: (i16, i16, i16) = (2, 3, 6);

fn app() -> Element {
    let mut degrees = use_signal(|| (0, 0, 0));

    use_hook(|| {
        spawn(async move {
            let mut ticker = interval(Duration::from_millis(25));
            loop {
                ticker.tick().await;

                degrees.with_mut(|(a, b, c)| {
                    if *a == 360 {
                        *a = 0;
                    }
                    if *b == 0 {
                        *b = 360;
                    }
                    if *c == 360 {
                        *c = 0;
                    }
                });

                degrees.with_mut(|(a, b, c)| {
                    *a += SPEEDS.0.clamp(0, 360);
                    *b += SPEEDS.1.clamp(0, 360);
                    *c += SPEEDS.2.clamp(0, 360);
                });
            }
        });
    });

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "100%",
            rect {
                rotate: "{degrees.read().0}deg",
                background: "rgb(65, 53, 67)",
                width: "250",
                height: "250",
                main_align: "center",
                cross_align: "center",
                rect {
                    rotate: "{degrees.read().1}deg",
                    background: "rgb(143, 67, 238)",
                    width: "180",
                    height: "180",
                    main_align: "center",
                    cross_align: "center",
                    rect {
                        rotate: "{degrees.read().2}deg",
                        background: "rgb(240, 235, 141)",
                        width: "100",
                        height: "100"
                    }
                }
            }
        }
    )
}
