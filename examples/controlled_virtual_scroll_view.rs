#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

const ITEM_SIZE: i32 = 30;

fn app(cx: Scope) -> Element {
    let data = cx.use_hook(|| {
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29,
        ]
    });
    let scroll_y = use_state(cx, || 0);
    render!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "25",
            width: "100%",
            height: "100%",
            container {
                height: "50",
                width: "100%",
                Button {
                    onclick: move |_| {
                        scroll_y.set(0);
                    },
                    label {
                        "Scroll to top"
                    }
                }
            }
            ControlledVirtualScrollView {
                width: "100%",
                height: "calc(100% - 50)",
                show_scrollbar: true,
                length: data.len() as i32,
                item_size: ITEM_SIZE as f32,
                builder_values: data,
                onscroll: |(axis, value)| {
                    if let Axis::Y = axis {
                        scroll_y.set(value)
                    }
                },
                scroll_x: 0,
                scroll_y: *scroll_y.get(),
                builder: Box::new(move |(key, index, data)| {
                    let val = data.as_ref().unwrap()[index as usize];
                    rsx! {
                        rect {
                            key: "{key}",
                            height: "{ITEM_SIZE}",
                            width: "100%",
                            direction: "horizontal",
                            label {
                                "{val}"
                            }
                        }
                    }
                })
            }
        }
    )
}
