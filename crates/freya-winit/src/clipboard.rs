#[cfg(not(target_os = "android"))]
use freya_clipboard::copypasta::{
    ClipboardContext,
    ClipboardProvider,
};
use freya_clipboard::prelude::Clipboard;
#[cfg(not(target_os = "android"))]
use raw_window_handle::HasDisplayHandle;
#[cfg(target_os = "linux")]
use raw_window_handle::RawDisplayHandle;
use winit::event_loop::OwnedDisplayHandle;

#[cfg(target_os = "android")]
pub(crate) fn create_clipboard(_display_handle: OwnedDisplayHandle) -> Clipboard {
    Clipboard::create(Some(Box::new(freya_clipboard::android::AndroidClipboard)))
}

#[cfg(not(target_os = "android"))]
pub(crate) fn create_clipboard(display_handle: OwnedDisplayHandle) -> Clipboard {
    #[cfg(target_os = "linux")]
    let mut primary: Option<Box<dyn ClipboardProvider>> = None;
    #[cfg(not(target_os = "linux"))]
    let primary: Option<Box<dyn ClipboardProvider>> = None;

    let provider = display_handle.display_handle().ok().and_then(|handle| {
        #[allow(clippy::match_single_binding)]
        match handle.as_raw() {
            #[cfg(target_os = "linux")]
            RawDisplayHandle::Wayland(handle) => {
                let (wayland_primary, wayland_clipboard) = unsafe {
                    freya_clipboard::copypasta::wayland_clipboard::create_clipboards_from_external(
                        handle.display.as_ptr(),
                    )
                };
                primary = Some(Box::new(WaylandPrimary {
                    primary: wayland_primary,
                    _display_handle: display_handle.clone(),
                }) as Box<dyn ClipboardProvider>);
                Some(Box::new(WaylandClipboard {
                    clipboard: wayland_clipboard,
                    _display_handle: display_handle.clone(),
                }) as Box<dyn ClipboardProvider>)
            }
            _ => {
                #[cfg(target_os = "linux")]
                {
                    primary = freya_clipboard::copypasta::x11_clipboard::X11ClipboardContext::<
                        freya_clipboard::copypasta::x11_clipboard::Primary,
                    >::new()
                    .ok()
                    .map(|clipboard| Box::new(clipboard) as Box<dyn ClipboardProvider>);
                }
                ClipboardContext::new()
                    .ok()
                    .map(|clipboard| Box::new(clipboard) as Box<dyn ClipboardProvider>)
            }
        }
    });

    let clipboard = Clipboard::create(provider);
    clipboard.set_primary_provider(primary);
    clipboard
}

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

#[cfg(target_os = "linux")]
struct WaylandPrimary {
    primary: freya_clipboard::copypasta::wayland_clipboard::Primary,
    _display_handle: OwnedDisplayHandle,
}

#[cfg(target_os = "linux")]
impl ClipboardProvider for WaylandPrimary {
    fn get_contents(&mut self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.primary.get_contents()
    }

    fn set_contents(
        &mut self,
        contents: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.primary.set_contents(contents)
    }
}
