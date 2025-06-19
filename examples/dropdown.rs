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
    let mut selected_dropdown = use_signal(|| 0);

    rsx!(
        Body {
            direction: "horizontal",
            padding: "4",
            spacing: "4",
            Dropdown {
                theme: theme_with!(DropdownTheme {
                    width: "200".into(),
                    arrow_fill: "rgb(0, 119, 182)".into()
                }),
                selected_item: rsx!(
                    label {
                        "Selected: {values[selected_dropdown()]}"
                    }
                ),
                for (i, ch) in values.iter().enumerate() {
                    DropdownItem {
                        onpress: move |_| selected_dropdown.set(i),
                        label { "Custom {ch}" }
                    }
                }
            }
            Dropdown {
                selected_item: rsx!(
                    label {
                        "{values[selected_dropdown()]}"
                     }
                ),
                for (i, ch) in values.iter().enumerate() {
                    DropdownItem {
                        onpress: move |_| selected_dropdown.set(i),
                        label { "Custom {ch}" }
                    }
                }
            }
        }
    )
}
