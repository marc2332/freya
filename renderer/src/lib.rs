use dioxus_core::{exports::futures_channel::mpsc::UnboundedSender, SchedulerMsg};
use dioxus_html::{
    geometry::{
        euclid::{Length, Point2D},
        Coordinates,
    },
    input_data::{keyboard_types::Modifiers, MouseButton},
    on::{KeyboardData, MouseData},
};
use dioxus_native_core::real_dom::RealDom;
use enumset::enum_set;
use glutin::event::{MouseScrollDelta, WindowEvent};
use layers_engine::NodeArea;
use skia_safe::{textlayout::FontCollection, FontMgr};
use state::node::NodeState;
use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use gl::types::*;
use glutin::dpi::PhysicalSize;
use glutin::event::ElementState;
use glutin::window::WindowId;
use glutin::{
    event::{Event, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    GlProfile,
};
use skia_safe::Color;
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    ColorType, Surface,
};

mod events_processor;
mod renderer;
mod work_loop;

use work_loop::work_loop;

use crate::events_processor::EventsProcessor;

type SkiaDom = Arc<Mutex<RealDom<NodeState>>>;
type EventEmitter = Arc<Mutex<Option<UnboundedSender<SchedulerMsg>>>>;
type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
pub type RendererRequests = Arc<Mutex<Vec<RendererRequest>>>;

#[derive(Clone, Debug)]
pub enum RendererRequest {
    MouseEvent {
        name: &'static str,
        event: MouseData,
    },
    #[allow(dead_code)]
    KeyboardEvent {
        name: &'static str,
        event: KeyboardData,
    },
}

pub fn run(skia_dom: SkiaDom, rev_render: Receiver<()>, event_emitter: EventEmitter) {
    let renderer_requests: RendererRequests = Arc::new(Mutex::new(Vec::new()));
    let cursor_pos = Arc::new(Mutex::new((0.0, 0.0)));
    let events_processor = EventsProcessor::default();

    let el = EventLoop::new();

    struct Env {
        surface: Surface,
        gr_context: skia_safe::gpu::DirectContext,
        windowed_context: WindowedContext,
        skia_dom: SkiaDom,
        fb_info: FramebufferInfo,
        renderer_requests: RendererRequests,
        event_emitter: EventEmitter,
        font_collection: FontCollection,
        events_processor: EventsProcessor,
    }

    impl Env {
        pub fn redraw(&mut self) {
            let canvas = self.surface.canvas();
            canvas.clear(Color::WHITE);
            let window_size = self.windowed_context.window().inner_size();
            work_loop(
                &self.skia_dom,
                canvas,
                NodeArea {
                    width: window_size.width as f32,
                    height: window_size.height as f32,
                    x: 0.0,
                    y: 0.0,
                },
                self.renderer_requests.clone(),
                &self.event_emitter,
                &mut self.font_collection,
                &mut self.events_processor,
            );
            self.gr_context.flush(None);
            self.windowed_context.swap_buffers().unwrap();
        }
    }

    let wins = Arc::new(Mutex::new(vec![]));

    let wb = WindowBuilder::new().with_title("test");

    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_gl_profile(GlProfile::Core);

    #[cfg(not(feature = "wayland"))]
    let cb = cb.with_double_buffer(Some(true));

    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let window_id = windowed_context.window().id();

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

    windowed_context
        .window()
        .set_inner_size(PhysicalSize::<u32>::new(300, 300));

    let mut surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
    let sf = windowed_context.window().scale_factor() as f32;
    surface.canvas().scale((sf, sf));

    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");

    let env = Env {
        surface,
        gr_context,
        windowed_context,
        fb_info,
        skia_dom,
        renderer_requests: renderer_requests.clone(),
        event_emitter,
        font_collection,
        events_processor,
    };

    wins.lock().unwrap().push(Arc::new(Mutex::new(env)));

    fn create_surface(
        windowed_context: &WindowedContext,
        fb_info: &FramebufferInfo,
        gr_context: &mut skia_safe::gpu::DirectContext,
    ) -> skia_safe::Surface {
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

    {
        let proxy = el.create_proxy();
        thread::spawn(move || {
            while let Ok(msg) = rev_render.recv() {
                proxy.send_event(msg).unwrap();
            }
        });
    }

    let get_window_context = move |window_id: WindowId| -> Option<Arc<Mutex<Env>>> {
        let mut win = None;
        for env in &*wins.lock().unwrap() {
            if env.lock().unwrap().windowed_context.window().id() == window_id {
                win = Some(env.clone())
            }
        }

        win
    };

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, window_id } => {
                let result = get_window_context(window_id);
                if let Some(env) = result {
                    let env = env.lock().unwrap();
                    env.windowed_context.window().request_redraw();
                }
                match event {
                    WindowEvent::MouseWheel { delta, .. } => {
                        let cursor_pos = cursor_pos.lock().unwrap();
                        let scroll_data = {
                            match delta {
                                MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                                MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                            }
                        };
                        renderer_requests
                            .lock()
                            .unwrap()
                            .push(RendererRequest::MouseEvent {
                                name: "scroll",
                                event: MouseData::new(
                                    Coordinates::new(
                                        Point2D::default(),
                                        Point2D::from_lengths(
                                            Length::new(cursor_pos.0),
                                            Length::new(cursor_pos.1),
                                        ),
                                        Point2D::default(),
                                        Point2D::from_lengths(
                                            Length::new(scroll_data.0),
                                            Length::new(scroll_data.1),
                                        ),
                                    ),
                                    Some(MouseButton::Primary),
                                    enum_set! {MouseButton::Primary},
                                    Modifiers::empty(),
                                ),
                            });
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let cursor_pos = {
                            let mut cursor_pos = cursor_pos.lock().unwrap();
                            cursor_pos.0 = position.x;
                            cursor_pos.1 = position.y;

                            *cursor_pos
                        };

                        renderer_requests
                            .lock()
                            .unwrap()
                            .push(RendererRequest::MouseEvent {
                                name: "mouseover",
                                event: MouseData::new(
                                    Coordinates::new(
                                        Point2D::default(),
                                        Point2D::from_lengths(
                                            Length::new(cursor_pos.0),
                                            Length::new(cursor_pos.1),
                                        ),
                                        Point2D::default(),
                                        Point2D::default(),
                                    ),
                                    Some(MouseButton::Primary),
                                    enum_set! {MouseButton::Primary},
                                    Modifiers::empty(),
                                ),
                            });
                    }
                    WindowEvent::MouseInput { state, .. } => {
                        let event_name = match state {
                            ElementState::Pressed => "mousedown",
                            ElementState::Released => "click",
                        };
                        let cursor_pos = cursor_pos.lock().unwrap();
                        renderer_requests
                            .lock()
                            .unwrap()
                            .push(RendererRequest::MouseEvent {
                                name: event_name,
                                event: MouseData::new(
                                    Coordinates::new(
                                        Point2D::default(),
                                        Point2D::from_lengths(
                                            Length::new(cursor_pos.0),
                                            Length::new(cursor_pos.1),
                                        ),
                                        Point2D::default(),
                                        Point2D::default(),
                                    ),
                                    Some(MouseButton::Primary),
                                    enum_set! {MouseButton::Primary},
                                    Modifiers::empty(),
                                ),
                            });
                    }
                    WindowEvent::Resized(physical_size) => {
                        let result = get_window_context(window_id);
                        if let Some(env) = result {
                            let mut env = env.lock().unwrap();
                            let mut context = env.gr_context.clone();
                            env.surface =
                                create_surface(&env.windowed_context, &env.fb_info, &mut context);
                            env.windowed_context.resize(physical_size)
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        if modifiers.logo() {
                            if let Some(VirtualKeyCode::Q) = virtual_keycode {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                    }
                    _ => (),
                }
            }
            Event::RedrawRequested(window_id) => {
                let result = get_window_context(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            Event::UserEvent(_) => {
                let result = get_window_context(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            _ => (),
        }
    });
}
