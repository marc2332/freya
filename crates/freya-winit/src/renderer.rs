use std::{
    borrow::Cow,
    fmt,
    pin::Pin,
    task::Waker,
};

use accesskit_winit::WindowEvent as AccessibilityWindowEvent;
use freya_core::integration::*;
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use futures_lite::future::FutureExt as _;
use futures_util::{
    FutureExt as _,
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
#[cfg(all(feature = "tray", not(target_os = "linux")))]
use tray_icon::TrayIcon;
use winit::{
    application::ApplicationHandler,
    dpi::{
        LogicalPosition,
        LogicalSize,
    },
    event::{
        ElementState,
        Ime,
        MouseScrollDelta,
        Touch,
        TouchPhase,
        WindowEvent,
    },
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    window::{
        Theme,
        Window,
        WindowId,
    },
};

use crate::{
    accessibility::AccessibilityTask,
    config::{
        CloseDecision,
        WindowConfig,
    },
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
    #[cfg(feature = "tray")]
    pub(crate) tray: (
        Option<crate::config::TrayIconGetter>,
        Option<crate::config::TrayHandler>,
    ),
    #[cfg(all(feature = "tray", not(target_os = "linux")))]
    pub(crate) tray_icon: Option<TrayIcon>,
    pub resumed: bool,
    pub windows: FxHashMap<WindowId, AppWindow>,
    pub proxy: EventLoopProxy<NativeEvent>,
    pub plugins: PluginsManager,
    pub fallback_fonts: Vec<Cow<'static, str>>,
    pub screen_reader: ScreenReader,
    pub font_manager: FontMgr,
    pub font_collection: FontCollection,
    pub futures: Vec<Pin<Box<dyn std::future::Future<Output = ()>>>>,
    pub waker: Waker,
}

pub struct RendererContext<'a> {
    pub windows: &'a mut FxHashMap<WindowId, AppWindow>,
    pub proxy: &'a mut EventLoopProxy<NativeEvent>,
    pub plugins: &'a mut PluginsManager,
    pub fallback_fonts: &'a mut Vec<Cow<'static, str>>,
    pub screen_reader: &'a mut ScreenReader,
    pub font_manager: &'a mut FontMgr,
    pub font_collection: &'a mut FontCollection,
    pub(crate) active_event_loop: &'a ActiveEventLoop,
}

impl RendererContext<'_> {
    pub fn launch_window(&mut self, window_config: WindowConfig) -> WindowId {
        let app_window = AppWindow::new(
            window_config,
            self.active_event_loop,
            self.proxy,
            self.plugins,
            self.font_collection,
            self.font_manager,
            self.fallback_fonts,
            self.screen_reader.clone(),
        );

        let window_id = app_window.window.id();

        self.proxy
            .send_event(NativeEvent::Window(NativeWindowEvent {
                window_id,
                action: NativeWindowEventAction::PollRunner,
            }))
            .ok();

        self.windows.insert(window_id, app_window);

        window_id
    }

    pub fn windows(&self) -> &FxHashMap<WindowId, AppWindow> {
        self.windows
    }

    pub fn windows_mut(&mut self) -> &mut FxHashMap<WindowId, AppWindow> {
        self.windows
    }

    pub fn exit(&mut self) {
        self.active_event_loop.exit();
    }
}

#[derive(Debug)]
pub enum NativeWindowEventAction {
    PollRunner,

    Accessibility(AccessibilityWindowEvent),

    PlatformEvent(PlatformEvent),

    User(UserEvent),
}

pub struct WithWindowCallback(pub(crate) Box<dyn FnOnce(&mut Window)>);

impl fmt::Debug for WithWindowCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WithWindowCallback")
    }
}

/// Proxy wrapper provided to launch tasks so they can post callbacks executed inside the renderer.
#[derive(Clone)]
pub struct LaunchProxy(pub EventLoopProxy<NativeEvent>);

impl LaunchProxy {
    /// Send a callback to the renderer to get access to [RendererContext].
    pub fn with<F, T: Send + 'static>(&self, f: F) -> futures_channel::oneshot::Receiver<T>
    where
        F: FnOnce(&mut RendererContext) -> T + Send + 'static,
    {
        let (tx, rx) = futures_channel::oneshot::channel::<T>();
        let cb = Box::new(move |ctx: &mut RendererContext| {
            let res = (f)(ctx);
            let _ = tx.send(res);
        });
        let _ = self
            .0
            .send_event(NativeEvent::Generic(NativeGenericEvent::RendererCallback(
                cb,
            )));
        rx
    }
}

#[derive(Debug)]
pub enum NativeWindowErasedEventAction {
    LaunchWindow {
        window_config: WindowConfig,
        ack: futures_channel::oneshot::Sender<WindowId>,
    },
    CloseWindow(WindowId),
    WithWindow {
        window_id: Option<WindowId>,
        callback: WithWindowCallback,
    },
}

#[derive(Debug)]
pub struct NativeWindowEvent {
    pub window_id: WindowId,
    pub action: NativeWindowEventAction,
}

#[cfg(feature = "tray")]
#[derive(Debug)]
pub enum NativeTrayEventAction {
    TrayEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    LaunchWindow(SingleThreadErasedEvent),
}

#[cfg(feature = "tray")]
#[derive(Debug)]
pub struct NativeTrayEvent {
    pub action: NativeTrayEventAction,
}

pub enum NativeGenericEvent {
    PollFutures,
    RendererCallback(Box<dyn FnOnce(&mut RendererContext) + Send + 'static>),
}

impl fmt::Debug for NativeGenericEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NativeGenericEvent::PollFutures => f.write_str("PollFutures"),
            NativeGenericEvent::RendererCallback(_) => f.write_str("RendererCallback"),
        }
    }
}

#[derive(Debug)]
pub enum NativeEvent {
    Window(NativeWindowEvent),
    #[cfg(feature = "tray")]
    Tray(NativeTrayEvent),
    Generic(NativeGenericEvent),
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
            #[cfg(feature = "tray")]
            {
                #[cfg(not(target_os = "linux"))]
                if let Some(tray_icon) = self.tray.0.take() {
                    self.tray_icon = Some((tray_icon)());
                }

                #[cfg(target_os = "macos")]
                {
                    use objc2_core_foundation::CFRunLoop;

                    let rl = CFRunLoop::main().expect("Failed to run CFRunLoop");
                    CFRunLoop::wake_up(&rl);
                }
            }

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

            let _ = self
                .proxy
                .send_event(NativeEvent::Generic(NativeGenericEvent::PollFutures));
        }
    }

    fn user_event(
        &mut self,
        active_event_loop: &winit::event_loop::ActiveEventLoop,
        event: NativeEvent,
    ) {
        match event {
            NativeEvent::Generic(NativeGenericEvent::RendererCallback(cb)) => {
                let mut renderer_context = RendererContext {
                    fallback_fonts: &mut self.fallback_fonts,
                    active_event_loop,
                    windows: &mut self.windows,
                    proxy: &mut self.proxy,
                    plugins: &mut self.plugins,
                    screen_reader: &mut self.screen_reader,
                    font_manager: &mut self.font_manager,
                    font_collection: &mut self.font_collection,
                };
                (cb)(&mut renderer_context);
            }
            NativeEvent::Generic(NativeGenericEvent::PollFutures) => {
                let mut cx = std::task::Context::from_waker(&self.waker);
                self.futures
                    .retain_mut(|fut| fut.poll(&mut cx).is_pending());
            }
            #[cfg(feature = "tray")]
            NativeEvent::Tray(NativeTrayEvent { action }) => {
                let renderer_context = RendererContext {
                    fallback_fonts: &mut self.fallback_fonts,
                    active_event_loop,
                    windows: &mut self.windows,
                    proxy: &mut self.proxy,
                    plugins: &mut self.plugins,
                    screen_reader: &mut self.screen_reader,
                    font_manager: &mut self.font_manager,
                    font_collection: &mut self.font_collection,
                };
                match action {
                    NativeTrayEventAction::TrayEvent(icon_event) => {
                        use crate::tray::TrayEvent;
                        if let Some(tray_handler) = &mut self.tray.1 {
                            (tray_handler)(TrayEvent::Icon(icon_event), renderer_context)
                        }
                    }
                    NativeTrayEventAction::MenuEvent(menu_event) => {
                        use crate::tray::TrayEvent;
                        if let Some(tray_handler) = &mut self.tray.1 {
                            (tray_handler)(TrayEvent::Menu(menu_event), renderer_context)
                        }
                    }
                    NativeTrayEventAction::LaunchWindow(data) => {
                        let window_config = data
                            .0
                            .downcast::<WindowConfig>()
                            .expect("Expected WindowConfig");
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
                    }
                }
            }
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
                            #[cfg(debug_assertions)]
                            {
                                tracing::info!("Updated app tree.");
                                tracing::info!("{:#?}", app.tree);
                                tracing::info!("{:#?}", app.runner);
                            }
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
                                let action = data
                                    .0
                                    .downcast::<NativeWindowErasedEventAction>()
                                    .expect("Expected NativeWindowErasedEventAction");
                                match *action {
                                    NativeWindowErasedEventAction::LaunchWindow {
                                        window_config,
                                        ack,
                                    } => {
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

                                        let window_id = app_window.window.id();

                                        let _ = self.proxy.send_event(NativeEvent::Window(
                                            NativeWindowEvent {
                                                window_id,
                                                action: NativeWindowEventAction::PollRunner,
                                            },
                                        ));

                                        self.windows.insert(window_id, app_window);
                                        let _ = ack.send(window_id);
                                    }
                                    NativeWindowErasedEventAction::CloseWindow(window_id) => {
                                        // Its fine to ignore if the window doesnt exist anymore
                                        let _ = self.windows.remove(&window_id);
                                        let has_windows = !self.windows.is_empty();

                                        let has_tray = {
                                            #[cfg(feature = "tray")]
                                            {
                                                self.tray.1.is_some()
                                            }
                                            #[cfg(not(feature = "tray"))]
                                            {
                                                false
                                            }
                                        };

                                        // Only exit when there is no window and no tray
                                        if !has_windows && !has_tray {
                                            active_event_loop.exit();
                                        }
                                    }
                                    NativeWindowErasedEventAction::WithWindow {
                                        window_id,
                                        callback,
                                    } => {
                                        if let Some(window_id) = window_id {
                                            if let Some(app) = self.windows.get_mut(&window_id) {
                                                (callback.0)(&mut app.window)
                                            }
                                        } else {
                                            (callback.0)(&mut app.window)
                                        }
                                    }
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
                WindowEvent::ThemeChanged(theme) => {
                    app.platform.preferred_theme.set(match theme {
                        Theme::Light => PreferredTheme::Light,
                        Theme::Dark => PreferredTheme::Dark,
                    });
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    app.window.request_redraw();
                    app.process_layout_on_next_render = true;
                    app.tree.layout.reset();
                }
                WindowEvent::CloseRequested => {
                    let mut on_close_hook = self
                        .windows
                        .get_mut(&window_id)
                        .and_then(|app| app.on_close.take());

                    let decision = if let Some(ref mut on_close) = on_close_hook {
                        let renderer_context = RendererContext {
                            fallback_fonts: &mut self.fallback_fonts,
                            active_event_loop: event_loop,
                            windows: &mut self.windows,
                            proxy: &mut self.proxy,
                            plugins: &mut self.plugins,
                            screen_reader: &mut self.screen_reader,
                            font_manager: &mut self.font_manager,
                            font_collection: &mut self.font_collection,
                        };
                        on_close(renderer_context, window_id)
                    } else {
                        CloseDecision::Close
                    };

                    if matches!(decision, CloseDecision::KeepOpen)
                        && let Some(app) = self.windows.get_mut(&window_id)
                    {
                        app.on_close = on_close_hook;
                    }

                    if matches!(decision, CloseDecision::Close) {
                        self.windows.remove(&window_id);
                        let has_windows = !self.windows.is_empty();

                        let has_tray = {
                            #[cfg(feature = "tray")]
                            {
                                self.tray.1.is_some()
                            }
                            #[cfg(not(feature = "tray"))]
                            {
                                false
                            }
                        };

                        // Only exit when there is no windows and no tray
                        if !has_windows && !has_tray {
                            event_loop.exit();
                        }
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
                            app.platform.root_size.set_if_modified(size);
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
                                    background: app.background,
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
                                let update = app
                                    .accessibility
                                    .process_updates(&mut app.tree, &app.events_sender);
                                app.platform
                                    .focused_accessibility_id
                                    .set_if_modified(update.focus);
                                let node_id = app.accessibility.focused_node_id().unwrap();
                                let layout_node = app.tree.layout.get(&node_id).unwrap();
                                app.platform.focused_accessibility_node.set_if_modified(
                                    AccessibilityTree::create_node(node_id, layout_node, &app.tree),
                                );
                                if let Some(mode) = mode {
                                    app.platform.navigation_mode.set(mode);
                                }

                                let area = layout_node.visible_area();
                                app.window.set_ime_cursor_area(
                                    LogicalPosition::new(area.min_x(), area.min_y()),
                                    LogicalSize::new(area.width(), area.height()),
                                );

                                app.accessibility_adapter.update_if_active(|| update);
                            }
                            AccessibilityTask::Init => {
                                let update = app.accessibility.init(&mut app.tree);
                                app.platform
                                    .focused_accessibility_id
                                    .set_if_modified(update.focus);
                                let node_id = app.accessibility.focused_node_id().unwrap();
                                let layout_node = app.tree.layout.get(&node_id).unwrap();
                                app.platform.focused_accessibility_node.set_if_modified(
                                    AccessibilityTree::create_node(node_id, layout_node, &app.tree),
                                );

                                let area = layout_node.visible_area();
                                app.window.set_ime_cursor_area(
                                    LogicalPosition::new(area.min_x(), area.min_y()),
                                    LogicalSize::new(area.width(), area.height()),
                                );

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
                    app.tree.layout.clear_dirty();
                    app.tree.layout.invalidate(NodeId::ROOT);
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    app.mouse_state = state;
                    app.platform
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
                    app.position = CursorPoint::from((position.x, position.y));

                    let mut platform_event = vec![PlatformEvent::Mouse {
                        name: MouseEventName::MouseMove,
                        cursor: app.position,
                        button: None,
                    }];

                    for dropped_file_path in app.dropped_file_paths.drain(..) {
                        platform_event.push(PlatformEvent::File {
                            name: FileEventName::FileDrop,
                            file_path: Some(dropped_file_path),
                            cursor: app.position,
                        });
                    }

                    let mut events_measurer_adapter = EventsMeasurerAdapter {
                        tree: &mut app.tree,
                        scale_factor: app.window.scale_factor(),
                    };
                    let processed_events = events_measurer_adapter.run(
                        &mut platform_event,
                        &mut app.nodes_state,
                        app.accessibility.focused_node_id(),
                    );
                    app.events_sender
                        .unbounded_send(EventsChunk::Processed(processed_events))
                        .unwrap();
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
                WindowEvent::Ime(Ime::Preedit(text, pos)) => {
                    let platform_event = PlatformEvent::ImePreedit {
                        name: ImeEventName::Preedit,
                        text,
                        cursor: pos,
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
                WindowEvent::DroppedFile(file_path) => {
                    app.dropped_file_paths.push(file_path);
                }
                WindowEvent::HoveredFile(file_path) => {
                    let platform_event = PlatformEvent::File {
                        name: FileEventName::FileHover,
                        file_path: Some(file_path),
                        cursor: app.position,
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
                WindowEvent::HoveredFileCancelled => {
                    let platform_event = PlatformEvent::File {
                        name: FileEventName::FileHoverCancelled,
                        file_path: None,
                        cursor: app.position,
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
                _ => {}
            }
        }
    }
}
