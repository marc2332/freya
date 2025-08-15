use dioxus::prelude::*;
use freya_core::accessibility::AccessibilityFocusStrategy;
use freya_elements::{
    self as dioxus_elements,
    events::{
        Key,
        KeyboardEvent,
        Modifiers,
    },
};
use freya_hooks::{
    use_init_native_platform,
    use_platform,
};

#[allow(non_snake_case)]
#[component]
pub fn NativeContainer(children: Element) -> Element {
    use_init_native_platform();
    let platform = use_platform();

    #[cfg(feature = "winit")]
    use_hook(|| {
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Get the window handle to create a clipboard
        platform.with_window(|window| {
            use dioxus_clipboard::integrations::window_handle::create_native_clipboard;
            use raw_window_handle::HasDisplayHandle;

            let display_handle = window.display_handle().unwrap();
            let clipboard = unsafe { create_native_clipboard(display_handle.as_raw()) };
            tx.send(clipboard).ok();
        });

        // Receive the clipboard and register it
        spawn(async move {
            let provider = rx.await;
            use dioxus_clipboard::integrations::window_handle::provide_native_clipboard;
            if let Ok(Some(provider)) = provider {
                provide_native_clipboard(provider);
            }
        });
    });

    let onglobalkeydown = move |e: KeyboardEvent| {
        if e.key == Key::Tab {
            if e.modifiers.contains(Modifiers::SHIFT) {
                platform.request_focus(AccessibilityFocusStrategy::Backward);
            } else {
                platform.request_focus(AccessibilityFocusStrategy::Forward);
            }
        }
    };

    rsx!(rect {
        width: "100%",
        height: "100%",
        onglobalkeydown,
        {children}
    })
}
