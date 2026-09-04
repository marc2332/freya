use freya_clipboard::{
    copypasta::{
        ClipboardContext,
        ClipboardProvider,
    },
    prelude::GlobalClipboard,
};
use raw_window_handle::HasDisplayHandle;
#[cfg(target_os = "linux")]
use raw_window_handle::RawDisplayHandle;
use winit::event_loop::OwnedDisplayHandle;

pub(crate) fn create_clipboard(display_handle: OwnedDisplayHandle) -> GlobalClipboard {
    let provider = display_handle.display_handle().ok().and_then(|handle| {
        #[allow(clippy::match_single_binding)]
        match handle.as_raw() {
            #[cfg(target_os = "linux")]
            RawDisplayHandle::Wayland(handle) => {
                let (_primary, clipboard) = unsafe {
                    freya_clipboard::copypasta::wayland_clipboard::create_clipboards_from_external(
                        handle.display.as_ptr(),
                    )
                };
                Some(Box::new(WaylandClipboard {
                    clipboard,
                    _display_handle: display_handle.clone(),
                }) as Box<dyn ClipboardProvider>)
            }
            _ => ClipboardContext::new()
                .ok()
                .map(|clipboard| Box::new(clipboard) as Box<dyn ClipboardProvider>),
        }
    });
    GlobalClipboard::new(provider)
}

/// Keeps the Wayland display alive for as long as the clipboard uses it.
#[cfg(target_os = "linux")]
struct WaylandClipboard {
    clipboard: freya_clipboard::copypasta::wayland_clipboard::Clipboard,
    _display_handle: OwnedDisplayHandle,
}

#[cfg(target_os = "linux")]
impl ClipboardProvider for WaylandClipboard {
    fn get_contents(&mut self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.clipboard.get_contents()
    }

    fn set_contents(
        &mut self,
        contents: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.clipboard.set_contents(contents)
    }
}
