use dioxus::prelude::*;
use freya_core::accessibility::AccessibilityFocusStrategy;
use freya_elements::{
    self as dioxus_elements,
    events::{
        Key,
        KeyboardEvent,
        Modifiers,
    },
};
use freya_hooks::{
    use_init_native_platform,
    use_platform,
};

#[allow(non_snake_case)]
#[component]
pub fn NativeContainer(children: Element) -> Element {
    use_init_native_platform();
    let platform = use_platform();

    let onglobalkeydown = move |e: KeyboardEvent| {
        if e.key == Key::Tab {
            if e.modifiers.contains(Modifiers::SHIFT) {
                platform.focus(AccessibilityFocusStrategy::Backward);
            } else {
                platform.focus(AccessibilityFocusStrategy::Forward);
            }
        }
    };

    rsx!(rect {
        width: "100%",
        height: "100%",
        onglobalkeydown,
        {children}
    })
}
