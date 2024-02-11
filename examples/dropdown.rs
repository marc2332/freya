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
            "Value A".to_string(),
            "Value B".to_string(),
            "Value C".to_string(),
        ]
    });
    let mut selected_dropdown = use_signal(|| "Value A".to_string());

    rsx!(
        Dropdown {
            value: selected_dropdown.read().clone(),
            for ch in values {
                DropdownItem {
                    value: ch.clone(),
                    onclick: {
                        to_owned![ch];
                        move |_| selected_dropdown.set(ch.clone())
                    },
                    label { "{ch}" }
                }
            }
        }
    )
}
