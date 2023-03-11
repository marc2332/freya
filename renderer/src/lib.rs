use app::App;
use dioxus_core::VirtualDom;

use dioxus_native_core::NodeId;
use freya_common::EventMessage;
use freya_core::dom::DioxusSafeDOM;

use freya_core::events::FreyaEvent;
use freya_elements::{from_winit_to_code, get_modifiers, get_non_text_keys, Code, Key};
use freya_layout::DioxusDOM;

use glutin::event::{
    ElementState, Event, KeyboardInput, ModifiersState, MouseScrollDelta, StartCause, TouchPhase,
    WindowEvent,
};
use glutin::event_loop::{ControlFlow, EventLoopBuilder};
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::UnboundedSender;
pub use window::{create_surface, WindowEnv};
pub use window_config::WindowConfig;

mod wireframe;

mod app;
mod elements;
mod renderer;
mod window;
mod window_config;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;

/// Start the winit event loop with the virtual dom polling
pub fn run<T: 'static + Clone>(
    vdom: VirtualDom,
    rdom: DioxusSafeDOM,
    window_config: WindowConfig<T>,
    mutations_sender: Option<UnboundedSender<()>>,
    hovered_node: HoveredNode,
) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _guard = rt.enter();

    let event_loop = EventLoopBuilder::<EventMessage>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // Hotreload
    #[cfg(debug_assertions)]
    {
        use std::process::exit;
        let proxy = proxy.clone();
        dioxus_hot_reload::connect(move |msg| match msg {
            dioxus_hot_reload::HotReloadMsg::UpdateTemplate(template) => {
                let _ = proxy.send_event(EventMessage::UpdateTemplate(template));
            }
            dioxus_hot_reload::HotReloadMsg::Shutdown => exit(0),
        });
    }

    let mut app = App::new(
        rdom,
        vdom,
        &proxy,
        mutations_sender,
        WindowEnv::from_config(window_config, &event_loop),
    );

    app.init_vdom();

    let mut cursor_pos = (0.0, 0.0);
    let mut last_keydown = Key::Unidentified;
    let mut last_code = Code::Unidentified;
    let mut modifiers_state = ModifiersState::empty();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                _ = proxy.send_event(EventMessage::PollVDOM);
            }
            Event::UserEvent(EventMessage::FocusAccessibilityNode(id)) => {
                app.set_accessibility_focus(id);
            }
            Event::UserEvent(EventMessage::RequestRerender) => {
                app.request_redraw();
            }
            Event::UserEvent(EventMessage::RequestRelayout) => {
                app.process_layout();
            }
            Event::UserEvent(ev) => {
                if let EventMessage::UpdateTemplate(template) = ev {
                    app.vdom_replace_template(template);
                }

                if matches!(ev, EventMessage::PollVDOM)
                    || matches!(ev, EventMessage::UpdateTemplate(..))
                {
                    app.poll_vdom();
                }
            }
            Event::RedrawRequested(_) => {
                app.clear_accessibility();
                app.process_layout();
                app.render(&hovered_node);
                app.render_accessibility();
            }
            Event::WindowEvent { event, .. } if app.on_accessibility_window_event(&event) => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { state, button, .. } => {
                        let event_name = match state {
                            ElementState::Pressed => "mousedown",
                            ElementState::Released => "click",
                        };

                        app.push_event(FreyaEvent::Mouse {
                            name: event_name,
                            cursor: cursor_pos,
                            button: Some(button),
                        });

                        app.process_events();
                    }
                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        if TouchPhase::Moved == phase {
                            let scroll_data = {
                                match delta {
                                    MouseScrollDelta::LineDelta(x, y) => (x as f64, y as f64),
                                    MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                                }
                            };

                            app.push_event(FreyaEvent::Wheel {
                                name: "wheel",
                                scroll: scroll_data,
                                cursor: cursor_pos,
                            });

                            app.process_events();
                        }
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        modifiers_state = modifiers;
                    }
                    WindowEvent::ReceivedCharacter(a) => {
                        // Emit the received character if the last pressed key wasn't text
                        if last_keydown == Key::Unidentified || !modifiers_state.is_empty() {
                            app.push_event(FreyaEvent::Keyboard {
                                name: "keydown",
                                key: Key::Character(a.to_string()),
                                code: last_code,
                                modifiers: get_modifiers(modifiers_state),
                            });

                            app.process_events();
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
                            app.push_event(FreyaEvent::Keyboard {
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

                        app.process_events();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_pos = (position.x, position.y);

                        app.push_event(FreyaEvent::Mouse {
                            name: "mouseover",
                            cursor: cursor_pos,
                            button: None,
                        });

                        app.process_events();
                    }
                    WindowEvent::Resized(size) => {
                        app.resize(size);
                    }
                    _ => {}
                }
            }
            Event::LoopDestroyed => {}
            _ => (),
        }
    });
}
