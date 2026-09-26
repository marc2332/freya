use freya_core::prelude::*;
use freya_router::prelude::RouterContext;

/// Adds native back and forward mouse button navigation to an element.
///
/// Call [`Self::native_router`] on a root `rect` inside a router. See the
/// [router example](https://github.com/marc2332/freya/blob/main/examples/feature_router_complex.rs).
pub trait NativeRouterExt {
    /// Wires native back and forward mouse buttons to router navigation.
    fn native_router(self) -> Self;
}

impl NativeRouterExt for Rect {
    fn native_router(self) -> Self {
        self.on_global_pointer_up(|e: Event<PointerEventData>| match e.button() {
            Some(MouseButton::Back) => RouterContext::get().go_back(),
            Some(MouseButton::Forward) => RouterContext::get().go_forward(),
            _ => {}
        })
    }
}
