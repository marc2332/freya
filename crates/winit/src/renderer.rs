use std::path::PathBuf;

use dioxus_core::VirtualDom;
use freya_core::{
    accessibility::AccessibilityFocusStrategy,
    dom::SafeDOM,
    event_loop_messages::EventLoopMessage,
    events::{
        FileEventName,
        KeyboardEventName,
        MouseEventName,
        PlatformEvent,
        TouchEventName,
        WheelEventName,
    },
    platform_state::NavigationMode,
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
    devtools::Devtools,
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
    window_state::{
        CreatedState,
        NotCreatedState,
        WindowState,
    },
    LaunchConfig,
};

const WHEEL_SPEED_MODIFIER: f64 = 53.0;
const TOUCHPAD_SPEED_MODIFIER: f64 = 2.0;

/// Window renderer using Skia, Glutin and Winit.
pub struct WinitRenderer<'a, State: Clone + 'static> {
    pub(crate) event_loop_proxy: EventLoopProxy<EventLoopMessage>,
    pub(crate) state: WindowState<'a, State>,
    pub(crate) cursor_pos: CursorPoint,
    pub(crate) mouse_state: ElementState,
    pub(crate) modifiers_state: ModifiersState,
    pub(crate) dropped_file_paths: Vec<PathBuf>,
    pub(crate) custom_scale_factor: f64,
}

impl<'a, State: Clone + 'static> WinitRenderer<'a, State> {
    /// Run the Winit Renderer.
    pub fn launch(
        vdom: VirtualDom,
        sdom: SafeDOM,
        mut config: LaunchConfig<State>,
        devtools: Option<Devtools>,
    ) {
        let mut event_loop_builder = EventLoop::<EventLoopMessage>::with_user_event();
        let event_loop_builder_hook = config.window_config.event_loop_builder_hook.take();
        if let Some(event_loop_builder_hook) = event_loop_builder_hook {
            event_loop_builder_hook(&mut event_loop_builder);
        }
        let event_loop = event_loop_builder
            .build()
            .expect("Failed to create event loop.");
        let proxy = event_loop.create_proxy();

        let mut winit_renderer = WinitRenderer::new(vdom, sdom, config, devtools, proxy);

        #[cfg(all(debug_assertions, feature = "hot-reloading"))]
        {
            dioxus_devtools::connect({
                let proxy = event_loop.create_proxy();
                move |event| {
                    println!("got event: {event:#?}");
                    let _ = proxy.send_event(EventLoopMessage::DioxusDevserverEvent(event));
                }
            });
        }

        event_loop.run_app(&mut winit_renderer).unwrap();
    }

    pub fn new(
        vdom: VirtualDom,
        sdom: SafeDOM,
        config: LaunchConfig<'a, State>,
        devtools: Option<Devtools>,
        proxy: EventLoopProxy<EventLoopMessage>,
    ) -> Self {
        WinitRenderer {
            state: WindowState::NotCreated(NotCreatedState {
                sdom,
                devtools,
                vdom,
                config,
            }),
            event_loop_proxy: proxy,
            cursor_pos: CursorPoint::default(),
            mouse_state: ElementState::Released,
            modifiers_state: ModifiersState::default(),
            dropped_file_paths: Vec::new(),
            custom_scale_factor: 0.,
        }
    }

    // Send and process an event
    fn send_event(&mut self, event: PlatformEvent) {
        let scale_factor = self.scale_factor();
        self.state
            .created_state()
            .app
            .send_event(event, scale_factor);
    }

    /// Get the current scale factor of the Window
    fn scale_factor(&self) -> f64 {
        match &self.state {
            WindowState::Created(CreatedState { window, .. }) => {
                window.scale_factor() + self.custom_scale_factor
            }
            _ => 0.0,
        }
    }

    /// Run the `on_setup` callback that was passed to the launch function
    pub fn run_on_setup(&mut self) {
        let state = self.state.created_state();
        if let Some(on_setup) = state.window_config.on_setup.take() {
            (on_setup)(&mut state.window)
        }
    }

    /// Run the `on_exit` callback that was passed to the launch function
    pub fn run_on_exit(&mut self) {
        let state = self.state.created_state();
        if let Some(on_exit) = state.window_config.on_exit.take() {
            (on_exit)(&mut state.window)
        }
    }
}

impl<State: Clone> ApplicationHandler<EventLoopMessage> for WinitRenderer<'_, State> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.state.has_been_created() {
            self.state.create(event_loop, &self.event_loop_proxy);
            self.run_on_setup();
        }
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if cause == StartCause::Init {
            self.event_loop_proxy
                .send_event(EventLoopMessage::PollVDOM)
                .ok();
        }
    }

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: EventLoopMessage,
    ) {
        let scale_factor = self.scale_factor();
        let CreatedState { window, app, .. } = self.state.created_state();
        match event {
            EventLoopMessage::FocusAccessibilityNode(strategy) => {
                app.request_focus_node(strategy);
                window.request_redraw();
            }
            EventLoopMessage::RequestRerender => {
                window.request_redraw();
            }
            EventLoopMessage::RequestFullRerender => {
                app.resize(window);
                window.request_redraw();
            }
            EventLoopMessage::InvalidateArea(mut area) => {
                let fdom = app.sdom.get();
                area.size *= scale_factor as f32;
                let mut compositor_dirty_area = fdom.compositor_dirty_area();
                compositor_dirty_area.unite_or_insert(&area)
            }
            EventLoopMessage::RemeasureTextGroup(text_id) => {
                app.measure_text_group(text_id, scale_factor);
            }
            EventLoopMessage::Accessibility(accesskit_winit::WindowEvent::ActionRequested(
                request,
            )) => {
                if accesskit::Action::Focus == request.action {
                    app.request_focus_node(AccessibilityFocusStrategy::Node(request.target));
                    window.request_redraw();
                }
            }
            EventLoopMessage::Accessibility(accesskit_winit::WindowEvent::InitialTreeRequested) => {
                app.init_accessibility_on_next_render = true;
            }
            EventLoopMessage::SetCursorIcon(icon) => window.set_cursor(icon),
            EventLoopMessage::WithWindow(use_window) => (use_window)(window),
            EventLoopMessage::ExitApp => event_loop.exit(),
            EventLoopMessage::PlatformEvent(platform_event) => self.send_event(platform_event),
            #[cfg(all(debug_assertions, feature = "hot-reloading"))]
            EventLoopMessage::DioxusDevserverEvent(event) => match event {
                dioxus_devtools::DevserverMsg::HotReload(hot_reload_msg) => {
                    if hot_reload_msg.jump_table.is_some() {
                        dioxus_devtools::apply_changes(&app.vdom, &hot_reload_msg);
                    } else {
                        eprintln!("got hot-reload message from dioxus-cli, freya does not work with hot reloading, \
                            please use hot patching instead by passing --hot-patch and disable hot-reloading with --hot-reload false");
                    }
                }
                dioxus_devtools::DevserverMsg::Shutdown => event_loop.exit(),
                _ => {}
            },
            EventLoopMessage::PollVDOM => {
                app.poll_vdom(window);
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let scale_factor = self.scale_factor();
        let CreatedState {
            surface,
            dirty_surface,
            window,
            window_config,
            app,
            is_window_focused,
            graphics_driver,
            ..
        } = self.state.created_state();
        app.accessibility
            .process_accessibility_event(&event, window);
        match event {
            WindowEvent::ThemeChanged(theme) => {
                app.platform_sender.send_modify(|state| {
                    state.preferred_theme = theme.into();
                });
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Ime(Ime::Commit(text)) => {
                self.send_event(PlatformEvent::Keyboard {
                    name: KeyboardEventName::KeyDown,
                    key: Key::Character(text),
                    code: Code::Unidentified,
                    modifiers: map_winit_modifiers(self.modifiers_state),
                });
            }
            WindowEvent::RedrawRequested => {
                app.platform_sender.send_if_modified(|state| {
                    let scale_factor_is_different = state.scale_factor == scale_factor;
                    state.scale_factor = scale_factor;
                    scale_factor_is_different
                });

                if app.process_layout_on_next_render {
                    app.process_layout(window.inner_size(), scale_factor);

                    app.process_layout_on_next_render = false;
                }

                if let Some(task) = app.accessibility_tasks_for_next_render.take() {
                    match task {
                        AccessibilityTask::ProcessWithMode(navigation_mode) => {
                            app.process_accessibility(window);
                            app.set_navigation_mode(navigation_mode);
                        }
                        AccessibilityTask::ProcessUpdate => {
                            app.process_accessibility(window);
                        }
                    }
                }

                if app.init_accessibility_on_next_render {
                    app.init_accessibility();
                    app.init_accessibility_on_next_render = false;
                }

                graphics_driver.make_current();

                app.render(
                    window_config.background,
                    surface,
                    dirty_surface,
                    window,
                    scale_factor,
                );

                app.event_loop_tick();
                window.pre_present_notify();
                graphics_driver.flush_and_submit();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                app.set_navigation_mode(NavigationMode::NotKeyboard);

                self.mouse_state = state;

                let name = match state {
                    ElementState::Pressed => MouseEventName::MouseDown,
                    ElementState::Released => match button {
                        MouseButton::Middle => MouseEventName::MiddleClick,
                        MouseButton::Right => MouseEventName::RightClick,
                        _ => MouseEventName::MouseUp,
                    },
                };

                self.send_event(PlatformEvent::Mouse {
                    name,
                    cursor: self.cursor_pos,
                    button: Some(map_winit_mouse_button(button)),
                });
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

                    self.send_event(PlatformEvent::Wheel {
                        name: WheelEventName::Wheel,
                        scroll: CursorPoint::from(scroll_data),
                        cursor: self.cursor_pos,
                    });
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
                if !*is_window_focused {
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
                        app.resize(window);
                        window.request_redraw();
                    }
                }

                #[cfg(not(feature = "disable-zoom-shortcuts"))]
                if !change_animation_clock && is_control_pressed && state == ElementState::Pressed {
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
                        app.resize(window);
                        window.request_redraw();
                    }
                }

                let name = match state {
                    ElementState::Pressed => KeyboardEventName::KeyDown,
                    ElementState::Released => KeyboardEventName::KeyUp,
                };
                self.send_event(PlatformEvent::Keyboard {
                    name,
                    key: map_winit_key(&logical_key),
                    code: map_winit_physical_key(&physical_key),
                    modifiers: map_winit_modifiers(self.modifiers_state),
                })
            }
            WindowEvent::CursorLeft { .. } => {
                if self.mouse_state == ElementState::Released {
                    self.cursor_pos = CursorPoint::new(-1.0, -1.0);

                    self.send_event(PlatformEvent::Mouse {
                        name: MouseEventName::MouseMove,
                        cursor: self.cursor_pos,
                        button: None,
                    });
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = CursorPoint::from((position.x, position.y));

                self.send_event(PlatformEvent::Mouse {
                    name: MouseEventName::MouseMove,
                    cursor: self.cursor_pos,
                    button: None,
                });

                for dropped_file_path in self.dropped_file_paths.drain(..).collect::<Vec<_>>() {
                    self.send_event(PlatformEvent::File {
                        name: FileEventName::FileDrop,
                        file_path: Some(dropped_file_path),
                        cursor: self.cursor_pos,
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
                self.cursor_pos = CursorPoint::from((location.x, location.y));

                let name = match phase {
                    TouchPhase::Cancelled => TouchEventName::TouchCancel,
                    TouchPhase::Ended => TouchEventName::TouchEnd,
                    TouchPhase::Moved => TouchEventName::TouchMove,
                    TouchPhase::Started => TouchEventName::TouchStart,
                };

                self.send_event(PlatformEvent::Touch {
                    name,
                    location: self.cursor_pos,
                    finger_id: id,
                    phase: map_winit_touch_phase(phase),
                    force: force.map(map_winit_touch_force),
                });
            }
            WindowEvent::Resized(size) => {
                let (new_surface, new_dirty_surface) = graphics_driver.resize(size);

                *surface = new_surface;
                *dirty_surface = new_dirty_surface;

                window.request_redraw();

                app.resize(window);
            }
            WindowEvent::DroppedFile(file_path) => {
                self.dropped_file_paths.push(file_path);
            }
            WindowEvent::HoveredFile(file_path) => {
                self.send_event(PlatformEvent::File {
                    name: FileEventName::FileHover,
                    file_path: Some(file_path),
                    cursor: self.cursor_pos,
                });
            }
            WindowEvent::HoveredFileCancelled => {
                self.send_event(PlatformEvent::File {
                    name: FileEventName::FileHoverCancelled,
                    file_path: None,
                    cursor: self.cursor_pos,
                });
            }
            WindowEvent::Focused(is_focused) => {
                *is_window_focused = is_focused;
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.run_on_exit();
    }
}
