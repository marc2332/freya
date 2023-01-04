use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider};
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

/// Create a focus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_shared_state_provider(cx, Uuid::new_v4);
}
