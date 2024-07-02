use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsDragProvider() -> Element {
    let column = vec!["I Like".to_string(), "Rust".to_string(), "🦀!".to_string()];

    rsx!(
        DragProvider::<String> {
            DropZone {
                ondrop: move |data: String| {
                    println!("drop {data}");
                },
                rect {
                    width: "100%",
                    height: "100%",
                    direction: "vertical",
                    for el in column {
                        DragZone {
                            data: el.to_string(),
                            drag_element: rsx!(
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
                    }
                }
            }
        }
    )
}
