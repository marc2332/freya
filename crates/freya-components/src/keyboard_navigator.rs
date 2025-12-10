use freya_core::{
    element::FpRender,
    prelude::*,
};

pub fn keyboard_navigator(app: FpRender) -> impl IntoElement {
    let platform = Platform::get();
    let on_global_key_down = move |e: Event<KeyboardEventData>| match e.key {
        Key::Tab if e.modifiers.contains(Modifiers::SHIFT) => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Backward(AccessibilityFocusMovement::OutsideGroup),
            ));
        }
        Key::Tab => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Forward(AccessibilityFocusMovement::OutsideGroup),
            ));
        }
        Key::ArrowUp => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Backward(AccessibilityFocusMovement::InsideGroup),
            ));
        }
        Key::ArrowDown => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Forward(AccessibilityFocusMovement::InsideGroup),
            ));
        }
        _ => {}
    };

    rect().on_global_key_down(on_global_key_down).child(app)
}
