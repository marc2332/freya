use freya_core::{
    element::FpRender,
    layers::Layer,
    prelude::*,
};
use torin::prelude::Position;

use crate::context_menu::ContextMenu;

pub fn integration(app: FpRender) -> impl IntoElement {
    let platform = use_hook(Platform::get);
    let mut context = use_hook(ContextMenu::get);

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

    let on_global_mouse_move = move |e: Event<MouseEventData>| {
        context.location.set(e.global_location);
    };

    rect()
        .on_global_mouse_move(on_global_mouse_move)
        .on_global_key_down(on_global_key_down)
        .child(app)
        .maybe_child(context.menu.read().clone().and_then(|(location, menu)| {
            let location = location.to_f32();
            Some(
                rect()
                    .layer(Layer::Overlay)
                    .position(Position::new_global().left(location.x).top(location.y))
                    .child(menu.on_close(move |_| context.menu.set(None))),
            )
        }))
}
