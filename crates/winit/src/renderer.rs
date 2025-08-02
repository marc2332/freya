use std::path::PathBuf;

use freya_core::{
    accessibility::AccessibilityFocusStrategy,
    event_loop_messages::{
        EventLoopAppMessage,
        EventLoopAppMessageAction,
        EventLoopMessage,
    },
    events::{
        FileEventName,
        KeyboardEventName,
        MouseEventName,
        PlatformEvent,
        TouchEventName,
        WheelEventName,
    },
    platform_state::NavigationMode,
    window_config::OnCloseResponse,
};
use freya_elements::events::{
    Code,
    Key,
};
use torin::geometry::CursorPoint;
use winit::{
    application::ApplicationHandler,
    event::{
        ElementState,
        Ime,
        KeyEvent,
        MouseButton,
        MouseScrollDelta,
        StartCause,
        Touch,
        TouchPhase,
        WindowEvent,
    },
    event_loop::{
        EventLoop,
        EventLoopProxy,
    },
    keyboard::ModifiersState,
};

use crate::{
    app::AccessibilityTask,
    events::{
        map_winit_mouse_button,
        map_winit_touch_force,
        map_winit_touch_phase,
    },
    keyboard::{
        map_winit_key,
        map_winit_modifiers,
        map_winit_physical_key,
    },
    renderer_state::RendererState,
    LaunchConfig,
};

const WHEEL_SPEED_MODIFIER: f64 = 53.0;
const TOUCHPAD_SPEED_MODIFIER: f64 = 2.0;

/// Window renderer using Skia, Glutin and Winit.
pub struct WinitRenderer {
    pub(crate) event_loop_proxy: EventLoopProxy<EventLoopMessage>,
    pub(crate) state: RendererState,
    pub(crate) cursor_pos: CursorPoint,
    pub(crate) mouse_state: ElementState,
    pub(crate) modifiers_state: ModifiersState,
    pub(crate) dropped_file_paths: Vec<PathBuf>,
    pub(crate) custom_scale_factor: f64,
}

impl WinitRenderer {
    /// Run the Winit Renderer.
    pub fn launch(mut config: LaunchConfig) {
        let mut event_loop_builder = EventLoop::<EventLoopMessage>::with_user_event();

        if let Some(event_loop_builder_hook) = config.event_loop_builder_hook.take() {
            event_loop_builder_hook(&mut event_loop_builder);
        }

        let event_loop = event_loop_builder
            .build()
            .expect("Failed to create event loop.");

        let proxy = event_loop.create_proxy();

        let mut winit_renderer = WinitRenderer::new(config, proxy);

        event_loop.run_app(&mut winit_renderer).unwrap();
    }

    pub fn new(config: LaunchConfig<'_>, proxy: EventLoopProxy<EventLoopMessage>) -> Self {
        WinitRenderer {
            state: RendererState::new(
                config.windows_configs,
                config.embedded_fonts,
                config.plugins,
                config.fallback_fonts,
                proxy.clone(),
            ),
            event_loop_proxy: proxy,
            cursor_pos: CursorPoint::default(),
            mouse_state: ElementState::Released,
            modifiers_state: ModifiersState::default(),
            dropped_file_paths: Vec::new(),
            custom_scale_factor: 0.,
        }
    }
}

impl ApplicationHandler<EventLoopMessage> for WinitRenderer {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.state.resumed {
            self.state.resumed = true;

            // Create the windows
            let windows_configs = self.state.windows_configs.drain(..).collect::<Vec<_>>();
            for mut window_config in windows_configs {
                let on_setup = window_config.on_setup.take();

                let window_id = self.state.new_app(event_loop, window_config);

                self.event_loop_proxy
                    .send_event(EventLoopMessage::App(EventLoopAppMessage {
                        window_id: Some(window_id),
                        action: EventLoopAppMessageAction::PollVDOM,
                    }))
                    .ok();

                if let Some(on_setup) = on_setup {
                    let app = self.state.apps.get_mut(&window_id).unwrap();
                    (on_setup)(&mut app.window)
                }
            }
        }
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if cause == StartCause::Init {
            for window_id in self.state.apps.keys() {
                self.event_loop_proxy
                    .send_event(EventLoopMessage::App(EventLoopAppMessage {
                        window_id: Some(*window_id),
                        action: EventLoopAppMessageAction::PollVDOM,
                    }))
                    .ok();
            }
        }
    }

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: EventLoopMessage,
    ) {
        let custom_scale_factor = self.custom_scale_factor;
        match event {
            EventLoopMessage::NewWindow(window_config) => {
                let window_id = self.state.new_app(event_loop, window_config);
                self.event_loop_proxy
                    .send_event(EventLoopMessage::App(EventLoopAppMessage {
                        window_id: Some(window_id),
                        action: EventLoopAppMessageAction::PollVDOM,
                    }))
                    .ok();
            }

            EventLoopMessage::App(EventLoopAppMessage {
                window_id: Some(window_id),
                action,
            }) => {
                let mut remove_app = false;
                self.state.with_app(window_id, |app, _state| {
                    let scale_factor = app.window.scale_factor() + custom_scale_factor;
                    match action {
                        EventLoopAppMessageAction::FocusAccessibilityNode(strategy) => {
                            app.request_focus_node(strategy);
                        }
                        EventLoopAppMessageAction::RequestRerender => {
                            app.window.request_redraw();
                        }
                        EventLoopAppMessageAction::RequestFullRerender => {
                            app.resize();
                        }
                        EventLoopAppMessageAction::InvalidateArea(mut area) => {
                            let fdom = app.sdom.get();
                            area.size *= scale_factor as f32;
                            let mut compositor_dirty_area = fdom.compositor_dirty_area();
                            compositor_dirty_area.unite_or_insert(&area)
                        }
                        EventLoopAppMessageAction::RemeasureTextGroup(text_id) => {
                            app.measure_text_group(text_id, scale_factor);
                        }
                        EventLoopAppMessageAction::Accessibility(
                            accesskit_winit::WindowEvent::ActionRequested(request),
                        ) => {
                            if accesskit::Action::Focus == request.action {
                                app.request_focus_node(AccessibilityFocusStrategy::Node(
                                    request.target,
                                ));
                            }
                        }
                        EventLoopAppMessageAction::Accessibility(
                            accesskit_winit::WindowEvent::InitialTreeRequested,
                        ) => {
                            app.init_accessibility_on_next_render = true;
                        }
                        EventLoopAppMessageAction::SetCursorIcon(icon) => {
                            app.window.set_cursor(icon);
                        }
                        EventLoopAppMessageAction::WithWindow(use_window) => {
                            (use_window)(&app.window)
                        }
                        EventLoopAppMessageAction::CloseWindow => {
                            remove_app = true;
                        }
                        EventLoopAppMessageAction::PlatformEvent(platform_event) => {
                            app.send_event(platform_event, scale_factor);
                        }
                        EventLoopAppMessageAction::PollVDOM => {
                            app.poll_vdom();
                        }

                        _ => {}
                    }
                });

                if remove_app {
                    let _app = self.state.apps.remove(&window_id).unwrap();

                    if self.state.apps.is_empty() {
                        event_loop.exit();
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let custom_scale_factor = self.custom_scale_factor;

        let mut remove_app = false;

        self.state.with_app(id, |app, state| {
            app.accessibility
                .process_accessibility_event(&event, &app.window);
            let scale_factor = app.window.scale_factor() + custom_scale_factor;

            match event {
                WindowEvent::ThemeChanged(theme) => {
                    app.platform_sender.send_modify(|state| {
                        state.preferred_theme = theme.into();
                    });
                }

                WindowEvent::Ime(Ime::Commit(text)) => {
                    app.send_event(
                        PlatformEvent::Keyboard {
                            name: KeyboardEventName::KeyDown,
                            key: Key::Character(text),
                            code: Code::Unidentified,
                            modifiers: map_winit_modifiers(self.modifiers_state),
                        },
                        scale_factor,
                    );
                }
                WindowEvent::CloseRequested => {
                    if let Some(on_close) = app.window_config.on_close.take() {
                        let response = (on_close)(&mut app.window);
                        if response == OnCloseResponse::Close {
                            remove_app = true;
                        }
                    } else {
                        remove_app = true;
                    }
                }
                WindowEvent::RedrawRequested => {
                    app.platform_sender.send_if_modified(|state| {
                        let scale_factor_is_different = state.scale_factor == scale_factor;
                        state.scale_factor = scale_factor;
                        scale_factor_is_different
                    });

                    if app.process_layout_on_next_render {
                        app.process_layout(
                            scale_factor,
                            state.font_collection,
                            state.fallback_fonts,
                        );

                        app.process_layout_on_next_render = false;
                    }

                    if let Some(task) = app.accessibility_tasks_for_next_render.take() {
                        match task {
                            AccessibilityTask::ProcessWithMode(navigation_mode) => {
                                app.process_accessibility();
                                app.set_navigation_mode(navigation_mode);
                            }
                            AccessibilityTask::ProcessUpdate => {
                                app.process_accessibility();
                            }
                        }
                    }

                    if app.init_accessibility_on_next_render {
                        app.init_accessibility();
                        app.init_accessibility_on_next_render = false;
                    }

                    app.graphics_driver.make_current();

                    app.render(
                        scale_factor as f32,
                        state.font_collection,
                        state.font_mgr,
                        state.fallback_fonts,
                    );

                    app.event_loop_tick();
                    app.window.pre_present_notify();
                    app.graphics_driver.flush_and_submit();
                }
                WindowEvent::MouseInput {
                    state: mouse_state,
                    button,
                    ..
                } => {
                    app.set_navigation_mode(NavigationMode::NotKeyboard);

                    self.mouse_state = mouse_state;

                    let name = match mouse_state {
                        ElementState::Pressed => MouseEventName::MouseDown,
                        ElementState::Released => match button {
                            MouseButton::Middle => MouseEventName::MiddleClick,
                            MouseButton::Right => MouseEventName::RightClick,
                            _ => MouseEventName::MouseUp,
                        },
                    };

                    app.send_event(
                        PlatformEvent::Mouse {
                            name,
                            cursor: self.cursor_pos,
                            button: Some(map_winit_mouse_button(button)),
                        },
                        scale_factor,
                    );
                }
                WindowEvent::MouseWheel { delta, phase, .. } => {
                    if TouchPhase::Moved == phase {
                        let scroll_data = {
                            match delta {
                                MouseScrollDelta::LineDelta(x, y) => (
                                    (x as f64 * WHEEL_SPEED_MODIFIER),
                                    (y as f64 * WHEEL_SPEED_MODIFIER),
                                ),
                                MouseScrollDelta::PixelDelta(pos) => (
                                    (pos.x * TOUCHPAD_SPEED_MODIFIER),
                                    (pos.y * TOUCHPAD_SPEED_MODIFIER),
                                ),
                            }
                        };

                        app.send_event(
                            PlatformEvent::Wheel {
                                name: WheelEventName::Wheel,
                                scroll: CursorPoint::from(scroll_data),
                                cursor: self.cursor_pos,
                            },
                            scale_factor,
                        );
                    }
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    self.modifiers_state = modifiers.state();
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
                    if !app.is_window_focused {
                        return;
                    }

                    #[allow(dead_code)]
                    let is_control_pressed = {
                        if cfg!(target_os = "macos") {
                            self.modifiers_state.super_key()
                        } else {
                            self.modifiers_state.control_key()
                        }
                    };

                    #[allow(dead_code)]
                    let change_animation_clock = is_control_pressed
                        && self.modifiers_state.alt_key()
                        && state == ElementState::Pressed;

                    #[cfg(debug_assertions)]
                    if change_animation_clock {
                        let ch = logical_key.to_text();
                        let render = if ch == Some("+") {
                            app.sdom.get().animation_clock().increase_by(0.2);
                            true
                        } else if ch == Some("-") {
                            app.sdom.get().animation_clock().decrease_by(0.2);
                            true
                        } else {
                            false
                        };

                        if render {
                            app.resize();
                            app.window.request_redraw();
                        }
                    }

                    #[cfg(not(feature = "disable-zoom-shortcuts"))]
                    if !change_animation_clock
                        && is_control_pressed
                        && state == ElementState::Pressed
                    {
                        let ch = logical_key.to_text();
                        let render = if ch == Some("+") {
                            self.custom_scale_factor =
                                (self.custom_scale_factor + 0.10).clamp(-1.0, 5.0);
                            true
                        } else if ch == Some("-") {
                            self.custom_scale_factor =
                                (self.custom_scale_factor - 0.10).clamp(-1.0, 5.0);
                            true
                        } else {
                            false
                        };

                        if render {
                            app.resize();
                            app.window.request_redraw();
                        }
                    }

                    let name = match state {
                        ElementState::Pressed => KeyboardEventName::KeyDown,
                        ElementState::Released => KeyboardEventName::KeyUp,
                    };
                    app.send_event(
                        PlatformEvent::Keyboard {
                            name,
                            key: map_winit_key(&logical_key),
                            code: map_winit_physical_key(&physical_key),
                            modifiers: map_winit_modifiers(self.modifiers_state),
                        },
                        scale_factor,
                    )
                }
                WindowEvent::CursorLeft { .. } => {
                    if self.mouse_state == ElementState::Released {
                        self.cursor_pos = CursorPoint::new(-1.0, -1.0);

                        app.send_event(
                            PlatformEvent::Mouse {
                                name: MouseEventName::MouseMove,
                                cursor: self.cursor_pos,
                                button: None,
                            },
                            scale_factor,
                        );
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.cursor_pos = CursorPoint::from((position.x, position.y));

                    app.send_event(
                        PlatformEvent::Mouse {
                            name: MouseEventName::MouseMove,
                            cursor: self.cursor_pos,
                            button: None,
                        },
                        scale_factor,
                    );

                    for dropped_file_path in self.dropped_file_paths.drain(..).collect::<Vec<_>>() {
                        app.send_event(
                            PlatformEvent::File {
                                name: FileEventName::FileDrop,
                                file_path: Some(dropped_file_path),
                                cursor: self.cursor_pos,
                            },
                            scale_factor,
                        );
                    }
                }
                WindowEvent::Touch(Touch {
                    location,
                    phase,
                    id,
                    force,
                    ..
                }) => {
                    self.cursor_pos = CursorPoint::from((location.x, location.y));

                    let name = match phase {
                        TouchPhase::Cancelled => TouchEventName::TouchCancel,
                        TouchPhase::Ended => TouchEventName::TouchEnd,
                        TouchPhase::Moved => TouchEventName::TouchMove,
                        TouchPhase::Started => TouchEventName::TouchStart,
                    };

                    app.send_event(
                        PlatformEvent::Touch {
                            name,
                            location: self.cursor_pos,
                            finger_id: id,
                            phase: map_winit_touch_phase(phase),
                            force: force.map(map_winit_touch_force),
                        },
                        scale_factor,
                    );
                }
                WindowEvent::Resized(size) => {
                    let (new_surface, new_dirty_surface) = app.graphics_driver.resize(size);

                    app.surface = new_surface;
                    app.dirty_surface = new_dirty_surface;

                    app.window.request_redraw();

                    app.resize();
                }
                WindowEvent::DroppedFile(file_path) => {
                    self.dropped_file_paths.push(file_path);
                }
                WindowEvent::HoveredFile(file_path) => {
                    app.send_event(
                        PlatformEvent::File {
                            name: FileEventName::FileHover,
                            file_path: Some(file_path),
                            cursor: self.cursor_pos,
                        },
                        scale_factor,
                    );
                }
                WindowEvent::HoveredFileCancelled => {
                    app.send_event(
                        PlatformEvent::File {
                            name: FileEventName::FileHoverCancelled,
                            file_path: None,
                            cursor: self.cursor_pos,
                        },
                        scale_factor,
                    );
                }
                WindowEvent::Focused(is_focused) => {
                    app.is_window_focused = is_focused;
                }
                _ => {}
            }
        });

        if remove_app {
            let _app = self.state.apps.remove(&id).unwrap();

            if self.state.apps.is_empty() {
                event_loop.exit();
            }
        }
    }
}
