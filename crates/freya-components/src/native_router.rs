use freya_core::prelude::*;
use freya_router::prelude::RouterContext;

pub trait NativeRouterExt {
    /// Wire native back/forward mouse buttons to router navigation.
    fn native_router(self) -> Self;
}

impl NativeRouterExt for Rect {
    fn native_router(self) -> Self {
        self.on_global_pointer_press(|e: Event<PointerEventData>| match e.button() {
            Some(MouseButton::Back) => RouterContext::get().go_back(),
            Some(MouseButton::Forward) => RouterContext::get().go_forward(),
            _ => {}
        })
    }
}
