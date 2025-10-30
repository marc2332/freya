use freya_core::prelude::{
    State,
    use_try_consume,
};

#[derive(Default, Clone)]
pub struct ActivableRouteContext(pub State<bool>);

impl ActivableRouteContext {
    pub fn is_active(&self) -> bool {
        *self.0.read()
    }
}

pub fn use_activable_route() -> bool {
    let ctx = use_try_consume::<ActivableRouteContext>();

    if let Some(ctx) = ctx {
        ctx.is_active()
    } else {
        false
    }
}
