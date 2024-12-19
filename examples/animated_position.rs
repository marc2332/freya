#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use rand::Rng;

fn main() {
    launch_with_props(app, "Animation position", (800.0, 700.0));
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let mut elements = use_signal(Vec::new);
    let mut direction = use_signal(|| "vertical".to_string());
    let mut function = use_signal(|| Function::Quad);

    let add = move |_| {
        let mut rng = rand::thread_rng();
        elements.write().insert(0, rng.gen());
    };

    let remove = move |_| {
        elements.write().remove(0);
    };

    let toggle = move |_| {
        if &*direction.read() == "vertical" {
            direction.set("horizontal".to_string());
        } else {
            direction.set("vertical".to_string());
        }
    };

    rsx!(
        rect {
            cross_align: "center",
            width: "100%",
            height: "100%",
            spacing: "4",
            padding: "4",
            background: "rgb(20, 20, 20)",
            rect {
                direction: "horizontal",
                main_align: "center",
                width: "100%",
                spacing: "4",
                Button {
                    onpress: add,
                    label {
                        "Add"
                    }
                }
                Button {
                    onpress: remove,
                    label {
                        "Remove"
                    }
                }
                Button {
                    onpress: toggle,
                    label {
                        "Toggle"
                    }
                }
                Dropdown {
                    value: function(),
                    for func in &[Function::Quad, Function::Elastic, Function::Quart, Function::Linear, Function::Circ] {
                        DropdownItem {
                            value: func.clone(),
                            onclick: {
                                to_owned![func];
                                move |_| function.set(func.clone())
                            },
                            label { "{func:?}" }
                        }
                    }
                }
            }
            rect {
                direction: "{direction}",
                spacing: "4",
                main_align: "center",
                cross_align: "center",
                height: "100%",
                width: "100%",
                {elements.read().iter().map(|e: &usize| rsx!(
                    AnimatedPosition {
                        key: "{e}",
                        width: "110",
                        height: "60",
                        function: function(),
                        duration: match function() {
                            Function::Elastic => Duration::from_millis(1100),
                            _ => Duration::from_millis(250),
                        },
                        rect {
                            width: "100%",
                            height: "100%",
                            background: "rgb(240, 200, 50)",
                            corner_radius: "999",
                            padding: "6 10",
                            main_align: "center",
                            cross_align: "center",
                            label {
                                font_size: "14",
                                color: "black",
                                "{e}"
                            }
                        }
                    }
                ))}
            }
        }
    )
}
