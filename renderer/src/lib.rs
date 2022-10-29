use dioxus_core::{exports::futures_channel::mpsc::UnboundedSender, SchedulerMsg};
use dioxus_native_core::real_dom::RealDom;
use freya_layout_common::LayoutMemorizer;
use freya_node_state::NodeState;
use glutin::event::ElementState;
use glutin::window::WindowId;
use glutin::{event::Event, event_loop::ControlFlow};
use glutin::{
    event::{KeyEvent, MouseScrollDelta, TouchPhase, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
};
use skia_safe::{textlayout::FontCollection, FontMgr};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
pub use window::{create_surface, create_windows_from_config, WindowConfig, WindowEnv};

mod events_processor;
mod renderer;
mod window;
mod work_loop;

type SafeDOM = Arc<Mutex<RealDom<NodeState>>>;
type SafeEventEmitter = Arc<Mutex<Option<UnboundedSender<SchedulerMsg>>>>;
type SafeLayoutManager = Arc<Mutex<LayoutMemorizer>>;
type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
pub type SafeFreyaEvents = Arc<Mutex<Vec<FreyaEvents>>>;

/// Events emitted in Freya
#[derive(Clone, Debug)]
pub enum FreyaEvents {
    /// A Mouse Event
    MouseEvent {
        name: &'static str,
        cursor: (f64, f64),
    },
    /// A Wheel event
    WheelEvent {
        name: &'static str,
        scroll: (f64, f64),
        cursor: (f64, f64),
    },
    /// A Keyboard event
    KeyboardEvent {
        name: &'static str,
        code: Key<'static>,
    },
}

/// Run the Windows Event Loop
pub fn run(windows_config: Vec<(SafeDOM, SafeEventEmitter, SafeLayoutManager, WindowConfig)>) {
    let cursor_pos = Arc::new(Mutex::new((0.0, 0.0)));
    let event_loop = EventLoop::<WindowId>::with_user_event();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");

    let wins = create_windows_from_config(windows_config, &event_loop, font_collection);

    let get_window_env = move |window_id: WindowId| -> Option<Arc<Mutex<WindowEnv>>> {
        let mut win = None;
        for env in &*wins.lock().unwrap() {
            if env.lock().unwrap().windowed_context.window().id() == window_id {
                win = Some(env.clone())
            }
        }

        win
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent {
                event, window_id, ..
            } => {
                let result = get_window_env(window_id);
                if let Some(result) = result {
                    let mut env = result.lock().unwrap();
                    match event {
                        WindowEvent::MouseWheel { delta, phase, .. } => {
                            if TouchPhase::Moved == phase {
                                let cursor_pos = cursor_pos.lock().unwrap();
                                let scroll_data = {
                                    match delta {
                                        MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                                        MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                                        _ => (0.0, 0.0),
                                    }
                                };

                                env.freya_events
                                    .lock()
                                    .unwrap()
                                    .push(FreyaEvents::WheelEvent {
                                        name: "wheel",
                                        scroll: scroll_data,
                                        cursor: *cursor_pos,
                                    });
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let cursor_pos = {
                                let mut cursor_pos = cursor_pos.lock().unwrap();
                                cursor_pos.0 = position.x;
                                cursor_pos.1 = position.y;

                                *cursor_pos
                            };

                            env.freya_events
                                .lock()
                                .unwrap()
                                .push(FreyaEvents::MouseEvent {
                                    name: "mouseover",
                                    cursor: cursor_pos,
                                });
                        }
                        WindowEvent::MouseInput { state, .. } => {
                            let event_name = match state {
                                ElementState::Pressed => "mousedown",
                                ElementState::Released => "click",
                                _ => "mousedown",
                            };
                            let cursor_pos = cursor_pos.lock().unwrap();
                            env.freya_events
                                .lock()
                                .unwrap()
                                .push(FreyaEvents::MouseEvent {
                                    name: event_name,
                                    cursor: *cursor_pos,
                                });
                        }
                        WindowEvent::Resized(physical_size) => {
                            *env.is_resizing.lock().unwrap() = true;
                            let mut context = env.gr_context.clone();
                            env.surface =
                                create_surface(&env.windowed_context, &env.fb_info, &mut context);
                            env.windowed_context.resize(physical_size);
                            *env.resizing_timer.lock().unwrap() = Instant::now();
                            env.layout_memorizer.lock().unwrap().dirty_nodes.clear();
                            env.layout_memorizer.lock().unwrap().nodes.clear();
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

                            env.freya_events
                                .lock()
                                .unwrap()
                                .push(FreyaEvents::KeyboardEvent {
                                    name: event_name,
                                    code: logical_key,
                                });
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) => {
                let result = get_window_env(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            Event::UserEvent(window_id) => {
                let result = get_window_env(window_id);
                if let Some(env) = result {
                    let mut env = env.lock().unwrap();
                    env.redraw();
                }
            }
            _ => (),
        }
    });
}
