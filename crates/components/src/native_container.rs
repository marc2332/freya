use dioxus::prelude::*;
use freya_core::accessibility::{
    AccessibilityFocusMovement,
    AccessibilityFocusStrategy,
};
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

    let onglobalkeydown = move |ev: KeyboardEvent| match ev.key {
        Key::Tab if ev.modifiers.contains(Modifiers::SHIFT) => {
            platform.request_focus(AccessibilityFocusStrategy::Backward(
                AccessibilityFocusMovement::OutsideGroup,
            ));
        }
        Key::Tab => {
            platform.request_focus(AccessibilityFocusStrategy::Forward(
                AccessibilityFocusMovement::OutsideGroup,
            ));
        }
        Key::ArrowUp => {
            platform.request_focus(AccessibilityFocusStrategy::Backward(
                AccessibilityFocusMovement::InsideGroup,
            ));
        }
        Key::ArrowDown => {
            platform.request_focus(AccessibilityFocusStrategy::Forward(
                AccessibilityFocusMovement::InsideGroup,
            ));
        }
        _ => {}
    };

    rsx!(rect {
        width: "100%",
        height: "100%",
        onglobalkeydown,
        {children}
    })
}
