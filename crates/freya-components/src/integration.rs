use freya_core::{
    integration::AppComponent,
    layers::Layer,
    prelude::*,
};
use torin::prelude::{
    Position,
    Size2D,
};

use crate::context_menu::ContextMenu;

pub fn integration(app: AppComponent) -> impl IntoElement {
    let platform = use_hook(Platform::get);
    let mut context = use_hook(ContextMenu::get);
    let mut menu_size = use_state(|| None::<Size2D>);

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

    let on_global_pointer_move = move |e: Event<PointerEventData>| {
        context.location.set(e.global_location());
    };

    rect()
        .on_global_pointer_move(on_global_pointer_move)
        .on_global_key_down(on_global_key_down)
        .child(app)
        .maybe_child(context.menu.read().clone().map(|(location, menu)| {
            let location = location.to_f32();
            let window = Platform::get().root_size.read();

            let (x, y, opacity) = match *menu_size.read() {
                None => (location.x, location.y, 0.0),
                Some(size) => {
                    let x = if location.x + size.width > window.width {
                        (location.x - size.width).max(0.0)
                    } else {
                        location.x
                    };
                    let y = if location.y + size.height > window.height {
                        (location.y - size.height).max(0.0)
                    } else {
                        location.y
                    };
                    (x, y, 1.0)
                }
            };

            rect()
                .layer(Layer::Overlay)
                .position(Position::new_global().left(x).top(y))
                .opacity(opacity)
                .on_sized(move |e: Event<SizedEventData>| {
                    menu_size.set(Some(e.area.size));
                })
                .child(menu.on_close(move |_| {
                    menu_size.set(None);
                    context.menu.set(None);
                }))
        }))
}
