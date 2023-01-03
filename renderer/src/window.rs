use freya_common::NodeArea;
use freya_processor::{
    events::EventsProcessor, process_work, SafeDOM, SafeEventEmitter, SafeFreyaEvents,
    SafeLayoutMemorizer,
};
use gl::types::*;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{window::WindowBuilder, GlProfile};
use skia_safe::Color;
use skia_safe::{gpu::DirectContext, textlayout::FontCollection};
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    ColorType, Surface,
};
use std::sync::{Arc, Mutex};

use crate::renderer::render_skia;
use crate::window_config::WindowConfig;

type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

/// Information related to a specific window
pub struct WindowEnv<T: Clone> {
    pub(crate) surface: Surface,
    pub(crate) gr_context: DirectContext,
    pub(crate) windowed_context: WindowedContext,
    pub(crate) fb_info: FramebufferInfo,
    pub(crate) rdom: SafeDOM,
    pub(crate) layout_memorizer: SafeLayoutMemorizer,
    pub(crate) freya_events: SafeFreyaEvents,
    pub(crate) event_emitter: SafeEventEmitter,
    pub(crate) font_collection: FontCollection,
    pub(crate) events_processor: EventsProcessor,
    pub(crate) window_config: WindowConfig<T>,
}

impl<T: Clone> WindowEnv<T> {
    pub fn from_config(
        rdom: &SafeDOM,
        event_emitter: SafeEventEmitter,
        layout_memorizer: &SafeLayoutMemorizer,
        window_config: WindowConfig<T>,
        event_loop: &EventLoop<()>,
        font_collection: FontCollection,
    ) -> Self {
        let events_processor = EventsProcessor::default();
        let freya_events = Arc::new(Mutex::new(Vec::new()));
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

        #[cfg(not(feature = "wayland"))]
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
            rdom: rdom.clone(),
            freya_events,
            event_emitter,
            font_collection,
            events_processor,
            window_config,
            layout_memorizer: layout_memorizer.clone(),
        }
    }

    pub fn redraw(&mut self) {
        let canvas = self.surface.canvas();

        canvas.clear(if self.window_config.decorations {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        });

        let window_size = self.windowed_context.window().inner_size();

        process_work(
            &self.rdom,
            NodeArea {
                width: window_size.width as f32,
                height: window_size.height as f32,
                x: 0.0,
                y: 0.0,
            },
            self.freya_events.clone(),
            &self.event_emitter,
            &mut self.font_collection,
            &mut self.events_processor,
            &self.layout_memorizer,
            canvas,
            |dom, element, font_collection, viewports_collection, canvas| {
                canvas.save();
                render_skia(dom, canvas, element, font_collection, viewports_collection);
                canvas.restore();
            },
        );

        self.gr_context.flush(None);
        self.windowed_context.swap_buffers().unwrap();
    }
}

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
