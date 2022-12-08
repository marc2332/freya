use dioxus::prelude::{use_context, use_context_provider, ScopeState, UseSharedState};
use uuid::Uuid;

/// Check if the hook caller is focused, and also provide a shortcut to
/// mark the same hook caller as focused.
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

/// Check if the hook caller is focused, and also provide
/// raw access to the focus holder and the hook caller ID.
pub fn use_raw_focus(cx: &ScopeState) -> (bool, Uuid, Option<UseSharedState<Uuid>>) {
    let my_id = cx.use_hook(Uuid::new_v4);
    let focused_id = use_context::<Uuid>(cx);
    let focused = Some(*my_id) == focused_id.map(|v| *v.read());
    (focused, *my_id, focused_id)
}

/// Create a Foxus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_context_provider(cx, Uuid::new_v4);
}
