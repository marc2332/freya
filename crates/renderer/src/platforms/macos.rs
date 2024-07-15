use std::mem;

use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use dioxus_core::VirtualDom;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{CommandQueue, Device, MTLPixelFormat, MetalLayer};
use objc::runtime::YES;
use winit::{
    dpi::LogicalSize,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    raw_window_handle::HasWindowHandle,
    window::Window,
};

use freya_common::EventMessage;
use freya_core::dom::SafeDOM;
use freya_engine::prelude::*;

use crate::{app::Application, config::WindowConfig, devtools::Devtools, LaunchConfig};

pub struct NotCreatedState<'a, State: Clone + 'static> {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) devtools: Option<Devtools>,
    pub(crate) config: LaunchConfig<'a, State>,
}

pub struct CreatedState {
    pub(crate) gr_context: DirectContext,
    pub(crate) metal_layer: MetalLayer,
    pub(crate) command_queue: CommandQueue,
    pub(crate) window: Window,
    pub(crate) window_config: WindowConfig,
    pub(crate) app: Application,
    pub(crate) is_window_focused: bool,
}

pub enum WindowState<'a, State: Clone + 'static> {
    NotCreated(NotCreatedState<'a, State>),
    Creating,
    Created(CreatedState),
}

impl<'a, State: Clone + 'a> WindowState<'a, State> {
    pub fn created_state(&mut self) -> &mut CreatedState {
        let Self::Created(created) = self else {
            panic!("Unexpected.")
        };
        created
    }

    pub fn has_been_created(&self) -> bool {
        matches!(self, Self::Created(..))
    }

    pub fn create(
        &mut self,
        event_loop: &ActiveEventLoop,
        event_loop_proxy: &EventLoopProxy<EventMessage>,
    ) {
        let Self::NotCreated(NotCreatedState {
            sdom,
            vdom,
            devtools,
            mut config,
        }) = mem::replace(self, WindowState::Creating)
        else {
            panic!("Unexpected.")
        };

        let mut window_attributes = Window::default_attributes()
            .with_visible(false)
            .with_title(config.window_config.title)
            .with_decorations(config.window_config.decorations)
            .with_transparent(config.window_config.transparent)
            .with_window_icon(config.window_config.icon.take())
            .with_inner_size(LogicalSize::<f64>::from(config.window_config.size));

        set_resource_cache_total_bytes_limit(1000000); // 1MB
        set_resource_cache_single_allocation_byte_limit(Some(500000)); // 0.5MB

        if let Some(min_size) = config.window_config.min_size {
            window_attributes =
                window_attributes.with_min_inner_size(LogicalSize::<f64>::from(min_size));
        }
        
        if let Some(max_size) = config.window_config.max_size {
            window_attributes =
                window_attributes.with_max_inner_size(LogicalSize::<f64>::from(max_size));
        }

        if let Some(with_window_attributes) = &config.window_config.window_attributes_hook {
            window_attributes = (with_window_attributes)(window_attributes);
        }

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create Window");

        let window_handle = window
            .window_handle()
            .expect("Failed to retrieve a window handle");

        let raw_window_handle = window_handle.as_raw();

        let device = Device::system_default().expect("no device found");
	    let scale_factor = window.scale_factor();

        // Allow IME
        window.set_ime_allowed(true);

        // Mak the window visible once built
        window.set_visible(true);
	    
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

        let command_queue = device.new_command_queue();

        let backend = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                command_queue.as_ptr() as mtl::Handle,
            )
        };

        let mut gr_context =
            direct_contexts::make_metal(&backend, None).expect("Could not create direct context");

        let mut app = Application::new(
            sdom,
            vdom,
            event_loop_proxy,
            devtools,
            &window,
            config.embedded_fonts,
            config.plugins,
            config.default_fonts,
        );

        app.init_doms(scale_factor as f32, config.state.clone());
        app.process_layout(window.inner_size(), scale_factor);

        *self = WindowState::Created(CreatedState {
            gr_context,
            metal_layer,
            command_queue,
            window,
            app,
            window_config: config.window_config,
            is_window_focused: false,
        });
    }
}

/// Create the surface for Skia to render in
pub fn create_surface(metal_layer: &MetalLayer, skia: &mut DirectContext) -> Surface {
    unsafe {
        let drawable = metal_layer.next_drawable().expect("no drawable found");
        let (drawable_width, drawable_height) = {
            let size = metal_layer.drawable_size();
            (size.width as scalar, size.height as scalar)
        };
        let texture_info = mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);

        let backend_render_target = backend_render_targets::make_mtl(
            (drawable_width as i32, drawable_height as i32),
            &texture_info,
        );

        wrap_backend_render_target(
            skia,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            None,
            None,
        )
        .expect("Could not create skia surface")
    }
}
