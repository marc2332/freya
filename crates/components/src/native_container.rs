use dioxus::prelude::*;
use freya_common::EventMessage;
use freya_elements::{
    elements as dioxus_elements,
    events::KeyboardEvent,
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

    let onkeydown = move |e: KeyboardEvent| {
        let allowed_to_navigate = native_platform.navigation_mark.peek().allowed();
        if e.key == Key::Tab && allowed_to_navigate {
            if e.modifiers.contains(Modifiers::SHIFT) {
                platform
                    .send(EventMessage::FocusPrevAccessibilityNode)
                    .unwrap();
            } else {
                platform
                    .send(EventMessage::FocusNextAccessibilityNode)
                    .unwrap();
            }
        } else {
            native_platform.navigation_mark.write().set_allowed(true)
        }
    };

    rsx!(rect {
        width: "100%",
        height: "100%",
        onkeydown,
        {children}
    })
}
