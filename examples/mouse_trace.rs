#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("Mouse Trace")
            .build(),
    );
}

const MOVEMENT_MARGIN: f64 = 75.0;
const BOX_COUNT: usize = 80;

#[allow(non_snake_case)]
fn Box() -> Element {
    rsx!(
        rect {
            background: "rgb(65, 53, 67)",
            width: "250",
            height: "250",
            main_align: "center",
            cross_align: "center",
            corner_radius: "100",
            rect {
                background: "rgb(143, 67, 238)",
                width: "180",
                height: "180",
                main_align: "center",
                cross_align: "center",
                corner_radius: "100",
                rect {
                    background: "rgb(240, 235, 141)",
                    width: "100",
                    height: "100",
                    corner_radius: "100",
                }
            }
        }
    )
}

fn app() -> Element {
    let mut positions = use_signal::<Vec<CursorPoint>>(Vec::new);

    let onmouseover = move |e: MouseEvent| {
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

    rsx!(
        rect {
            onmouseover,
            width: "100%",
            height: "100%",
            {positions.read().iter().map(|pos| rsx!(
                rect {
                    width: "0",
                    height: "0",
                    offset_x: "{pos.x}",
                    offset_y: "{pos.y}",
                    Box {}
                }
            ))}
        }
    )
}
