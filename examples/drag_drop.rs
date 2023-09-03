#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[derive(PartialEq, Eq)]
enum SwapDirection {
    LeftToRight,
    RightToLeft,
}

fn app(cx: Scope) -> Element {
    let data = use_state::<(Vec<String>, Vec<String>)>(cx, || {
        (
            vec!["I Like".to_string(), "Rust".to_string(), "🦀!".to_string()],
            vec![],
        )
    });

    render!(
        DragProvider::<String> {
            rect {
                direction: "horizontal",
                width: "100%",
                height: "100%",
                Column {
                    data: data.clone(),
                    direction: SwapDirection::RightToLeft,
                    column: data.get().0.clone()
                }
                Column {
                    data: data.clone(),
                    direction: SwapDirection::LeftToRight,
                    column: data.get().1.clone()
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Column(
    cx: Scope,
    direction: SwapDirection,
    data: UseState<(Vec<String>, Vec<String>)>,
    column: Vec<String>,
) -> Element {
    let swap = |el: String, direction: &SwapDirection| {
        data.with_mut(|data| {
            data.0.retain(|e| e != &el);
            data.1.retain(|e| e != &el);
            match direction {
                SwapDirection::LeftToRight => {
                    data.1.push(el);
                }
                SwapDirection::RightToLeft => {
                    data.0.push(el);
                }
            }
        });
    };

    let (color, background) = match direction {
        SwapDirection::LeftToRight => ("white", "rgb(0, 48, 73)"),
        SwapDirection::RightToLeft => ("black", "rgb(234, 226, 183)"),
    };

    render!(
        rect {
            width: "50%",
            height: "100%",
            DropZone {
                ondrop: move |data: String| {
                    swap(data, direction);
                },
                rect {
                    width: "100%",
                    height: "100%",
                    background: background,
                    direction: "vertical",
                    color: color,
                    for el in column {
                        rsx!(
                            DragZone {
                                data: el.to_string(),
                                drag_element: render!(
                                    label {
                                        width: "200",
                                        font_size: "20",
                                       "Moving '{el}'"
                                    }
                                ),
                                label {
                                    font_size: "30",
                                    "{el}"
                                }
                            }
                        )
                    }
                }
            }
        }
    )
}
