use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use freya_engine::prelude::{
    backend_render_targets, direct_contexts, scalar, wrap_backend_render_target, BackendContext,
    ColorType, DirectContext, Handle, Surface as SkiaSurface, SurfaceOrigin, TextureInfo,
};
use metal::{CommandQueue, Device, MTLPixelFormat, MetalLayer};
use objc::runtime::YES;
use raw_window_handle::HasWindowHandle;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use crate::{size::WinitSize, LaunchConfig};

/// Graphics driver using Metal.
pub struct MetalDriver {
    pub(crate) gr_context: DirectContext,
    pub(crate) metal_layer: MetalLayer,
    pub(crate) command_queue: CommandQueue,
    pub(crate) scale_factor: f64,
}

impl MetalDriver {
    pub fn new<State: Clone + 'static>(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        _config: &LaunchConfig<State>,
    ) -> (Self, Window, SkiaSurface) {
        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create Window");

        let scale_factor = window.scale_factor();
        let window_handle = window
            .window_handle()
            .expect("Failed to retrieve a window handle");
        let raw_window_handle = window_handle.as_raw();

        let device = Device::system_default().expect("no device found");
        let command_queue = device.new_command_queue();

        let metal_layer = {
            let draw_size = window.inner_size();
            let layer = MetalLayer::new();
            layer.set_device(&device);
            layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);
            // Disabling this option allows Skia's Blend Mode to work.
            // More about: https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly
            layer.set_framebuffer_only(false);

            unsafe {
                let view = match raw_window_handle {
                    raw_window_handle::RawWindowHandle::AppKit(appkit) => appkit.ns_view.as_ptr(),
                    _ => panic!("Wrong window handle type"),
                } as cocoa_id;
                view.setWantsLayer(YES);
                view.setLayer(layer.as_ref() as *const _ as _);
            }
            layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
            layer
        };

        let backend = unsafe {
            BackendContext::new(device.as_ptr() as Handle, command_queue.as_ptr() as Handle)
        };

        let mut gr_context =
            direct_contexts::make_metal(&backend, None).expect("Could not create Metal context");

        let drawable = metal_layer.next_drawable().unwrap();
        let (drawable_width, drawable_height) = {
            let size = metal_layer.drawable_size();
            (size.width as scalar, size.height as scalar)
        };

        let skia_surface = unsafe {
            let texture_info = TextureInfo::new(drawable.texture().as_ptr() as Handle);

            let backend_render_target = backend_render_targets::make_mtl(
                (drawable_width as i32, drawable_height as i32),
                &texture_info,
            );

            wrap_backend_render_target(
                &mut gr_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
            .unwrap()
        };

        let driver = MetalDriver {
            gr_context,
            metal_layer,
            command_queue,
            scale_factor,
        };

        (driver, window, skia_surface)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> (SkiaSurface, SkiaSurface) {
        let drawable = self.metal_layer.next_drawable().unwrap();

        let mut surface = unsafe {
            let texture_info = TextureInfo::new(drawable.texture().as_ptr() as Handle);

            let backend_render_target =
                backend_render_targets::make_mtl(size.to_skia(), &texture_info);

            wrap_backend_render_target(
                &mut self.gr_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
            .unwrap()
        };

        let dirty_surface = surface.new_surface_with_dimensions(size.to_skia()).unwrap();

        self.metal_layer
            .set_drawable_size(CGSize::new(size.width as f64, size.height as f64));

        (surface, dirty_surface)
    }
}
