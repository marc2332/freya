#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let data = use_state(cx, || vec![
        vec!["test".to_string(), "test".to_string(),],
        vec!["test".to_string(), "aaaaaaaa".to_string(),],
        vec!["dddddddddddddd".to_string(), "bbbbbb".to_string(),],
        vec!["test".to_string(), "ccccc".to_string(),],
        vec!["Woah".to_string(), "2222222".to_string(),],
        vec!["Nice".to_string(), "hola".to_string(),],
        vec!["Rust".to_string(), "hello".to_string(),],
        vec!["is".to_string(), "hi!".to_string(),],
        vec!["really nice!".to_string(), "test".to_string(),],
    ]);

    render!(
        rect {
            padding: "10",
            Table {
                columns: 2,
                TableHead {
                    TableRow {
                        TableCell {
                            label {
                                width: "100%",
                                align: "center",
                                "Name"
                            }
                        }
                        TableCell {
                            label {
                                width: "100%",
                                align: "center",
                                "Other name"
                            }
                        }
                    }
                }
                TableBody {
                    ScrollView {
                        for (i, items) in data.iter().enumerate() {
                            TableRow {
                                key: "{i}",
                                for item in items.iter(){
                                    TableCell {
                                        key: "{item}",
                                        label {
                                            width: "100%",
                                            align: "right",
                                            "{item}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
