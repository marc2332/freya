use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsInput() -> Element {
    let mut value = use_signal(String::new);

    rsx!(
        label {
            "Value: {value}"
        }
        Input {
            value: value.read().clone(),
            onchange: move |e| {
                 value.set(e)
            }
        }
    )
}
