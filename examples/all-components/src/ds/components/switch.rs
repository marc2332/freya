use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsSwitch() -> Element {
    let mut is_enabled = use_signal(|| true);
    rsx!(Switch {
        enabled: is_enabled(),
        ontoggled: move |_| { is_enabled.toggle() }
    })
}
