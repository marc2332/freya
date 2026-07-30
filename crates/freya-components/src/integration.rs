use freya_core::{
    integration::AppComponent,
    prelude::*,
};

#[cfg(feature = "zoom-shortcuts")]
const ZOOM_STEP: f64 = 0.10;

pub fn integration(app: AppComponent) -> impl IntoElement {
    let platform = use_hook(Platform::get);

    let on_global_key_down = move |e: Event<KeyboardEventData>| match &e.key {
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
        #[cfg(feature = "zoom-shortcuts")]
        Key::Character(c) if e.modifiers.contains(Modifiers::ctrl_or_meta()) => {
            let custom_scale_factor = *platform.custom_scale_factor.peek();
            match c.as_str() {
                "+" | "=" => platform.set_custom_scale_factor(custom_scale_factor + ZOOM_STEP),
                "-" => platform.set_custom_scale_factor(custom_scale_factor - ZOOM_STEP),
                "0" => platform.set_custom_scale_factor(1.0),
                _ => {}
            }
        }
        _ => {}
    };

    rect().on_global_key_down(on_global_key_down).child(app)
}
