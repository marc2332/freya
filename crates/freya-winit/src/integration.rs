pub use freya_core::integration::is_ime_role;
use winit::{
    dpi::{
        LogicalPosition,
        LogicalSize,
    },
    window::{
        ImeCapabilities,
        ImeEnableRequest,
        ImeRequest,
        ImeRequestData,
        Window,
    },
};

pub(crate) fn update_ime(
    window: &dyn Window,
    allowed: bool,
    position: LogicalPosition<f32>,
    size: LogicalSize<f32>,
) {
    let enabled = window.ime_capabilities().is_some();
    let request = if allowed {
        let data = ImeRequestData::default().with_cursor_area(position.into(), size.into());
        if enabled {
            ImeRequest::Update(data)
        } else {
            let Some(request) =
                ImeEnableRequest::new(ImeCapabilities::new().with_cursor_area(), data)
            else {
                return;
            };
            ImeRequest::Enable(request)
        }
    } else if enabled {
        ImeRequest::Disable
    } else {
        return;
    };
    if let Err(error) = window.request_ime_update(request) {
        tracing::warn!(?error, "Failed to update input method");
    }
}
