use dioxus_core::{exports::futures_channel::mpsc::UnboundedSender, SchedulerMsg};
use dioxus_native_core::real_dom::RealDom;
use freya_layers::NodeArea;
use freya_node_state::node::NodeState;
use glutin::{
    event::{KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
};
use skia_safe::{textlayout::FontCollection, FontMgr};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use gl::types::*;
use glutin::dpi::PhysicalSize;
use glutin::event::ElementState;
use glutin::window::WindowId;
use glutin::{event::Event, event_loop::ControlFlow, window::WindowBuilder, GlProfile};
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
        cursor: (f64, f64),
    },
    WheelEvent {
        name: &'static str,
        scroll: (f64, f64),
        cursor: (f64, f64),
    },
    #[allow(dead_code)]
    KeyboardEvent {
        name: &'static str,
        code: Key<'static>,
    },
}

struct WindowEnv {
    surface: Surface,
    gr_context: skia_safe::gpu::DirectContext,
    windowed_context: WindowedContext,
    skia_dom: SkiaDom,
    fb_info: FramebufferInfo,
    renderer_requests: RendererRequests,
    event_emitter: EventEmitter,
    font_collection: FontCollection,
    events_processor: EventsProcessor,
    is_resizing: Arc<Mutex<bool>>,
    resizing_timer: Arc<Mutex<Instant>>,
    win_config: WindowConfig,
}

impl WindowEnv {
    pub fn redraw(&mut self) {
        let canvas = self.surface.canvas();
        canvas.clear(if self.win_config.decorations {
            Color::WHITE
        } else {
            Color::TRANSPARENT
        });
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

#[derive(Clone)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub decorations: bool,
    pub title: &'static str,
    pub transparent: bool,
}

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

fn create_windows_from_config(
    windows_config: Vec<(SkiaDom, EventEmitter, WindowConfig)>,
    el: &EventLoop<WindowId>,
    font_collection: FontCollection,
) -> Arc<Mutex<Vec<Arc<Mutex<WindowEnv>>>>> {
    let wins = Arc::new(Mutex::new(vec![]));

    for (skia_dom, event_emitter, win_config) in windows_config {
        let events_processor = EventsProcessor::default();
        let renderer_requests: RendererRequests = Arc::new(Mutex::new(Vec::new()));
        let wb = WindowBuilder::new()
            .with_title(win_config.title)
            .with_decorations(win_config.decorations)
            .with_transparent(win_config.transparent);

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

        windowed_context
            .window()
            .set_inner_size(PhysicalSize::<u32>::new(
                win_config.width,
                win_config.height,
            ));

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

        let env = WindowEnv {
            surface,
            gr_context,
            windowed_context,
            fb_info,
            skia_dom,
            renderer_requests: renderer_requests.clone(),
            event_emitter,
            font_collection: font_collection.clone(),
            events_processor,
            is_resizing: Arc::new(Mutex::new(false)),
            resizing_timer: Arc::new(Mutex::new(Instant::now())),
            win_config,
        };

        let proxy = el.create_proxy();
        let is_resizing = env.is_resizing.clone();
        let resize_timer = env.resizing_timer.clone();
        thread::spawn(move || {
            let time = 1000;
            #[cfg(target_os = "windows")]
            let fps = 120; // Seems like Windows needs more renderings to feel equally faster
            #[cfg(not(target_os = "windows"))]
            let fps = 60;

            let step = time / fps;
            loop {
                if *is_resizing.lock().unwrap() == false {
                    // Trigger redraw
                    proxy.send_event(window_id).unwrap();
                    thread::sleep(Duration::from_millis(step));
                }
                if resize_timer.lock().unwrap().elapsed().as_millis() > 50 {
                    *is_resizing.lock().unwrap() = false;
                }
            }
        });

        wins.lock().unwrap().push(Arc::new(Mutex::new(env)));
    }

    wins
}

pub fn run(windows_config: Vec<(SkiaDom, EventEmitter, WindowConfig)>) {
    let cursor_pos = Arc::new(Mutex::new((0.0, 0.0)));
    let el = EventLoop::<WindowId>::with_user_event();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");

    let wins = create_windows_from_config(windows_config, &el, font_collection);

    let get_window_context = move |window_id: WindowId| -> Option<Arc<Mutex<WindowEnv>>> {
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
            Event::WindowEvent {
                event, window_id, ..
            } => {
                let result = get_window_context(window_id);
                if let Some(result) = result {
                    let mut env = result.lock().unwrap();
                    match event {
                        WindowEvent::MouseWheel { delta, .. } => {
                            let cursor_pos = cursor_pos.lock().unwrap();
                            let scroll_data = {
                                match delta {
                                    MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                                    MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                                    _ => (0.0, 0.0),
                                }
                            };
                            env.renderer_requests.lock().unwrap().push(
                                RendererRequest::WheelEvent {
                                    name: "wheel",
                                    scroll: scroll_data,
                                    cursor: *cursor_pos,
                                },
                            );
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let cursor_pos = {
                                let mut cursor_pos = cursor_pos.lock().unwrap();
                                cursor_pos.0 = position.x;
                                cursor_pos.1 = position.y;

                                *cursor_pos
                            };

                            env.renderer_requests.lock().unwrap().push(
                                RendererRequest::MouseEvent {
                                    name: "mouseover",
                                    cursor: cursor_pos,
                                },
                            );
                        }
                        WindowEvent::MouseInput { state, .. } => {
                            let event_name = match state {
                                ElementState::Pressed => "mousedown",
                                ElementState::Released => "click",
                                _ => "mousedown",
                            };
                            let cursor_pos = cursor_pos.lock().unwrap();
                            env.renderer_requests.lock().unwrap().push(
                                RendererRequest::MouseEvent {
                                    name: event_name,
                                    cursor: *cursor_pos,
                                },
                            );
                        }
                        WindowEvent::Resized(physical_size) => {
                            *env.is_resizing.lock().unwrap() = true;
                            let mut context = env.gr_context.clone();
                            env.surface =
                                create_surface(&env.windowed_context, &env.fb_info, &mut context);
                            env.windowed_context.resize(physical_size);
                            *env.resizing_timer.lock().unwrap() = Instant::now();
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key, state, ..
                                },
                            ..
                        } => {
                            let event_name = match state {
                                ElementState::Pressed => "keydown",
                                ElementState::Released => "keyup",
                                _ => "keydown",
                            };

                            env.renderer_requests.lock().unwrap().push(
                                RendererRequest::KeyboardEvent {
                                    name: event_name,
                                    code: logical_key,
                                },
                            );
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) => {
                let result = get_window_context(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            Event::UserEvent(window_id) => {
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
