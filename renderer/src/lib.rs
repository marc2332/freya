use dioxus_core::VirtualDom;
use dioxus_native_core::real_dom::RealDom;
use dioxus_native_core::SendAnyMap;
use freya_common::LayoutMemorizer;
use freya_node_state::{CustomAttributeValues, NodeState};
use freya_processor::events::FreyaEvent;
use freya_processor::{DomEvent, SafeDOM};
use futures::task::ArcWake;
use futures::{pin_mut, task, FutureExt};
use glutin::event::{ElementState, StartCause};
use glutin::event_loop::EventLoopProxy;
use glutin::{event::Event, event_loop::ControlFlow};
use glutin::{
    event::{KeyEvent, MouseScrollDelta, TouchPhase, WindowEvent},
    event_loop::EventLoop,
};
use skia_safe::{textlayout::FontCollection, FontMgr};
use std::sync::{Arc, Mutex};
use std::task::Waker;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
pub use window::{create_surface, WindowEnv};
pub use window_config::WindowConfig;

mod renderer;
mod window;
mod window_config;

/// Start the Tao event loop with the virtual dom polling
pub fn run<T: 'static + Clone>(
    mut vdom: VirtualDom,
    rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    window_config: WindowConfig<T>,
) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _guard = rt.enter();

    let event_loop = EventLoop::<()>::with_user_event();
    let (event_emitter, mut event_emitter_rx) = unbounded_channel::<DomEvent>();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");
    let layout_memorizer = Arc::new(Mutex::new(LayoutMemorizer::new()));
    let app_state = window_config.state.clone();

    if let Some(state) = &app_state {
        vdom.base_scope().provide_context(state.clone());
    }

    let muts = vdom.rebuild();
    let (to_update, _) = rdom.lock().unwrap().apply_mutations(muts);

    let mut ctx = SendAnyMap::new();
    ctx.insert(layout_memorizer.clone());
    rdom.lock().unwrap().update_state(to_update, ctx);

    let mut window_env = WindowEnv::from_config(
        &rdom,
        event_emitter,
        &layout_memorizer,
        window_config,
        &event_loop,
        font_collection,
    );

    let proxy = event_loop.create_proxy();
    let waker = tao_waker(&proxy);
    let cursor_pos = Arc::new(Mutex::new((0.0, 0.0)));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                window_env.redraw();
            }
            Event::NewEvents(StartCause::Init) => {
                _ = proxy.send_event(());
            }
            Event::UserEvent(_s) => {
                poll_vdom(
                    &waker,
                    &mut vdom,
                    &rdom,
                    &layout_memorizer,
                    &mut event_emitter_rx,
                    &app_state,
                );
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput { state, button, .. } => {
                    let event_name = match state {
                        ElementState::Pressed => "mousedown",
                        ElementState::Released => "click",
                        _ => "mousedown",
                    };
                    let cursor_pos = cursor_pos.lock().unwrap();
                    window_env
                        .freya_events
                        .lock()
                        .unwrap()
                        .push(FreyaEvent::Mouse {
                            name: event_name,
                            cursor: *cursor_pos,
                            button: Some(button),
                        });
                }
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

                        window_env
                            .freya_events
                            .lock()
                            .unwrap()
                            .push(FreyaEvent::Wheel {
                                name: "wheel",
                                scroll: scroll_data,
                                cursor: *cursor_pos,
                            });
                    }
                }
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

                    window_env
                        .freya_events
                        .lock()
                        .unwrap()
                        .push(FreyaEvent::Keyboard {
                            name: event_name,
                            code: logical_key,
                        });
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let cursor_pos = {
                        let mut cursor_pos = cursor_pos.lock().unwrap();
                        cursor_pos.0 = position.x;
                        cursor_pos.1 = position.y;

                        *cursor_pos
                    };

                    window_env
                        .freya_events
                        .lock()
                        .unwrap()
                        .push(FreyaEvent::Mouse {
                            name: "mouseover",
                            cursor: cursor_pos,
                            button: None,
                        });
                }
                WindowEvent::Resized(size) => {
                    let mut context = window_env.gr_context.clone();
                    window_env.surface = create_surface(
                        &window_env.windowed_context,
                        &window_env.fb_info,
                        &mut context,
                    );
                    window_env.windowed_context.resize(size);
                    window_env
                        .layout_memorizer
                        .lock()
                        .unwrap()
                        .dirty_nodes
                        .clear();
                    window_env.layout_memorizer.lock().unwrap().nodes.clear();
                }
                _ => {}
            },
            Event::LoopDestroyed => {}
            _ => (),
        }
    });
}

pub fn tao_waker(proxy: &EventLoopProxy<()>) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<()>);

    // this should be implemented by most platforms, but ios is missing this until
    // https://github.com/tauri-apps/wry/issues/830 is resolved
    unsafe impl Send for DomHandle {}
    unsafe impl Sync for DomHandle {}

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(());
        }
    }

    task::waker(Arc::new(DomHandle(proxy.clone())))
}

fn poll_vdom<T: 'static + Clone>(
    waker: &Waker,
    vdom: &mut VirtualDom,
    rdom: &Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    layout_memorizer: &Arc<Mutex<LayoutMemorizer>>,
    event_emitter_rx: &mut UnboundedReceiver<DomEvent>,
    state: &Option<T>,
) {
    let mut cx = std::task::Context::from_waker(waker);

    loop {
        if let Some(state) = state.clone() {
            vdom.base_scope().provide_context(state);
        }

        {
            let fut = async {
                select! {
                    ev = event_emitter_rx.recv() => {
                        if let Some(ev) = ev {
                            let data = ev.data.any();
                            vdom.handle_event(&ev.name, data, ev.element_id, false);

                            vdom.process_events();
                        }
                    },
                    _ = vdom.wait_for_work() => {},
                }
            };
            pin_mut!(fut);

            match fut.poll_unpin(&mut cx) {
                std::task::Poll::Ready(_) => {}
                std::task::Poll::Pending => break,
            }
        }

        let mutations = vdom.render_immediate();
        let (to_update, _diff) = rdom.lock().unwrap().apply_mutations(mutations);

        let mut ctx = SendAnyMap::new();
        ctx.insert(layout_memorizer.clone());
        rdom.lock().unwrap().update_state(to_update, ctx);
    }
}
