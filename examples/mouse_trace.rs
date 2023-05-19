#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        WindowConfig::<()>::builder()
            .with_width(1920.0)
            .with_height(1080.0)
            .with_decorations(false)
            .with_transparency(true)
            .with_title("Rotate")
            .build(),
    );
}

const MOVEMENT_MARGIN: f64 = 75.0;
const BOX_COUNT: usize = 80;

#[allow(non_snake_case)]
fn Box(cx: Scope) -> Element {
    render!(
        rect {
            background: "rgb(65, 53, 67)",
            width: "250",
            height: "250",
            direction: "both",
            display: "center",
            radius: "100",
            rect {
                background: "rgb(143, 67, 238)",
                width: "180",
                height: "180",
                direction: "both",
                display: "center",
                radius: "100",
                rect {
                    background: "rgb(240, 235, 141)",
                    width: "100",
                    height: "100",
                    radius: "100",
                }
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    let positions = use_state::<Vec<CursorPoint>>(cx, Vec::new);

    let onmouseover = |e: MouseEvent| {
        let coordinates = e.get_screen_coordinates();
        positions.with_mut(|positions| {
            if let Some(pos) = positions.first() {
                if (pos.x + MOVEMENT_MARGIN < coordinates.x
                    && pos.x - MOVEMENT_MARGIN > coordinates.x)
                    && (pos.y + MOVEMENT_MARGIN < coordinates.y
                        && pos.y - MOVEMENT_MARGIN > coordinates.y)
                {
                    return;
                }
            }
            positions.insert(0, (coordinates.x - 125.0, coordinates.y - 125.0).into());
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
                    scroll_x: "{pos.x}",
                    scroll_y: "{pos.y}",
                    Box {}
                }
            ))
        }
    )
}
