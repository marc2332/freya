use dioxus::prelude::{use_context, use_context_provider, ScopeState};
use uuid::Uuid;

/// Connect to the Focus provider.
pub fn use_focus(cx: &ScopeState) -> (bool, impl Fn() + '_) {
    let my_id = cx.use_hook(Uuid::new_v4);
    let focused_id = use_context::<Uuid>(cx);

    let focused = Some(*my_id) == focused_id.map(|v| *v.read());

    let focus = move || {
        if let Some(focused_id) = focused_id {
            *focused_id.write() = *my_id;
        }
    };

    (focused, focus)
}

/// Create a Foxus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_context_provider(cx, Uuid::new_v4);
}
