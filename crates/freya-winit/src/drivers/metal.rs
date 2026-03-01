use freya_engine::prelude::{
    ColorType,
    DirectContext,
    Surface as SkiaSurface,
    SurfaceOrigin,
    backend_render_targets,
    direct_contexts,
    mtl,
    wrap_backend_render_target,
};
use objc2::{
    rc::Retained,
    runtime::ProtocolObject,
};
use objc2_app_kit::NSView;
use objc2_core_foundation::CGSize;
use objc2_metal::{
    MTLCommandBuffer,
    MTLCommandQueue,
    MTLCreateSystemDefaultDevice,
    MTLDevice,
    MTLDrawable,
    MTLPixelFormat,
};
use objc2_quartz_core::{
    CAMetalDrawable,
    CAMetalLayer,
};
use raw_window_handle::{
    HasWindowHandle,
    RawWindowHandle,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

use crate::config::WindowConfig;

/// Graphics driver using Metal (macOS native).
pub struct MetalDriver {
    metal_layer: Retained<CAMetalLayer>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    gr_context: DirectContext,
}

impl MetalDriver {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        window_config: &WindowConfig,
    ) -> (Self, Window) {
        let window = event_loop
            .create_window(window_attributes)
            .expect("Could not create window with Metal context");

        let device = MTLCreateSystemDefaultDevice().expect("No Metal-capable device found");

        let size = window.inner_size();

        let metal_layer = {
            let layer = CAMetalLayer::new();
            layer.setDevice(Some(&device));
            layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
            layer.setPresentsWithTransaction(false);
            // Disabling framebufferOnly allows Skia's blend modes to work correctly.
            // See: https://developer.apple.com/documentation/quartzcore/cametallayer/1478168-framebufferonly
            layer.setFramebufferOnly(false);
            layer.setDrawableSize(CGSize::new(size.width as f64, size.height as f64));

            // Handle transparency
            if window_config.transparent {
                layer.setOpaque(false);
            }

            let raw_handle = window
                .window_handle()
                .expect("Could not get window handle")
                .as_raw();

            match raw_handle {
                RawWindowHandle::AppKit(appkit) => {
                    let view = unsafe { (appkit.ns_view.as_ptr() as *mut NSView).as_ref() }
                        .expect("NSView pointer is null");

                    view.setWantsLayer(true);
                    view.setLayer(Some(&layer.clone()));
                }
                _ => panic!("Metal driver only supports AppKit (macOS) windows"),
            };

            layer
        };

        let command_queue = device
            .newCommandQueue()
            .expect("Could not create Metal command queue");

        let backend = unsafe {
            mtl::BackendContext::new(
                Retained::as_ptr(&device) as mtl::Handle,
                Retained::as_ptr(&command_queue) as mtl::Handle,
            )
        };

        let gr_context =
            direct_contexts::make_metal(&backend, None).expect("Could not create Metal context");

        let driver = Self {
            metal_layer,
            command_queue,
            gr_context,
        };

        (driver, window)
    }

    pub fn present(
        &mut self,
        _size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        let Some(drawable) = self.metal_layer.nextDrawable() else {
            // No drawable available, skip this frame
            return;
        };

        let (drawable_width, drawable_height) = {
            let size = self.metal_layer.drawableSize();
            (size.width as i32, size.height as i32)
        };

        let texture_info =
            unsafe { mtl::TextureInfo::new(Retained::as_ptr(&drawable.texture()) as mtl::Handle) };

        let backend_render_target =
            backend_render_targets::make_mtl((drawable_width, drawable_height), &texture_info);

        let mut surface = wrap_backend_render_target(
            &mut self.gr_context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            None,
            None,
        )
        .expect("Could not create Skia surface from Metal texture");

        render(&mut surface);

        window.pre_present_notify();
        self.gr_context.flush_and_submit();
        drop(surface);

        let command_buffer = self
            .command_queue
            .commandBuffer()
            .expect("Could not get Metal command buffer");

        let mtl_drawable: Retained<ProtocolObject<dyn MTLDrawable>> = (&drawable).into();
        command_buffer.presentDrawable(&mtl_drawable);
        command_buffer.commit();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.metal_layer
            .setDrawableSize(CGSize::new(size.width as f64, size.height as f64));
    }
}
