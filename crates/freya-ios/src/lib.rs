#![cfg(target_os = "ios")]

use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::prelude::*;
use freya_winit::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};

mod keyboard;
mod text_field;

pub use keyboard::KeyboardAttachError;
use keyboard::SoftKeyboard;

/// Freya plugin for iOS integration.
///
/// Attaches an invisible native text field to every window and registers a root component that
/// shows it while an input is focused. This enables IME composition and keyboards that match the
/// accessibility role of the input, which winit's own keyboard support does not provide.
#[derive(Default)]
pub struct IosPlugin {
    keyboard: KeyboardSlot,
}

/// The soft keyboard of the window, available once the window is created.
#[derive(Clone, Default)]
struct KeyboardSlot(Rc<RefCell<Option<SoftKeyboard>>>);

impl FreyaPlugin for IosPlugin {
    fn plugin_id(&self) -> &'static str {
        "ios"
    }

    fn on_event(&mut self, event: &mut PluginEvent, handle: PluginHandle) {
        match event {
            PluginEvent::RunnerCreated { runner } => {
                let keyboard = self.keyboard.clone();
                runner.provide_root_context(move || keyboard);
            }
            PluginEvent::WindowCreated { window, .. } => {
                match SoftKeyboard::attach(window, handle) {
                    Ok(keyboard) => *self.keyboard.0.borrow_mut() = Some(keyboard),
                    Err(err) => tracing::error!("Failed to attach the soft keyboard: {err:?}"),
                }
            }
            _ => {}
        }
    }

    fn root_component(&self, root: Element) -> Element {
        IosRoot { inner: root }.into_element()
    }
}

/// Root component that manages iOS platform integration.
#[derive(Clone)]
struct IosRoot {
    inner: Element,
}

impl PartialEq for IosRoot {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Component for IosRoot {
    fn render(&self) -> impl IntoElement {
        let keyboard = use_consume::<KeyboardSlot>();

        use_side_effect(move || {
            let platform = Platform::get();
            // The focused id changes one frame before the focused node, so only the node is
            // subscribed to. Otherwise the keyboard would be configured with the previous role.
            let role = platform.focused_accessibility_node.read().role();
            let id = *platform.focused_accessibility_id.peek();
            if let Some(keyboard) = keyboard.0.borrow().as_ref() {
                keyboard.update(id, role);
            }
        });

        self.inner.clone()
    }
}
