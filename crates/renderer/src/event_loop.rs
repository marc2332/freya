use std::time::Instant;

use accesskit::Action;
use accesskit_winit::ActionRequestEvent;
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_elements::events::keyboard::{
    from_winit_to_code, get_modifiers, get_non_text_keys, Code, Key,
};
use torin::geometry::CursorPoint;
use tracing::info;
use winit::event::{
    ElementState, Event, KeyboardInput, ModifiersState, MouseScrollDelta, StartCause, Touch,
    TouchPhase, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

use crate::app::App;
use crate::HoveredNode;

// https://github.com/emilk/egui/issues/461
// https://github.com/rust-windowing/winit/issues/22
// https://github.com/flutter/flutter/issues/71385
const WHEEL_SPEED_MODIFIER: f32 = 53.0;

pub fn run_event_loop<State: Clone>(
    mut app: App<State>,
    event_loop: EventLoop<EventMessage>,
    proxy: EventLoopProxy<EventMessage>,
    hovered_node: HoveredNode,
) {
    let mut cursor_pos = CursorPoint::default();
    let mut last_keydown = Key::Unidentified;
    let mut last_code = Code::Unidentified;
    let mut modifiers_state = ModifiersState::empty();

    let window_env = app.window_env();

    window_env.run_on_setup();

    let mut frames = 0;
    let mut instant = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => {
                _ = proxy.send_event(EventMessage::PollVDOM);
            }
            Event::UserEvent(EventMessage::FocusAccessibilityNode(id)) => {
                app.accessibility().set_accessibility_focus(id);
            }
            Event::UserEvent(EventMessage::RequestRerender) => {
                app.window_env().window().request_redraw();
            }
            Event::UserEvent(EventMessage::RequestRelayout) => {
                app.process_layout();
            }
            Event::UserEvent(EventMessage::RemeasureTextGroup(text_id)) => {
                app.measure_text_group(&text_id);
            }
            Event::UserEvent(EventMessage::ActionRequestEvent(ActionRequestEvent {
                request,
                ..
            })) =>
            {
                #[allow(clippy::single_match)]
                match request.action {
                    Action::Focus => {
                        app.accessibility().set_accessibility_focus(request.target);
                    }
                    _ => {}
                }
            }
            Event::UserEvent(EventMessage::SetCursorIcon(icon)) => {
                app.window_env().window.set_cursor_icon(icon)
            }
            Event::UserEvent(ev) => {
                if let EventMessage::UpdateTemplate(template) = ev {
                    app.vdom_replace_template(template);
                }

                if matches!(ev, EventMessage::PollVDOM)
                    || matches!(ev, EventMessage::UpdateTemplate(_))
                {
                    app.poll_vdom();
                }
            }
            Event::RedrawRequested(_) => {
                app.process_layout();
                app.render(&hovered_node);
                app.tick();

                if instant.elapsed().as_millis() >= 1000 {
                    info!("{} FPS", frames);
                    instant = Instant::now();
                    frames = 0;
                } else {
                    frames += 1;
                }
            }
            Event::WindowEvent { event, .. } if app.on_window_event(&event) => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { state, button, .. } => {
                        let event_name = match state {
                            ElementState::Pressed => "mousedown",
                            ElementState::Released => "click",
                        };

                        app.push_event(FreyaEvent::Mouse {
                            name: event_name.to_string(),
                            cursor: cursor_pos,
                            button: Some(button),
                        });

                        app.process_events();
                    }
                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        if TouchPhase::Moved == phase {
                            let scroll_data = {
                                match delta {
                                    MouseScrollDelta::LineDelta(x, y) => (
                                        (x * WHEEL_SPEED_MODIFIER) as f64,
                                        (y * WHEEL_SPEED_MODIFIER) as f64,
                                    ),
                                    MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
                                }
                            };

                            app.push_event(FreyaEvent::Wheel {
                                name: "wheel".to_string(),
                                scroll: CursorPoint::from(scroll_data),
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
                                name: "keydown".to_string(),
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
                        if state == ElementState::Pressed && virtual_keycode == VirtualKeyCode::Tab
                        {
                            let direction = if modifiers_state.shift() {
                                AccessibilityFocusDirection::Backward
                            } else {
                                AccessibilityFocusDirection::Forward
                            };

                            app.focus_next_node(direction);

                            return;
                        }

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
                                name: event_name.to_string(),
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
                        cursor_pos = CursorPoint::from((position.x, position.y));

                        app.push_event(FreyaEvent::Mouse {
                            name: "mouseover".to_string(),
                            cursor: cursor_pos,
                            button: None,
                        });

                        app.process_events();
                    }
                    WindowEvent::Touch(Touch {
                        location,
                        phase,
                        id,
                        force,
                        ..
                    }) => {
                        cursor_pos = CursorPoint::from((location.x, location.y));

                        let event_name = match phase {
                            TouchPhase::Cancelled => "touchcancel",
                            TouchPhase::Ended => "touchend",
                            TouchPhase::Moved => "touchmove",
                            TouchPhase::Started => "touchstart",
                        };

                        app.push_event(FreyaEvent::Touch {
                            name: event_name.to_string(),
                            location: cursor_pos,
                            finger_id: id,
                            phase,
                            force,
                        });

                        app.process_events();
                    }
                    WindowEvent::Resized(size) => {
                        app.resize(size);
                    }
                    _ => {}
                }
            }
            Event::LoopDestroyed => {
                app.window_env().run_on_exit();
            }
            _ => (),
        }
    });
}
