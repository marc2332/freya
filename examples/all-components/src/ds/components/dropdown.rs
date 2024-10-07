use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsDropdown() -> Element {
    let values = use_hook(|| {
        vec![
            "First Option".to_string(),
            "Second Option".to_string(),
            "Rust".to_string(),
        ]
    });
    let mut selected_dropdown = use_signal(|| "First Option".to_string());

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
