#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::time::interval;

fn main() {
    launch_with_props(app, "Rotate", (350, 350));
}

const SPEEDS: (i16, i16, i16) = (2, 3, 6);

fn app(cx: Scope) -> Element {
    let degrees = use_state(cx, || (0, 0, 0));

    use_effect(cx, (), move |_| {
        to_owned![degrees];
        async move {
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

                degrees.modify(|(a, b, c)| {
                    (
                        (a + SPEEDS.0).clamp(0, 360),
                        (b - SPEEDS.1).clamp(0, 360),
                        (c + SPEEDS.2).clamp(0, 360),
                    )
                });
            }
        }
    });

    render!(
        rect {
            direction: "both",
            display: "center",
            width: "100%",
            height: "100%",
            rect {
                rotate: "{degrees.0}",
                background: "rgb(65, 53, 67)",
                padding: "25",
                width: "250",
                height: "250",
                direction: "both",
                display: "center",
                rect {
                    rotate: "{degrees.1}",
                    background: "rgb(143, 67, 238)",
                    padding: "25",
                    width: "180",
                    height: "180",
                    direction: "both",
                    display: "center",
                    rect {
                        rotate: "{degrees.2}",
                        background: "rgb(240, 235, 141)",
                        padding: "25",
                        width: "100",
                        height: "100"
                    }
                }
            }
        }
    )
}
