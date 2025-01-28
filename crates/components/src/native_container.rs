use dioxus::prelude::*;
use freya_common::AccessibilityFocusStrategy;
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
    let mut native_platform = use_init_native_platform();
    let platform = use_platform();

    let onglobalkeydown = move |e: KeyboardEvent| {
        let allowed_to_navigate = native_platform.navigation_mark.peek().allowed();
        if e.key == Key::Tab && allowed_to_navigate {
            if e.modifiers.contains(Modifiers::SHIFT) {
                platform.focus(AccessibilityFocusStrategy::Backward);
            } else {
                platform.focus(AccessibilityFocusStrategy::Forward);
            }
        } else {
            native_platform.navigation_mark.write().set_allowed(true)
        }
    };

    rsx!(rect {
        width: "100%",
        height: "100%",
        onglobalkeydown,
        {children}
    })
}
