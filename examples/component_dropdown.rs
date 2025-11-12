use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let values = use_hook(|| {
        vec![
            "Rust".to_string(),
            "Turbofish".to_string(),
            "Crabs".to_string(),
        ]
    });
    let mut selected_dropdown = use_state(|| 0);

    rect()
        .center()
        .expanded()
        .horizontal()
        .spacing(6.)
        .child(
            Dropdown::new()
                .selected_item(values[selected_dropdown()].to_string())
                .children_iter(values.iter().enumerate().map(|(i, val)| {
                    DropdownItem::new()
                        .selected(selected_dropdown() == i)
                        .on_press(move |_| selected_dropdown.set(i))
                        .child(val.to_string())
                        .into()
                })),
        )
        .child(
            Button::new()
                .on_press(move |_| selected_dropdown.set(0))
                .child("Reset"),
        )
        .into()
}
