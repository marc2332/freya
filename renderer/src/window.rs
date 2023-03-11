use freya_common::{EventMessage, NodeArea};

use freya_core::{events::EventsProcessor, process_render, EventEmitter, EventsQueue};
use freya_core::{process_events, process_layout, ViewportsCollection};
use freya_layout::{DioxusDOM, Layers};
use gl::types::*;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{window::WindowBuilder, GlProfile};
use skia_safe::{gpu::DirectContext, textlayout::FontCollection};
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    ColorType, Surface,
};
use skia_safe::{Color, FontMgr};

use crate::renderer::render_skia;
use crate::window_config::WindowConfig;
use crate::HoveredNode;

type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

/// Manager for a Window
pub struct WindowEnv<T: Clone> {
    pub(crate) surface: Surface,
    pub(crate) gr_context: DirectContext,
    pub(crate) windowed_context: WindowedContext,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) font_collection: FontCollection,
    pub(crate) window_config: WindowConfig<T>,
}

impl<T: Clone> WindowEnv<T> {
    /// Create a Window environment from a set of configuration
    pub fn from_config(
        window_config: WindowConfig<T>,
        event_loop: &EventLoop<EventMessage>,
    ) -> Self {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");

        let wb = WindowBuilder::new()
            .with_title(window_config.title)
            .with_decorations(window_config.decorations)
            .with_transparent(window_config.transparent)
            .with_inner_size(PhysicalSize::<u32>::new(
                window_config.width,
                window_config.height,
            ));

        let cb = glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_stencil_buffer(8)
            .with_pixel_format(24, 8)
            .with_gl_profile(GlProfile::Core);

        #[cfg(not(target_os = "linux"))]
        let cb = cb.with_double_buffer(Some(true));

        let windowed_context = cb.build_windowed(wb, event_loop).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        gl::load_with(|s| windowed_context.get_proc_address(s));

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            }
        };

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

        let mut surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
        let sf = windowed_context.window().scale_factor() as f32;
        surface.canvas().scale((sf, sf));

        WindowEnv {
            surface,
            gr_context,
            windowed_context,
            fb_info,
            font_collection,
            window_config,
        }
    }

    // Process the events and emit them to the RealDOM
    pub fn process_events(
        &mut self,
        rdom: &DioxusDOM,
        layers: &Layers,
        events: &mut EventsQueue,
        event_emitter: &EventEmitter,
        events_processor: &mut EventsProcessor,
        viewports_collection: &ViewportsCollection,
    ) {
        process_events(
            rdom,
            layers,
            events,
            event_emitter,
            events_processor,
            viewports_collection,
        );
    }

    // Reprocess the layout
    pub fn process_layout(&mut self, rdom: &DioxusDOM) -> (Layers, ViewportsCollection) {
        let window_size = self.windowed_context.window().inner_size();
        process_layout(
            rdom,
            NodeArea {
                width: window_size.width as f32,
                height: window_size.height as f32,
                x: 0.0,
                y: 0.0,
            },
            &mut self.font_collection,
        )
    }

    /// Redraw the window
    pub fn render(
        &mut self,
        layers: &Layers,
        viewports_collection: &ViewportsCollection,
        hovered_node: &HoveredNode,
        rdom: &DioxusDOM,
    ) {
        let canvas = self.surface.canvas();

        canvas.clear(if self.window_config.decorations {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        });

        process_render(
            viewports_collection,
            rdom,
            &mut self.font_collection,
            layers,
            canvas,
            |dom, element, font_collection, viewports_collection, canvas| {
                canvas.save();
                let render_wireframe = if let Some(hovered_node) = &hovered_node {
                    hovered_node
                        .lock()
                        .unwrap()
                        .map(|id| id == element.node_id)
                        .unwrap_or_default()
                } else {
                    false
                };
                render_skia(
                    dom,
                    canvas,
                    element,
                    font_collection,
                    viewports_collection,
                    render_wireframe,
                );
                canvas.restore();
            },
        );

        self.gr_context.flush(None);
        self.windowed_context.swap_buffers().unwrap();
    }

    pub fn request_redraw(&self) {
        self.windowed_context.window().request_redraw()
    }
}

/// Create the surface for Skia to render in
pub fn create_surface(
    windowed_context: &WindowedContext,
    fb_info: &FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
) -> Surface {
    let pixel_format = windowed_context.get_pixel_format();
    let size = windowed_context.window().inner_size();
    let backend_render_target = BackendRenderTarget::new_gl(
        (
            size.width.try_into().unwrap(),
            size.height.try_into().unwrap(),
        ),
        pixel_format.multisampling.map(|s| s.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        *fb_info,
    );
    Surface::from_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .unwrap()
}
