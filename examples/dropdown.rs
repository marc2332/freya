#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let selected_dropdown = use_state(cx, || "A".to_string());

    use_init_focus(cx);
    render!(
        AccessibilityFocusBridge {},
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
