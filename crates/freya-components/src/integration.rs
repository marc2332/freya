use freya_core::{
    integration::AppComponent,
    prelude::*,
};

pub fn integration(app: AppComponent) -> impl IntoElement {
    let platform = use_hook(Platform::get);

    let on_global_key_down = move |e: Event<KeyboardEventData>| match e.key {
        Key::Named(NamedKey::Tab) if e.modifiers == Modifiers::SHIFT => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Backward(AccessibilityFocusMovement::OutsideGroup),
            ));
        }
        Key::Named(NamedKey::Tab) if e.modifiers.is_empty() => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Forward(AccessibilityFocusMovement::OutsideGroup),
            ));
        }
        Key::Named(NamedKey::ArrowUp) if e.modifiers.is_empty() => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Backward(AccessibilityFocusMovement::InsideGroup),
            ));
        }
        Key::Named(NamedKey::ArrowDown) if e.modifiers.is_empty() => {
            platform.send(UserEvent::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Forward(AccessibilityFocusMovement::InsideGroup),
            ));
        }
        _ => {}
    };

    rect().on_global_key_down(on_global_key_down).child(app)
}
