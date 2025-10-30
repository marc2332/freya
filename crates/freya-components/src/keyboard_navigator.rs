use freya_core::{
    element::FpRender,
    prelude::*,
};

pub fn keyboard_navigator(app: FpRender) -> Element {
    let event_notifier = EventNotifier::get();
    let on_global_key_down = move |e: Event<KeyboardEventData>| match e.key {
        Key::Tab if e.modifiers.contains(Modifiers::SHIFT) => {
            event_notifier.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Backward(AccessibilityFocusMovement::OutsideGroup),
            ));
        }
        Key::Tab => {
            event_notifier.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Forward(AccessibilityFocusMovement::OutsideGroup),
            ));
        }
        Key::ArrowUp => {
            event_notifier.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Backward(AccessibilityFocusMovement::InsideGroup),
            ));
        }
        Key::ArrowDown => {
            event_notifier.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Forward(AccessibilityFocusMovement::InsideGroup),
            ));
        }
        _ => {}
    };

    rect()
        .on_global_key_down(on_global_key_down)
        .child(app)
        .into()
}
