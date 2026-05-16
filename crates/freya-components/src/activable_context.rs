use freya_core::prelude::{
    Readable,
    use_try_consume,
};

/// Context that exposes whether the current subtree is considered active.
///
/// Provided by [`Activable`](crate::activable::Activable) for user-controlled state
/// and by [`ActivableRoute`](crate::activable_route::ActivableRoute) for route-driven
/// state. Descendants read it via [`use_is_active`].
#[derive(Clone)]
pub struct ActivableContext(pub Readable<bool>);

impl ActivableContext {
    pub fn is_active(&self) -> bool {
        *self.0.read()
    }
}

/// Returns whether the closest ancestor [`ActivableContext`] provider is active.
///
/// Falls back to `false` when no provider is found in the tree.
pub fn use_is_active() -> bool {
    let ctx = use_try_consume::<ActivableContext>();

    if let Some(ctx) = ctx {
        ctx.is_active()
    } else {
        false
    }
}
