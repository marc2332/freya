use freya_engine::prelude::{
    ColorType,
    DirectContext,
    Surface as SkiaSurface,
    SurfaceOrigin,
    backend_render_targets,
    direct_contexts,
    mtl,
};
use objc2::{
    rc::Retained,
    runtime::ProtocolObject,
};
#[cfg(target_os = "macos")]
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
#[cfg(target_os = "ios")]
use objc2_quartz_core::CAAutoresizingMask;
use objc2_quartz_core::{
    CAMetalDrawable,
    CAMetalLayer,
};
#[cfg(target_os = "ios")]
use objc2_ui_kit::UIView;
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

use crate::drivers::surface::wrap_render_target;

/// Graphics driver using Metal on Apple platforms.
pub struct MetalDriver {
    metal_layer: Retained<CAMetalLayer>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    gr_context: DirectContext,
    #[cfg(target_os = "ios")]
    view: Retained<UIView>,
}

impl MetalDriver {
    #[cfg(target_os = "ios")]
    fn sync_view_geometry(&self, scale: f64) {
        let Some(window) = self.view.window() else {
            return;
        };
        let bounds = window.bounds();
        self.view.setFrame(bounds);
        self.metal_layer.setFrame(self.view.bounds());
        self.metal_layer.setContentsScale(scale);
    }

    pub fn resource_cache_usage(&self) -> (usize, usize) {
        let usage = self.gr_context.resource_cache_usage();
        (usage.resource_bytes, self.gr_context.resource_cache_limit())
    }

    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
        gpu_resource_cache_limit: usize,
    ) -> (Self, Window) {
        let transparent = window_attributes.transparent;
        let window = event_loop
            .create_window(window_attributes)
            .expect("Could not create window with Metal context");

        let device = MTLCreateSystemDefaultDevice().expect("No Metal-capable device found");

        let size = crate::drawable_size(&window);

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

            let raw_handle = window
                .window_handle()
                .expect("Could not get window handle")
                .as_raw();

            #[cfg(target_os = "macos")]
            {
                match raw_handle {
                    RawWindowHandle::AppKit(appkit) => {
                        let view = unsafe { (appkit.ns_view.as_ptr() as *mut NSView).as_ref() }
                            .expect("NSView pointer is null");

                        view.setWantsLayer(true);
                        view.setLayer(Some(&layer));
                    }
                    _ => panic!("Metal driver only supports AppKit (macOS) windows"),
                };
                layer
            }

            #[cfg(target_os = "ios")]
            {
                let view = match raw_handle {
                    RawWindowHandle::UiKit(uikit) => {
                        unsafe { Retained::retain(uikit.ui_view.as_ptr() as *mut UIView) }
                            .expect("UIView pointer is null")
                    }
                    _ => panic!("Metal driver only supports UIKit (iOS) windows"),
                };
                if let Some(window) = view.window() {
                    view.setFrame(window.bounds());
                }
                layer.setAutoresizingMask(
                    CAAutoresizingMask::LayerWidthSizable | CAAutoresizingMask::LayerHeightSizable,
                );
                view.layer().addSublayer(&layer);
                (layer, view)
            }
        };

        #[cfg(target_os = "ios")]
        let (metal_layer, view) = metal_layer;

        let command_queue = device
            .newCommandQueue()
            .expect("Could not create Metal command queue");

        let backend = unsafe {
            mtl::BackendContext::new(
                Retained::as_ptr(&device) as mtl::Handle,
                Retained::as_ptr(&command_queue) as mtl::Handle,
            )
        };

        let mut gr_context =
            direct_contexts::make_metal(&backend, None).expect("Could not create Metal context");

        gr_context.set_resource_cache_limit(gpu_resource_cache_limit);

        let driver = Self {
            metal_layer,
            command_queue,
            gr_context,
            #[cfg(target_os = "ios")]
            view,
        };

        (driver, window)
    }

    #[cfg_attr(not(target_os = "ios"), allow(unused_variables))]
    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        #[cfg(target_os = "ios")]
        {
            let scale = window.scale_factor();
            self.sync_view_geometry(scale);
            self.metal_layer
                .setDrawableSize(CGSize::new(size.width as f64, size.height as f64));
        }

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

        let mut surface = wrap_render_target(
            &mut self.gr_context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
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
        #[cfg(target_os = "ios")]
        {
            let scale = self
                .view
                .window()
                .map(|window| window.screen().scale())
                .unwrap_or(1.0);
            self.sync_view_geometry(scale);
        }

        self.metal_layer
            .setDrawableSize(CGSize::new(size.width as f64, size.height as f64));
    }
}
