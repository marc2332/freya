use std::borrow::Cow;

use accesskit_winit::WindowEvent as AccessibilityWindowEvent;
use freya_core::integration::*;
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use futures_util::{
    FutureExt,
    StreamExt,
    select,
};
use ragnarok::{
    EventsExecutorRunner,
    EventsMeasurerRunner,
};
use rustc_hash::FxHashMap;
use torin::prelude::{
    CursorPoint,
    Size2D,
};
use winit::{
    application::ApplicationHandler,
    event::{
        ElementState,
        MouseScrollDelta,
        Touch,
        TouchPhase,
        WindowEvent,
    },
    event_loop::EventLoopProxy,
    window::WindowId,
};

use crate::{
    accessibility::AccessibilityTask,
    config::WindowConfig,
    plugins::{
        PluginEvent,
        PluginHandle,
        PluginsManager,
    },
    window::AppWindow,
    winit_mappings::{
        self,
        map_winit_mouse_button,
        map_winit_touch_force,
        map_winit_touch_phase,
    },
};

pub struct WinitRenderer {
    pub windows_configs: Vec<WindowConfig>,
    pub resumed: bool,
    pub windows: FxHashMap<WindowId, AppWindow>,
    pub proxy: EventLoopProxy<NativeEvent>,
    pub plugins: PluginsManager,
    pub fallback_fonts: Vec<Cow<'static, str>>,
    pub screen_reader: ScreenReader,
    pub font_manager: FontMgr,
    pub font_collection: FontCollection,
}

#[derive(Debug)]
pub enum NativeWindowEventAction {
    PollRunner,

    Accessibility(AccessibilityWindowEvent),

    PlatformEvent(PlatformEvent),

    User(UserEvent),
}

#[derive(Debug)]
pub struct NativeWindowEvent {
    pub window_id: WindowId,
    pub action: NativeWindowEventAction,
}

#[derive(Debug)]
pub enum NativeEvent {
    Window(NativeWindowEvent),
}

impl From<accesskit_winit::Event> for NativeEvent {
    fn from(event: accesskit_winit::Event) -> Self {
        NativeEvent::Window(NativeWindowEvent {
            window_id: event.window_id,
            action: NativeWindowEventAction::Accessibility(event.window_event),
        })
    }
}

impl ApplicationHandler<NativeEvent> for WinitRenderer {
    fn resumed(&mut self, active_event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.resumed {
            for window_config in self.windows_configs.drain(..) {
                let app_window = AppWindow::new(
                    window_config,
                    active_event_loop,
                    &self.proxy,
                    &mut self.plugins,
                    &self.font_collection,
                    &self.font_manager,
                    &self.fallback_fonts,
                    self.screen_reader.clone(),
                );

                self.proxy
                    .send_event(NativeEvent::Window(NativeWindowEvent {
                        window_id: app_window.window.id(),
                        action: NativeWindowEventAction::PollRunner,
                    }))
                    .ok();

                self.windows.insert(app_window.window.id(), app_window);
            }
            self.resumed = true;
        }
    }

    fn user_event(
        &mut self,
        active_event_loop: &winit::event_loop::ActiveEventLoop,
        event: NativeEvent,
    ) {
        match event {
            NativeEvent::Window(NativeWindowEvent { action, window_id }) => {
                if let Some(app) = &mut self.windows.get_mut(&window_id) {
                    match action {
                        NativeWindowEventAction::PollRunner => {
                            let mut cx = std::task::Context::from_waker(&app.waker);

                            {
                                let fut = std::pin::pin!(async {
                                    select! {
                                        events_chunk = app.events_receiver.next() => {
                                            match events_chunk {
                                                Some(EventsChunk::Processed(processed_events)) => {
                                                    let events_executor_adapter = EventsExecutorAdapter {
                                                        runner: &mut app.runner,
                                                    };
                                                    events_executor_adapter.run(&mut app.nodes_state, processed_events);
                                                }
                                                Some(EventsChunk::Batch(events)) => {
                                                    for event in events {
                                                        app.runner.handle_event(event.node_id, event.name, event.data, event.bubbles);
                                                    }
                                                }
                                                _ => {}
                                            }

                                        },
                                         _ = app.runner.handle_events().fuse() => {},
                                    }
                                });

                                match fut.poll(&mut cx) {
                                    std::task::Poll::Ready(_) => {
                                        self.proxy
                                            .send_event(NativeEvent::Window(NativeWindowEvent {
                                                window_id: app.window.id(),
                                                action: NativeWindowEventAction::PollRunner,
                                            }))
                                            .ok();
                                    }
                                    std::task::Poll::Pending => {}
                                }
                            }

                            self.plugins.send(
                                PluginEvent::StartedUpdatingTree {
                                    window: &app.window,
                                    tree: &app.tree,
                                },
                                PluginHandle::new(&self.proxy),
                            );
                            let mutations = app.runner.sync_and_update();
                            let result = app.tree.apply_mutations(mutations);
                            if result.needs_render {
                                app.process_layout_on_next_render = true;
                                app.window.request_redraw();
                            }
                            self.plugins.send(
                                PluginEvent::FinishedUpdatingTree {
                                    window: &app.window,
                                    tree: &app.tree,
                                },
                                PluginHandle::new(&self.proxy),
                            );
                        }
                        NativeWindowEventAction::Accessibility(
                            accesskit_winit::WindowEvent::AccessibilityDeactivated,
                        ) => {
                            self.screen_reader.set(false);
                        }
                        NativeWindowEventAction::Accessibility(
                            accesskit_winit::WindowEvent::ActionRequested(_),
                        ) => {}
                        NativeWindowEventAction::Accessibility(
                            accesskit_winit::WindowEvent::InitialTreeRequested,
                        ) => {
                            app.accessibility_tasks_for_next_render = AccessibilityTask::Init;
                            app.window.request_redraw();
                            self.screen_reader.set(true);
                        }
                        NativeWindowEventAction::User(user_event) => match user_event {
                            UserEvent::RequestRedraw => {
                                app.window.request_redraw();
                            }
                            UserEvent::FocusAccessibilityNode(strategy) => {
                                let task = match strategy {
                                    AccessibilityFocusStrategy::Backward(_)
                                    | AccessibilityFocusStrategy::Forward(_) => {
                                        AccessibilityTask::ProcessUpdate {
                                            mode: Some(NavigationMode::Keyboard),
                                        }
                                    }
                                    _ => AccessibilityTask::ProcessUpdate { mode: None },
                                };
                                app.tree.accessibility_diff.request_focus(strategy);
                                app.accessibility_tasks_for_next_render = task;
                                app.window.request_redraw();
                            }
                            UserEvent::SetCursorIcon(cursor_icon) => {
                                app.window.set_cursor(cursor_icon);
                            }
                            UserEvent::Erased(data) => {
                                if let Ok(window_config) = data.0.downcast::<WindowConfig>() {
                                    let app_window = AppWindow::new(
                                        *window_config,
                                        active_event_loop,
                                        &self.proxy,
                                        &mut self.plugins,
                                        &self.font_collection,
                                        &self.font_manager,
                                        &self.fallback_fonts,
                                        self.screen_reader.clone(),
                                    );

                                    self.proxy
                                        .send_event(NativeEvent::Window(NativeWindowEvent {
                                            window_id: app_window.window.id(),
                                            action: NativeWindowEventAction::PollRunner,
                                        }))
                                        .ok();

                                    self.windows.insert(app_window.window.id(), app_window);
                                } else {
                                    unreachable!()
                                }
                            }
                        },
                        NativeWindowEventAction::PlatformEvent(platform_event) => {
                            let mut events_measurer_adapter = EventsMeasurerAdapter {
                                tree: &mut app.tree,
                                scale_factor: app.window.scale_factor(),
                            };
                            let processed_events = events_measurer_adapter.run(
                                &mut vec![platform_event],
                                &mut app.nodes_state,
                                app.accessibility.focused_node_id(),
                            );
                            app.events_sender
                                .unbounded_send(EventsChunk::Processed(processed_events))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(app) = &mut self.windows.get_mut(&window_id) {
            app.accessibility_adapter.process_event(&app.window, &event);
            match event {
                WindowEvent::ScaleFactorChanged { .. } => {
                    app.window.request_redraw();
                    app.process_layout_on_next_render = true;
                    app.tree.layout.reset();
                }
                WindowEvent::CloseRequested => {
                    self.windows.remove(&window_id);
                    if self.windows.is_empty() {
                        event_loop.exit();
                    }
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    app.modifiers_state = modifiers.state();
                }
                WindowEvent::RedrawRequested => {
                    hotpath::measure_block!("RedrawRequested", {
                        if app.process_layout_on_next_render {
                            self.plugins.send(
                                PluginEvent::StartedMeasuringLayout {
                                    window: &app.window,
                                    tree: &app.tree,
                                },
                                PluginHandle::new(&self.proxy),
                            );
                            let size: Size2D = (
                                app.window.inner_size().width as f32,
                                app.window.inner_size().height as f32,
                            )
                                .into();

                            app.tree.measure_layout(
                                size,
                                &self.font_collection,
                                &self.font_manager,
                                &app.events_sender,
                                app.window.scale_factor(),
                                &self.fallback_fonts,
                            );
                            app.platform_state.root_size.set_if_modified(size);
                            app.process_layout_on_next_render = false;
                            self.plugins.send(
                                PluginEvent::FinishedMeasuringLayout {
                                    window: &app.window,
                                    tree: &app.tree,
                                },
                                PluginHandle::new(&self.proxy),
                            );
                        }

                        app.driver.present(
                            app.window.inner_size().cast(),
                            &app.window,
                            |surface| {
                                self.plugins.send(
                                    PluginEvent::BeforeRender {
                                        window: &app.window,
                                        canvas: surface.canvas(),
                                        font_collection: &self.font_collection,
                                        tree: &app.tree,
                                    },
                                    PluginHandle::new(&self.proxy),
                                );

                                let render_pipeline = RenderPipeline {
                                    font_collection: &mut self.font_collection,
                                    font_manager: &self.font_manager,
                                    tree: &app.tree,
                                    canvas: surface.canvas(),
                                    scale_factor: app.window.scale_factor(),
                                };

                                render_pipeline.render();

                                self.plugins.send(
                                    PluginEvent::AfterRender {
                                        window: &app.window,
                                        canvas: surface.canvas(),
                                        font_collection: &self.font_collection,
                                        tree: &app.tree,
                                        animation_clock: &app.animation_clock,
                                    },
                                    PluginHandle::new(&self.proxy),
                                );
                                self.plugins.send(
                                    PluginEvent::BeforePresenting {
                                        window: &app.window,
                                        font_collection: &self.font_collection,
                                        tree: &app.tree,
                                    },
                                    PluginHandle::new(&self.proxy),
                                );
                            },
                        );
                        self.plugins.send(
                            PluginEvent::AfterPresenting {
                                window: &app.window,
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );

                        self.plugins.send(
                            PluginEvent::BeforeAccessibility {
                                window: &app.window,
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );

                        match app.accessibility_tasks_for_next_render.take() {
                            AccessibilityTask::ProcessUpdate { mode } => {
                                let update = app.accessibility.process_updates(&mut app.tree);
                                app.platform_state
                                    .focused_accessibility_id
                                    .set_if_modified(update.focus);
                                let node_id = app.accessibility.focused_node_id().unwrap();
                                let layout_node = app.tree.layout.get(&node_id).unwrap();
                                app.platform_state
                                    .focused_accessibility_node
                                    .set_if_modified(AccessibilityTree::create_node(
                                        node_id,
                                        layout_node,
                                        &app.tree,
                                    ));
                                if let Some(mode) = mode {
                                    app.platform_state.navigation_mode.set(mode);
                                }
                                app.accessibility_adapter.update_if_active(|| update);
                            }
                            AccessibilityTask::Init => {
                                let update = app.accessibility.init(&mut app.tree);
                                app.platform_state
                                    .focused_accessibility_id
                                    .set_if_modified(update.focus);
                                let node_id = app.accessibility.focused_node_id().unwrap();
                                let layout_node = app.tree.layout.get(&node_id).unwrap();
                                app.platform_state
                                    .focused_accessibility_node
                                    .set_if_modified(AccessibilityTree::create_node(
                                        node_id,
                                        layout_node,
                                        &app.tree,
                                    ));
                                app.accessibility_adapter.update_if_active(|| update);
                            }
                            AccessibilityTask::None => {}
                        }

                        self.plugins.send(
                            PluginEvent::AfterAccessibility {
                                window: &app.window,
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );

                        if app.ticker_sender.receiver_count() > 0 {
                            app.ticker_sender.broadcast_blocking(()).unwrap();
                        }

                        self.plugins.send(
                            PluginEvent::AfterRedraw {
                                window: &app.window,
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );
                    });
                }
                WindowEvent::Resized(size) => {
                    app.driver.resize(size);

                    app.window.request_redraw();

                    app.process_layout_on_next_render = true;
                    app.tree.layout.reset();
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    app.mouse_state = state;
                    app.platform_state
                        .navigation_mode
                        .set(NavigationMode::NotKeyboard);

                    let name = if state == ElementState::Pressed {
                        MouseEventName::MouseDown
                    } else {
                        MouseEventName::MouseUp
                    };
                    let platform_event = PlatformEvent::Mouse {
                        name,
                        cursor: (app.position.x, app.position.y).into(),
                        button: Some(map_winit_mouse_button(button)),
                    };
                    let mut events_measurer_adapter = EventsMeasurerAdapter {
                        tree: &mut app.tree,
                        scale_factor: app.window.scale_factor(),
                    };
                    let processed_events = events_measurer_adapter.run(
                        &mut vec![platform_event],
                        &mut app.nodes_state,
                        app.accessibility.focused_node_id(),
                    );
                    app.events_sender
                        .unbounded_send(EventsChunk::Processed(processed_events))
                        .unwrap();
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    let name = match event.state {
                        ElementState::Pressed => KeyboardEventName::KeyDown,
                        ElementState::Released => KeyboardEventName::KeyUp,
                    };
                    let platform_event = PlatformEvent::Keyboard {
                        name,
                        key: winit_mappings::map_winit_key(&event.logical_key),
                        code: winit_mappings::map_winit_physical_key(&event.physical_key),
                        modifiers: winit_mappings::map_winit_modifiers(app.modifiers_state),
                    };
                    let mut events_measurer_adapter = EventsMeasurerAdapter {
                        tree: &mut app.tree,
                        scale_factor: app.window.scale_factor(),
                    };
                    let processed_events = events_measurer_adapter.run(
                        &mut vec![platform_event],
                        &mut app.nodes_state,
                        app.accessibility.focused_node_id(),
                    );
                    app.events_sender
                        .unbounded_send(EventsChunk::Processed(processed_events))
                        .unwrap();
                }

                WindowEvent::MouseWheel { delta, phase, .. } => {
                    const WHEEL_SPEED_MODIFIER: f64 = 53.0;
                    const TOUCHPAD_SPEED_MODIFIER: f64 = 2.0;

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

                        let platform_event = PlatformEvent::Wheel {
                            name: WheelEventName::Wheel,
                            scroll: scroll_data.into(),
                            cursor: app.position,
                            source: WheelSource::Device,
                        };
                        let mut events_measurer_adapter = EventsMeasurerAdapter {
                            tree: &mut app.tree,
                            scale_factor: app.window.scale_factor(),
                        };
                        let processed_events = events_measurer_adapter.run(
                            &mut vec![platform_event],
                            &mut app.nodes_state,
                            app.accessibility.focused_node_id(),
                        );
                        app.events_sender
                            .unbounded_send(EventsChunk::Processed(processed_events))
                            .unwrap();
                    }
                }

                WindowEvent::CursorLeft { .. } => {
                    if app.mouse_state == ElementState::Released {
                        app.position = CursorPoint::from((-1., -1.));
                        let platform_event = PlatformEvent::Mouse {
                            name: MouseEventName::MouseMove,
                            cursor: app.position,
                            button: None,
                        };
                        let mut events_measurer_adapter = EventsMeasurerAdapter {
                            tree: &mut app.tree,
                            scale_factor: app.window.scale_factor(),
                        };
                        let processed_events = events_measurer_adapter.run(
                            &mut vec![platform_event],
                            &mut app.nodes_state,
                            app.accessibility.focused_node_id(),
                        );
                        app.events_sender
                            .unbounded_send(EventsChunk::Processed(processed_events))
                            .unwrap();
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let platform_event = PlatformEvent::Mouse {
                        name: MouseEventName::MouseMove,
                        cursor: (position.x, position.y).into(),
                        button: None,
                    };
                    let mut events_measurer_adapter = EventsMeasurerAdapter {
                        tree: &mut app.tree,
                        scale_factor: app.window.scale_factor(),
                    };
                    let processed_events = events_measurer_adapter.run(
                        &mut vec![platform_event],
                        &mut app.nodes_state,
                        app.accessibility.focused_node_id(),
                    );
                    app.events_sender
                        .unbounded_send(EventsChunk::Processed(processed_events))
                        .unwrap();
                    app.position = CursorPoint::from((position.x, position.y));
                }

                WindowEvent::Touch(Touch {
                    location,
                    phase,
                    id,
                    force,
                    ..
                }) => {
                    app.position = CursorPoint::from((location.x, location.y));

                    let name = match phase {
                        TouchPhase::Cancelled => TouchEventName::TouchCancel,
                        TouchPhase::Ended => TouchEventName::TouchEnd,
                        TouchPhase::Moved => TouchEventName::TouchMove,
                        TouchPhase::Started => TouchEventName::TouchStart,
                    };

                    let platform_event = PlatformEvent::Touch {
                        name,
                        location: app.position,
                        finger_id: id,
                        phase: map_winit_touch_phase(phase),
                        force: force.map(map_winit_touch_force),
                    };
                    let mut events_measurer_adapter = EventsMeasurerAdapter {
                        tree: &mut app.tree,
                        scale_factor: app.window.scale_factor(),
                    };
                    let processed_events = events_measurer_adapter.run(
                        &mut vec![platform_event],
                        &mut app.nodes_state,
                        app.accessibility.focused_node_id(),
                    );
                    app.events_sender
                        .unbounded_send(EventsChunk::Processed(processed_events))
                        .unwrap();
                    app.position = CursorPoint::from((location.x, location.y));
                }
                _ => {}
            }
        }
    }
}
