#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use winit::window::CursorIcon;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut font_size = use_state(cx, || 30f32);

    let onwheel = move |e: WheelEvent| {
        let y = e.get_delta_y();
        font_size += (y as f32) * 5.0;
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            CursorArea {
                icon: CursorIcon::Text,
                label {
                    onwheel: onwheel,
                    "text cursor"
                }
            }
            CursorArea {
                icon: CursorIcon::Progress,
                label {
                    onwheel: onwheel,
                    "Loading..."
                }
            }
        }
    )
}
