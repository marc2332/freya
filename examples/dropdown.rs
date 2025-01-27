#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let values = use_hook(|| {
        vec![
            "First Option".to_string(),
            "Second Option".to_string(),
            "Rust".to_string(),
        ]
    });
    let mut selected_dropdown = use_signal(|| "First Option".to_string());

    rsx!(
        rect {
            direction: "horizontal",
            Dropdown {
                theme: theme_with!(DropdownTheme {
                    width: "200".into(),
                    arrow_fill: "rgb(0, 119, 182)".into()
                }),
                value: selected_dropdown.read().clone(),
                for ch in values.iter() {
                    DropdownItem {
                        value: ch.clone(),
                        onpress: {
                            to_owned![ch];
                            move |_| selected_dropdown.set(ch.clone())
                        },
                        label { "Custom {ch}" }
                    }
                }
            }
            Dropdown {
                value: selected_dropdown.read().clone(),
                for ch in values {
                    DropdownItem {
                        value: ch.clone(),
                        onpress: {
                            to_owned![ch];
                            move |_| selected_dropdown.set(ch.clone())
                        },
                        label { "{ch}" }
                    }
                }
            }
        }
    )
}