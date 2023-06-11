use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};

pub fn use_init_scroll(cx: &ScopeState) {
    use_shared_state_provider(cx, UseScroll::default);
}

pub fn use_scroll(cx: &ScopeState) -> &UseSharedState<UseScroll> {
    use_shared_state(cx).unwrap()
}

#[derive(Debug, Clone, Default)]
pub struct UseScroll {
    pub scroll_y: i32,
}

impl UseScroll {
    pub fn go_top(&mut self) {
        self.scroll_y = 0;
    }
}
