#[cfg(feature = "gpu")]
use freya_engine::prelude::{
    BackendRenderTarget,
    RecordingContext,
    SurfaceOrigin,
    wrap_backend_render_target,
};
use freya_engine::prelude::{
    Borrows,
    ColorType,
    ImageInfo,
    Surface,
    SurfaceProps,
    wrap_pixels,
};
#[cfg(all(
    feature = "platform-text-gamma",
    any(target_os = "linux", target_os = "macos")
))]
use freya_engine::prelude::{
    PixelGeometry,
    SurfacePropsFlags,
};

#[cfg(all(feature = "platform-text-gamma", target_os = "linux"))]
fn default_surface_props() -> Option<SurfaceProps> {
    Some(SurfaceProps::new_with_text_properties(
        SurfacePropsFlags::default(),
        PixelGeometry::Unknown,
        0.2,
        1.0,
    ))
}

#[cfg(all(feature = "platform-text-gamma", target_os = "macos"))]
fn default_surface_props() -> Option<SurfaceProps> {
    Some(SurfaceProps::new_with_text_properties(
        SurfacePropsFlags::default(),
        PixelGeometry::Unknown,
        0.0,
        0.0,
    ))
}

#[cfg(not(all(
    feature = "platform-text-gamma",
    any(target_os = "linux", target_os = "macos")
)))]
fn default_surface_props() -> Option<SurfaceProps> {
    None
}

#[cfg(feature = "gpu")]
pub(super) fn wrap_render_target(
    context: &mut RecordingContext,
    render_target: &BackendRenderTarget,
    origin: SurfaceOrigin,
    color_type: ColorType,
) -> Option<Surface> {
    wrap_backend_render_target(
        context,
        render_target,
        origin,
        color_type,
        None,
        default_surface_props().as_ref(),
    )
}

pub(super) fn wrap_software_pixels<'pixels>(
    image_info: &ImageInfo,
    pixels: &'pixels mut [u8],
    row_bytes: usize,
) -> Option<Borrows<'pixels, Surface>> {
    wrap_pixels(
        image_info,
        pixels,
        Some(row_bytes),
        default_surface_props().as_ref(),
    )
}
