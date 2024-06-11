use dioxus_hooks::use_context;
use dioxus_signals::{
    ReadOnlySignal,
    Signal,
};
use freya_core::prelude::PreferredTheme;

/// Access the preferred theme selected by the user.
pub fn use_preferred_theme() -> ReadOnlySignal<PreferredTheme> {
    use_context::<Signal<PreferredTheme>>().into()
}
