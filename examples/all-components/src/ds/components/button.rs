use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsButton() -> Element {
    rsx!(
        Button {
            onclick: move |_| {
                println!("button clicked")
            },
            label {
                "A button"
            }
        }
    )
}

#[component]
pub fn ButtonThemeEditor(theme: Signal<Theme>) -> Element {
    rsx!(Input {
        value: theme().button.background,
        onchange: move |e: String| { theme.write().button.background = e.into() }
    })
}
