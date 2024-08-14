use std::{
    num::NonZeroU32,
    path::PathBuf,
};

use dioxus_core::VirtualDom;
use freya_common::EventMessage;
use freya_core::{
    accessibility::AccessibilityFocusDirection,
    dom::SafeDOM,
    events::{
        EventName,
        PlatformEvent,
    },
    prelude::NavigationMode,
};
use freya_elements::events::{
    map_winit_key,
    map_winit_modifiers,
    map_winit_physical_key,
    Code,
    Key,
};
use glutin::prelude::{
    GlSurface,
    PossiblyCurrentGlContext,
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
    devtools::Devtools,
    window_state::{
        create_surface,
        CreatedState,
        NotCreatedState,
        WindowState,
    },
    HoveredNode,
    LaunchConfig,
};

const WHEEL_SPEED_MODIFIER: f32 = 53.0;

/// Desktop renderer using Skia, Glutin and Winit
pub struct DesktopRenderer<'a, State: Clone + 'static> {
    pub(crate) event_loop_proxy: EventLoopProxy<EventMessage>,
    pub(crate) state: WindowState<'a, State>,
    pub(crate) hovered_node: HoveredNode,
    pub(crate) cursor_pos: CursorPoint,
    pub(crate) mouse_state: ElementState,
    pub(crate) modifiers_state: ModifiersState,
    pub(crate) dropped_file_path: Option<PathBuf>,
}

impl<'a, State: Clone + 'static> DesktopRenderer<'a, State> {
    /// Run the Desktop Renderer.
    pub fn launch(
        vdom: VirtualDom,
        sdom: SafeDOM,
        config: LaunchConfig<State>,
        devtools: Option<Devtools>,
        hovered_node: HoveredNode,
    ) {
        let event_loop = EventLoop::<EventMessage>::with_user_event()
            .build()
            .expect("Failed to create event loop.");
        let proxy = event_loop.create_proxy();

        // Hotreload support for Dioxus
        #[cfg(feature = "hot-reload")]
        {
            use std::process::exit;
            let proxy = proxy.clone();
            dioxus_hot_reload::connect(move |msg| match msg {
                dioxus_hot_reload::HotReloadMsg::UpdateTemplate(template) => {
                    let _ = proxy.send_event(EventMessage::UpdateTemplate(template));
                }
                dioxus_hot_reload::HotReloadMsg::Shutdown => exit(0),
                dioxus_hot_reload::HotReloadMsg::UpdateAsset(_) => {}
            });
        }

        let mut desktop_renderer =
            DesktopRenderer::new(vdom, sdom, config, devtools, hovered_node, proxy);

        event_loop.run_app(&mut desktop_renderer).unwrap();
    }

    pub fn new(
        vdom: VirtualDom,
        sdom: SafeDOM,
        config: LaunchConfig<'a, State>,
        devtools: Option<Devtools>,
        hovered_node: HoveredNode,
        proxy: EventLoopProxy<EventMessage>,
    ) -> Self {
        DesktopRenderer {
            state: WindowState::NotCreated(NotCreatedState {
                sdom,
                devtools,
                vdom,
                config,
            }),
            hovered_node,
            event_loop_proxy: proxy,
            cursor_pos: CursorPoint::default(),
            mouse_state: ElementState::Released,
            modifiers_state: ModifiersState::default(),
            dropped_file_path: None,
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
            WindowState::Created(CreatedState { window, .. }) => window.scale_factor(),
            _ => 0.0,
        }
    }

    /// Run the `on_setup` callback that was passed to the launch function
    pub fn run_on_setup(&mut self) {
        let state = self.state.created_state();
        if let Some(on_setup) = &state.window_config.on_setup {
            (on_setup)(&mut state.window)
        }
    }

    /// Run the `on_exit` callback that was passed to the launch function
    pub fn run_on_exit(&mut self) {
        let state = self.state.created_state();
        if let Some(on_exit) = &state.window_config.on_exit {
            (on_exit)(&mut state.window)
        }
    }
}

impl<'a, State: Clone> ApplicationHandler<EventMessage> for DesktopRenderer<'a, State> {
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
                .send_event(EventMessage::PollVDOM)
                .ok();
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: EventMessage) {
        let scale_factor = self.scale_factor();
        let CreatedState { window, app, .. } = self.state.created_state();
        match event {
            EventMessage::FocusAccessibilityNode(id) => {
                app.focus_node(id, window);
            }
            EventMessage::RequestRerender => {
                window.request_redraw();
            }
            EventMessage::RemeasureTextGroup(text_id) => {
                app.measure_text_group(text_id, scale_factor);
            }
            EventMessage::Accessibility(accesskit_winit::WindowEvent::ActionRequested(request)) => {
                if accesskit::Action::Focus == request.action {
                    app.focus_node(request.target, window);
                }
            }
            EventMessage::Accessibility(accesskit_winit::WindowEvent::InitialTreeRequested) => {
                app.accessibility.process_initial_tree();
            }
            EventMessage::SetCursorIcon(icon) => window.set_cursor(icon),
            EventMessage::FocusPrevAccessibilityNode => {
                app.set_navigation_mode(NavigationMode::Keyboard);
                app.focus_next_node(AccessibilityFocusDirection::Backward, window);
            }
            EventMessage::FocusNextAccessibilityNode => {
                app.set_navigation_mode(NavigationMode::Keyboard);
                app.focus_next_node(AccessibilityFocusDirection::Forward, window);
            }
            EventMessage::WithWindow(use_window) => (use_window)(window),
            EventMessage::QueueFocusAccessibilityNode(node_id) => {
                app.queue_focus_node(node_id);
            }
            EventMessage::ExitApp => event_loop.exit(),
            ev => {
                if let EventMessage::UpdateTemplate(template) = ev {
                    app.vdom_replace_template(template);
                }

                if matches!(ev, EventMessage::PollVDOM)
                    || matches!(ev, EventMessage::UpdateTemplate(_))
                {
                    app.poll_vdom(window);
                }
            }
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
            gr_context,
            surface,
            dirty_surface,
            gl_surface,
            gl_context,
            window,
            app,
            fb_info,
            num_samples,
            stencil_size,
            is_window_focused,
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
                    name: EventName::KeyDown,
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

                if app.measure_layout_on_next_render {
                    app.process_layout(window.inner_size(), scale_factor);
                    app.process_accessibility(window);

                    app.measure_layout_on_next_render = false;
                }
                //surface.canvas().clear(window_config.background);
                gl_context.make_current(gl_surface).unwrap();
                app.render(&self.hovered_node, surface.canvas(), dirty_surface, window);

                app.event_loop_tick();
                window.pre_present_notify();
                gr_context.flush_and_submit();
                gl_surface.swap_buffers(gl_context).unwrap();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                app.set_navigation_mode(NavigationMode::NotKeyboard);

                self.mouse_state = state;

                let name = match state {
                    ElementState::Pressed => EventName::MouseDown,
                    ElementState::Released => match button {
                        MouseButton::Middle => EventName::MiddleClick,
                        MouseButton::Right => EventName::RightClick,
                        MouseButton::Left => EventName::Click,
                        _ => EventName::PointerUp,
                    },
                };

                self.send_event(PlatformEvent::Mouse {
                    name,
                    cursor: self.cursor_pos,
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

                    self.send_event(PlatformEvent::Wheel {
                        name: EventName::Wheel,
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

                let name = match state {
                    ElementState::Pressed => EventName::KeyDown,
                    ElementState::Released => EventName::KeyUp,
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
                        name: EventName::MouseOver,
                        cursor: self.cursor_pos,
                        button: None,
                    });
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = CursorPoint::from((position.x, position.y));

                self.send_event(PlatformEvent::Mouse {
                    name: EventName::MouseOver,
                    cursor: self.cursor_pos,
                    button: None,
                });

                if let Some(dropped_file_path) = self.dropped_file_path.take() {
                    self.send_event(PlatformEvent::File {
                        name: EventName::FileDrop,
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
                    TouchPhase::Cancelled => EventName::TouchCancel,
                    TouchPhase::Ended => EventName::TouchEnd,
                    TouchPhase::Moved => EventName::TouchMove,
                    TouchPhase::Started => EventName::TouchStart,
                };

                self.send_event(PlatformEvent::Touch {
                    name,
                    location: self.cursor_pos,
                    finger_id: id,
                    phase,
                    force,
                });
            }
            WindowEvent::Resized(size) => {
                *surface =
                    create_surface(window, *fb_info, gr_context, *num_samples, *stencil_size);

                *dirty_surface = surface
                    .new_surface_with_dimensions((
                        size.width.try_into().expect("Could not convert width"),
                        size.height.try_into().expect("Could not convert height"),
                    ))
                    .unwrap();

                gl_surface.resize(
                    gl_context,
                    NonZeroU32::new(size.width.max(1)).unwrap(),
                    NonZeroU32::new(size.height.max(1)).unwrap(),
                );

                window.request_redraw();

                app.resize(window);
            }
            WindowEvent::DroppedFile(file_path) => {
                self.dropped_file_path = Some(file_path);
            }
            WindowEvent::HoveredFile(file_path) => {
                self.send_event(PlatformEvent::File {
                    name: EventName::GlobalFileHover,
                    file_path: Some(file_path),
                    cursor: self.cursor_pos,
                });
            }
            WindowEvent::HoveredFileCancelled => {
                self.send_event(PlatformEvent::File {
                    name: EventName::GlobalFileHoverCancelled,
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

impl<T: Clone> Drop for DesktopRenderer<'_, T> {
    fn drop(&mut self) {
        if let WindowState::Created(CreatedState {
            gl_context,
            gl_surface,
            gr_context,
            ..
        }) = &mut self.state
        {
            if !gl_context.is_current() && gl_context.make_current(gl_surface).is_err() {
                gr_context.abandon();
            }
        }
    }
}
