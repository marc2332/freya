use dioxus_hooks::try_use_context;
use dioxus_signals::{Readable, Signal};

#[derive(Default, Clone)]
pub struct ActivableRouteContext(pub Signal<bool>);

impl ActivableRouteContext {
    pub fn is_active(&self) -> bool {
        *self.0.read()
    }
}

pub fn use_activable_route() -> bool {
    let ctx = try_use_context::<ActivableRouteContext>();

    if let Some(ctx) = ctx {
        ctx.is_active()
    } else {
        false
    }
}
