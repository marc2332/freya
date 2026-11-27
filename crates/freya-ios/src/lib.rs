#![cfg(target_os = "ios")]

use std::collections::HashMap;

use freya_winit::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};
use winit::window::WindowId;

mod keyboard;
mod text_field;

pub use keyboard::KeyboardAttachError;
use keyboard::SoftKeyboard;

/// Provides native iOS keyboard layouts and IME composition for Freya inputs.
#[derive(Default)]
pub struct IosPlugin {
    keyboards: HashMap<WindowId, SoftKeyboard>,
}

impl FreyaPlugin for IosPlugin {
    fn plugin_id(&self) -> &'static str {
        "ios"
    }

    fn on_event(&mut self, event: &mut PluginEvent, handle: PluginHandle) {
        match event {
            PluginEvent::WindowCreated { window, .. } => {
                match SoftKeyboard::attach(*window, handle) {
                    Ok(keyboard) => {
                        self.keyboards.insert(window.id(), keyboard);
                    }
                    Err(err) => tracing::error!(?err, "Failed to attach the soft keyboard"),
                }
            }
            PluginEvent::ImeUpdate {
                window,
                focused_id,
                role,
                handled,
            } if !**handled && let Some(keyboard) = self.keyboards.get(&window.id()) => {
                keyboard.update(*focused_id, *role);
                **handled = true;
            }
            PluginEvent::WindowClosed { window, .. } => {
                self.keyboards.remove(&window.id());
            }
            _ => {}
        }
    }
}
