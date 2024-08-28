use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::ForeignType;
use metal::{CommandQueue, Device, MTLPixelFormat, MetalLayer};
use objc::runtime::YES;
use winit::{raw_window_handle::HasWindowHandle, window::Window};

use freya_engine::prelude::*;

pub struct SkiaSurface {
    pub(crate) gr_context: DirectContext,
    pub(crate) metal_layer: MetalLayer,
    pub(crate) command_queue: CommandQueue,
    pub(crate) scale_factor: f64,
}

impl SkiaSurface {
    pub fn create(window: &Window) -> Self {
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
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                command_queue.as_ptr() as mtl::Handle,
            )
        };

        let mut gr_context =
            direct_contexts::make_metal(&backend, None).expect("Could not create direct context");

        Self {
            gr_context,
            metal_layer,
            command_queue,
            scale_factor
        }
    }
}
