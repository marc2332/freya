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

/// Graphics driver using Metal (macOS native).
pub struct MetalDriver {
    metal_layer: Retained<CAMetalLayer>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    gr_context: DirectContext,
    /// Whether the device was created by Freya rather than by an external renderer.
    owns_device: bool,
}

impl MetalDriver {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
    ) -> (Self, Window) {
        let device = MTLCreateSystemDefaultDevice().expect("No Metal-capable device found");
        let command_queue = device
            .newCommandQueue()
            .expect("Could not create Metal command queue");

        Self::build(
            event_loop,
            window_attributes,
            gpu_resource_cache_limit,
            device,
            command_queue,
            true,
        )
        .expect("Could not create window with Metal context")
    }

    /// Build the driver on a Metal device owned by an external renderer.
    ///
    /// Sharing the command queue is what orders the owner's work against Freya's command buffers.
    pub fn from_external(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
        external: crate::drivers::ExternalMetalDevice,
    ) -> Result<(Self, Window), Box<dyn std::error::Error>> {
        Self::build(
            event_loop,
            window_attributes,
            gpu_resource_cache_limit,
            external.device,
            external.command_queue,
            false,
        )
    }

    fn build(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
        device: Retained<ProtocolObject<dyn MTLDevice>>,
        command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
        owns_device: bool,
    ) -> Result<(Self, Window), Box<dyn std::error::Error>> {
        let transparent = window_attributes.transparent;
        let window = event_loop.create_window(window_attributes)?;

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
            if transparent {
                layer.setOpaque(false);
            }

            let raw_handle = window.window_handle()?.as_raw();

            match raw_handle {
                RawWindowHandle::AppKit(appkit) => {
                    let view = unsafe { (appkit.ns_view.as_ptr() as *mut NSView).as_ref() }
                        .ok_or("NSView pointer is null")?;

                    view.setWantsLayer(true);
                    view.setLayer(Some(&layer));
                }
                _ => return Err("Metal driver only supports AppKit windows".into()),
            };

            layer
        };

        let backend = unsafe {
            mtl::BackendContext::new(
                Retained::as_ptr(&device) as mtl::Handle,
                Retained::as_ptr(&command_queue) as mtl::Handle,
            )
        };

        let mut gr_context =
            direct_contexts::make_metal(&backend, None).ok_or("Could not create Metal context")?;

        gr_context.set_resource_cache_limit(gpu_resource_cache_limit);

        let driver = Self {
            metal_layer,
            command_queue,
            gr_context,
            owns_device,
        };

        Ok((driver, window))
    }

    /// The Skia context when the device is shared with an external renderer.
    pub fn shared_gr_context(&self) -> Option<DirectContext> {
        (!self.owns_device).then(|| self.gr_context.clone())
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
