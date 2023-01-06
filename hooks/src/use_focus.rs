use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};
use uuid::Uuid;

/// Listen for focus changes, or mark your self as focused.
pub fn use_focus(cx: &ScopeState) -> (bool, impl Fn() + '_) {
    let my_id = cx.use_hook(Uuid::new_v4);
    let focused_id = use_shared_state::<Uuid>(cx);

    let is_focused = Some(*my_id) == focused_id.map(|v| *v.read());

    let focus = move || {
        if let Some(focused_id) = focused_id {
            *focused_id.write() = *my_id;
        }
    };

    (is_focused, focus)
}

/// Check if the hook caller is focused, and also provide
/// raw access to the focus holder and the hook caller ID.
pub fn use_raw_focus(cx: &ScopeState) -> (bool, Uuid, Option<UseSharedState<Uuid>>) {
    let my_id = cx.use_hook(Uuid::new_v4);
    let focused_id = use_shared_state::<Uuid>(cx);
    let focused = Some(*my_id) == focused_id.map(|v| *v.read());
    (focused, *my_id, focused_id)
}

/// Create a focus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_shared_state_provider(cx, Uuid::new_v4);
}
