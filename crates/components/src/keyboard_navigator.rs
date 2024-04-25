use dioxus::prelude::*;
use freya_common::EventMessage;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::KeyboardEvent;
use freya_hooks::{use_init_accessibility, use_platform};

#[allow(non_snake_case)]
#[component]
pub fn KeyboardNavigator(children: Element) -> Element {
    let mut navigation_mark = use_init_accessibility();
    let platform = use_platform();

    let onkeydown = move |e: KeyboardEvent| {
        let allowed_to_navigate = navigation_mark.peek().allowed();
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
            navigation_mark.write().set_allowed(true)
        }
    };

    rsx!(rect {
        width: "100%",
        height: "100%",
        onkeydown,
        {children}
    })
}
