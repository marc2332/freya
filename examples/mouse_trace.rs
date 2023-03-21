#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::time::interval;

fn main() {
    launch_cfg(
        app,
        WindowConfig::<()>::builder()
            .with_width(1920)
            .with_height(1080)
            .with_decorations(false)
            .with_transparency(true)
            .with_title("Rotate")
            .build(),
    );
}

const MOVEMENT_MARGIN: f64 = 150.0;
const BOX_COUNT: usize = 15;
const SPEEDS: (i16, i16, i16) = (2, 3, 6);

#[allow(non_snake_case)]
fn Box(cx: Scope) -> Element {
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
            rotate: "{degrees.0}",
            background: "rgb(65, 53, 67)",
            width: "250",
            height: "250",
            direction: "both",
            display: "center",
            radius: "10",
            rect {
                rotate: "{degrees.1}",
                background: "rgb(143, 67, 238)",
                width: "180",
                height: "180",
                direction: "both",
                display: "center",
                radius: "10",
                rect {
                    rotate: "{degrees.2}",
                    background: "rgb(240, 235, 141)",
                    width: "100",
                    height: "100",
                    radius: "10",
                }
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    let positions = use_state(cx, Vec::new);

    let onmouseover = |e: MouseEvent| {
        let coordinates = e.get_screen_coordinates();
        positions.with_mut(|positions| {
            if let Some((x, y)) = positions.first() {
                if (*x + MOVEMENT_MARGIN < coordinates.x && *x - MOVEMENT_MARGIN > coordinates.x)
                    && (*y + MOVEMENT_MARGIN < coordinates.y
                        && *y - MOVEMENT_MARGIN > coordinates.y)
                {
                    return;
                }
            }
            positions.insert(0, (coordinates.x - 125.0, coordinates.y - 125.0));
            positions.truncate(BOX_COUNT);
        })
    };

    render!(
        rect {
            onmouseover: onmouseover,
            width: "100%",
            height: "100%",
            positions.get().iter().map(|pos| rsx!(
                rect {
                    width: "0",
                    height: "0",
                    scroll_x: "{pos.0}",
                    scroll_y: "{pos.1}",
                    Box {}
                }
            ))
        }
    )
}
