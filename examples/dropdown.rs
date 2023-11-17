#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let values = cx.use_hook(|| {
        vec![
            "Value A".to_string(),
            "Value B".to_string(),
            "Value C".to_string(),
        ]
    });
    let selected_dropdown = use_state(cx, || "Value A".to_string());

    render!(
        Dropdown {
            value: selected_dropdown.get().clone(),
            values.iter().map(|ch| {
                rsx!(
                    DropdownItem {
                        value: ch.to_string(),
                        onclick: move |_| selected_dropdown.set(ch.to_string()),
                        label { "{ch}" }
                    }
                )
            })
        }
        Button {
            onclick: |_| selected_dropdown.set("Value A".to_string()),
            label {
                "Reset"
            }
        }
        Dropdown {
            value: selected_dropdown.get().clone(),
            values.iter().map(|ch| {
                rsx!(
                    DropdownItem {
                        value: ch.to_string(),
                        onclick: move |_| selected_dropdown.set(ch.to_string()),
                        label { "{ch}" }
                    }
                )
            })
        }
    )
}
