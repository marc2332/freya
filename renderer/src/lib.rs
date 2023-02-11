use dioxus_core::VirtualDom;
use dioxus_native_core::real_dom::RealDom;
use dioxus_native_core::{NodeId, SendAnyMap};
use freya_core::events::FreyaEvent;
use freya_core::{events::DomEvent, SharedRealDOM};
use freya_elements::{from_winit_to_code, get_modifiers, get_non_text_keys, Code, Key};
use freya_node_state::{CustomAttributeValues, NodeState};
use futures::task::ArcWake;
use futures::{pin_mut, task, FutureExt};
use glutin::event::{
    ElementState, Event, KeyboardInput, ModifiersState, MouseScrollDelta, StartCause, TouchPhase,
    WindowEvent,
};
use glutin::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use skia_safe::{textlayout::FontCollection, FontMgr};
use std::sync::{Arc, Mutex};
use std::task::Waker;
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
pub use window::{create_surface, WindowEnv};
pub use window_config::WindowConfig;

mod wireframe;

mod elements;
mod renderer;
mod window;
mod window_config;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;

/// Start the winit event loop with the virtual dom polling
pub fn run<T: 'static + Clone>(
    mut vdom: VirtualDom,
    rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    window_config: WindowConfig<T>,
    mutations_sender: Option<UnboundedSender<()>>,
    hovered_node: HoveredNode,
) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _guard = rt.enter();

    let event_loop = EventLoopBuilder::with_user_event().build();
    let (event_emitter, mut event_emitter_rx) = unbounded_channel::<DomEvent>();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), "Fira Sans");
    let app_state = window_config.state.clone();

    if let Some(state) = &app_state {
        vdom.base_scope().provide_context(state.clone());
    }

    let muts = vdom.rebuild();
    let (to_update, diff) = rdom.lock().unwrap().apply_mutations(muts);

    if !diff.is_empty() {
        mutations_sender.as_ref().map(|s| s.send(()));
    }

    let ctx = SendAnyMap::new();
    rdom.lock().unwrap().update_state(to_update, ctx);

    let mut window_env = WindowEnv::from_config(
        &rdom,
        event_emitter,
        window_config,
        &event_loop,
        font_collection,
    );

    let proxy = event_loop.create_proxy();
    let waker = winit_waker(&proxy);
    let cursor_pos = Arc::new(Mutex::new((0.0, 0.0)));

    let mut last_keydown = Key::Unidentified;
    let mut last_code = Code::Unidentified;
    let mut modifiers_state = ModifiersState::empty();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::NewEvents(StartCause::Init) => {
                _ = proxy.send_event(());
            }
            Event::UserEvent(()) => {
                poll_vdom(
                    &waker,
                    &mut vdom,
                    &rdom,
                    &mut event_emitter_rx,
                    &app_state,
                    &mutations_sender,
                );

                window_env.windowed_context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                window_env.process_layout();
                window_env.render(&hovered_node);
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { state, button, .. } => {
                        let event_name = match state {
                            ElementState::Pressed => "mousedown",
                            ElementState::Released => "click",
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
                        window_env.process_events()
                    }
                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        if TouchPhase::Moved == phase {
                            let cursor_pos = cursor_pos.lock().unwrap();
                            let scroll_data = {
                                match delta {
                                    MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                                    MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
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
                            window_env.process_events()
                        }
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        modifiers_state = modifiers;
                    }
                    WindowEvent::ReceivedCharacter(a) => {
                        // Emit the received character if the last pressed key wasn't text
                        if last_keydown == Key::Unidentified || !modifiers_state.is_empty() {
                            window_env
                                .freya_events
                                .lock()
                                .unwrap()
                                .push(FreyaEvent::Keyboard {
                                    name: "keydown",
                                    key: Key::Character(a.to_string()),
                                    code: last_code,
                                    modifiers: get_modifiers(modifiers_state),
                                });
                            window_env.process_events()
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_keycode),
                                state,
                                ..
                            },
                        ..
                    } => {
                        let event_name = match state {
                            ElementState::Pressed => "keydown",
                            ElementState::Released => "keyup",
                        };

                        // Only emit keys that aren't text (e.g ArrowUp isn't text)
                        // Text characters will be emitted by `WindowEvent::ReceivedCharacter`
                        let key = get_non_text_keys(&virtual_keycode);
                        if key != Key::Unidentified {
                            // Winit doesn't enable the alt modifier when pressing the AltGraph key, this is a workaround
                            if key == Key::AltGraph {
                                if state == ElementState::Pressed {
                                    modifiers_state.insert(ModifiersState::ALT)
                                } else {
                                    modifiers_state.remove(ModifiersState::ALT)
                                }
                            }

                            if state == ElementState::Pressed {
                                // Cache this key so `WindowEvent::ReceivedCharacter` knows
                                // it shouldn't emit anything until this same key emits keyup
                                last_keydown = key.clone();
                            } else {
                                // Uncache any key
                                last_keydown = Key::Unidentified;
                            }
                            window_env
                                .freya_events
                                .lock()
                                .unwrap()
                                .push(FreyaEvent::Keyboard {
                                    name: event_name,
                                    key,
                                    code: from_winit_to_code(&virtual_keycode),
                                    modifiers: get_modifiers(modifiers_state),
                                });
                        } else {
                            last_keydown = Key::Unidentified;
                        }

                        if state == ElementState::Pressed {
                            // Cache the key code on keydown event
                            last_code = from_winit_to_code(&virtual_keycode);
                        } else {
                            // Uncache any key code
                            last_code = Code::Unidentified;
                        }
                        window_env.process_events()
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
                        window_env.process_events()
                    }
                    WindowEvent::Resized(size) => {
                        let mut context = window_env.gr_context.clone();
                        window_env.surface = create_surface(
                            &window_env.windowed_context,
                            &window_env.fb_info,
                            &mut context,
                        );
                        window_env.windowed_context.resize(size);
                        window_env.windowed_context.window().request_redraw();
                    }
                    _ => {}
                }
            }
            Event::LoopDestroyed => {}
            _ => (),
        }
    });
}

pub fn winit_waker(proxy: &EventLoopProxy<()>) -> std::task::Waker {
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
    event_emitter_rx: &mut UnboundedReceiver<DomEvent>,
    state: &Option<T>,
    mutations_sender: &Option<UnboundedSender<()>>,
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
        let (to_update, diff) = rdom.lock().unwrap().apply_mutations(mutations);

        if !diff.is_empty() {
            mutations_sender.as_ref().map(|s| s.send(()));
        }

        let ctx = SendAnyMap::new();
        rdom.lock().unwrap().update_state(to_update, ctx);
    }
}
