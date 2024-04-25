use accesskit::Action;
use accesskit_winit::ActionRequestEvent;
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_elements::events::keyboard::{
    map_winit_key, map_winit_modifiers, map_winit_physical_key, Code, Key,
};
use torin::geometry::CursorPoint;
use winit::event::{
    ElementState, Event, Ime, KeyEvent, MouseButton, MouseScrollDelta, StartCause, Touch,
    TouchPhase, WindowEvent,
};
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::keyboard::ModifiersState;

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
    let mut modifiers_state = ModifiersState::empty();
    let mut dropped_file_path = None;

    app.window_env.run_on_setup();

    event_loop
        .run(move |event, event_loop| match event {
            Event::NewEvents(StartCause::Init) => {
                _ = proxy.send_event(EventMessage::PollVDOM);
            }
            Event::UserEvent(EventMessage::FocusAccessibilityNode(id)) => {
                app.accessibility
                    .set_accessibility_focus(id, &app.window_env.window);
            }
            Event::UserEvent(EventMessage::RequestRerender) => {
                app.window_env.window.request_redraw();
            }
            Event::UserEvent(EventMessage::RemeasureTextGroup(text_id)) => {
                app.measure_text_group(&text_id);
            }
            Event::UserEvent(EventMessage::ActionRequestEvent(ActionRequestEvent {
                request,
                ..
            })) => {
                if Action::Focus == request.action {
                    app.accessibility
                        .set_accessibility_focus(request.target, &app.window_env.window);
                }
            }
            Event::UserEvent(EventMessage::SetCursorIcon(icon)) => {
                app.window_env.window.set_cursor_icon(icon)
            }
            Event::UserEvent(EventMessage::FocusPrevAccessibilityNode) => {
                app.set_navigation_mode(NavigationMode::Keyboard);
                app.focus_next_node(AccessibilityFocusDirection::Backward);
            }
            Event::UserEvent(EventMessage::FocusNextAccessibilityNode) => {
                app.set_navigation_mode(NavigationMode::Keyboard);
                app.focus_next_node(AccessibilityFocusDirection::Forward);
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
            Event::WindowEvent { event, .. } => {
                app.accessibility
                    .process_accessibility_event(&event, &app.window_env.window);
                match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::Ime(Ime::Commit(text)) => {
                        app.send_event(PlatformEvent::Keyboard {
                            name: EventName::KeyDown,
                            key: Key::Character(text),
                            code: Code::Unidentified,
                            modifiers: map_winit_modifiers(modifiers_state),
                        });
                    }
                    WindowEvent::RedrawRequested => {
                        if app.measure_layout_on_next_render {
                            app.process_layout();

                            app.measure_layout_on_next_render = false;
                        }
                        app.render(&hovered_node);
                        app.event_loop_tick();
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        app.set_navigation_mode(NavigationMode::NotKeyboard);

                        let name = match state {
                            ElementState::Pressed => EventName::MouseDown,
                            ElementState::Released => match button {
                                MouseButton::Middle => EventName::MiddleClick,
                                MouseButton::Right => EventName::RightClick,
                                MouseButton::Left => EventName::Click,
                                _ => EventName::PointerUp,
                            },
                        };

                        app.send_event(PlatformEvent::Mouse {
                            name,
                            cursor: cursor_pos,
                            button: Some(button),
                        });
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

                            app.send_event(PlatformEvent::Wheel {
                                name: EventName::Wheel,
                                scroll: CursorPoint::from(scroll_data),
                                cursor: cursor_pos,
                            });
                        }
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        modifiers_state = modifiers.state();
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key,
                                logical_key,
                                state,
                                ..
                            },
                        ..
                    } => {
                        let name = match state {
                            ElementState::Pressed => EventName::KeyDown,
                            ElementState::Released => EventName::KeyUp,
                        };
                        app.send_event(PlatformEvent::Keyboard {
                            name,
                            key: map_winit_key(&logical_key),
                            code: map_winit_physical_key(&physical_key),
                            modifiers: map_winit_modifiers(modifiers_state),
                        })
                    }
                    WindowEvent::CursorLeft { .. } => {
                        cursor_pos = CursorPoint::new(-1.0, -1.0);

                        app.send_event(PlatformEvent::Mouse {
                            name: EventName::MouseOver,
                            cursor: cursor_pos,
                            button: None,
                        });
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_pos = CursorPoint::from((position.x, position.y));

                        app.send_event(PlatformEvent::Mouse {
                            name: EventName::MouseOver,
                            cursor: cursor_pos,
                            button: None,
                        });

                        if let Some(dropped_file_path) = dropped_file_path.take() {
                            app.send_event(PlatformEvent::File {
                                name: EventName::FileDrop,
                                file_path: Some(dropped_file_path),
                                cursor: cursor_pos,
                            });
                        }
                    }
                    WindowEvent::Touch(Touch {
                        location,
                        phase,
                        id,
                        force,
                        ..
                    }) => {
                        cursor_pos = CursorPoint::from((location.x, location.y));

                        let name = match phase {
                            TouchPhase::Cancelled => EventName::TouchCancel,
                            TouchPhase::Ended => EventName::TouchEnd,
                            TouchPhase::Moved => EventName::TouchMove,
                            TouchPhase::Started => EventName::TouchStart,
                        };

                        app.send_event(PlatformEvent::Touch {
                            name,
                            location: cursor_pos,
                            finger_id: id,
                            phase,
                            force,
                        });
                    }
                    WindowEvent::Resized(size) => {
                        app.resize(size);
                    }
                    WindowEvent::DroppedFile(file_path) => {
                        dropped_file_path = Some(file_path);
                    }
                    WindowEvent::HoveredFile(file_path) => {
                        app.send_event(PlatformEvent::File {
                            name: EventName::GlobalFileHover,
                            file_path: Some(file_path),
                            cursor: cursor_pos,
                        });
                    }
                    WindowEvent::HoveredFileCancelled => {
                        app.send_event(PlatformEvent::File {
                            name: EventName::GlobalFileHoverCancelled,
                            file_path: None,
                            cursor: cursor_pos,
                        });
                    }
                    _ => {}
                }
            }
            Event::LoopExiting => {
                app.window_env.run_on_exit();
            }
            _ => (),
        })
        .expect("Failed to run Eventloop.");
}
