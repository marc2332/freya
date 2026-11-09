use std::{
    borrow::Cow,
    fmt,
    io,
    rc::Rc,
    time::Instant,
};

#[cfg(feature = "accessibility")]
use accesskit_xplat::WindowEvent as AccessibilityWindowEvent;
use freya_clipboard::prelude::ClipboardError;
use freya_components::cache::AssetCacher;
use freya_core::{
    integration::*,
    metrics::Metrics,
    prelude::GlobalTasks,
};
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
    SkData,
    TypefaceFontProvider,
    register_font_typeface,
};
use futures_channel::oneshot;
use rustc_hash::FxHashMap;
use torin::prelude::CursorPoint;
#[cfg(feature = "tray")]
use tray_icon::TrayIcon;
#[cfg(feature = "accessibility")]
use winit::dpi::{
    LogicalPosition,
    LogicalSize,
};
use winit::{
    application::ApplicationHandler,
    event::{
        ButtonSource,
        ElementState,
        Ime,
        MouseScrollDelta,
        PointerKind,
        PointerSource,
        TouchPhase,
        WindowEvent,
    },
    event_loop::{
        ActiveEventLoop,
        AsyncRequestSerial,
        ControlFlow,
        DndAction,
        EventLoopProxy,
    },
    window::{
        Theme,
        WindowId,
    },
};

#[cfg(feature = "accessibility")]
use crate::integration::is_ime_role;
use crate::{
    accessibility::AccessibilityTask,
    clipboard::PendingClipboardRead,
    config::{
        CloseDecision,
        WindowConfig,
    },
    data_transfer::{
        DataFormatExt,
        TransferDataExt,
    },
    drivers::{
        GraphicsContext,
        GraphicsDriver,
    },
    native_drag::IncomingDrag,
    plugins::{
        PluginEvent,
        PluginHandle,
        PluginsManager,
    },
    window::{
        AppWindow,
        frame_interval,
    },
    winit_mappings::{
        self,
        map_winit_mouse_button,
        map_winit_touch_force,
        map_winit_touch_phase,
    },
};

#[derive(Clone)]
pub struct NativeEventProxy {
    proxy: EventLoopProxy,
    sender: std::sync::mpsc::Sender<NativeEvent>,
}

impl NativeEventProxy {
    pub(crate) fn new(proxy: EventLoopProxy, sender: std::sync::mpsc::Sender<NativeEvent>) -> Self {
        Self { proxy, sender }
    }

    pub fn send_event(
        &self,
        event: NativeEvent,
    ) -> Result<(), std::sync::mpsc::SendError<NativeEvent>> {
        self.sender.send(event)?;
        self.proxy.wake_up();
        Ok(())
    }
}

pub struct WinitRenderer {
    pub windows_configs: Vec<WindowConfig>,
    #[cfg(feature = "tray")]
    pub(crate) tray: (
        Option<crate::config::TrayIconGetter>,
        Option<crate::config::TrayHandler>,
    ),
    #[cfg(feature = "tray")]
    pub(crate) tray_icon: Option<TrayIcon>,
    pub resumed: bool,
    pub windows: FxHashMap<WindowId, AppWindow>,
    pub global_contexts: GlobalContexts,
    pub proxy: NativeEventProxy,
    pub(crate) events: std::sync::mpsc::Receiver<NativeEvent>,
    pub(crate) clipboard_reads: FxHashMap<AsyncRequestSerial, PendingClipboardRead>,
    pub plugins: PluginsManager,
    pub fallback_fonts: Vec<Cow<'static, str>>,
    pub font_provider: TypefaceFontProvider,
    pub font_manager: FontMgr,
    pub font_collection: FontCollection,
    pub exit_on_close: bool,
    pub gpu_resource_cache_limit: usize,
    pub graphics_context: GraphicsContext,
}

impl WinitRenderer {
    /// Processes buffered input when its deadline is due and requests redraws for UI changes.
    fn process_pending_events(&mut self) {
        let now = Instant::now();
        for app in self.windows.values_mut() {
            if !app
                .event_buffer
                .deadline()
                .is_some_and(|deadline| deadline <= now)
            {
                continue;
            }
            app.event_buffer.interval = frame_interval(app.window.as_ref());
            if app.process_buffered_events(
                &mut self.font_collection,
                &self.font_manager,
                &self.fallback_fonts,
                &mut self.plugins,
                PluginHandle::new(&self.proxy),
            ) {
                app.window.request_redraw();
            }
        }
    }
}

impl Drop for WinitRenderer {
    fn drop(&mut self) {
        let tasks = self.global_contexts.get_context::<GlobalTasks>();
        tasks.clear();
        for window_id in self.windows.keys() {
            Platform::get().unregister_window(window_id.into_raw() as u64);
        }
        self.windows.clear();
        // Cancel cleanup tasks spawned during window teardown.
        tasks.clear();
        self.global_contexts.unregister();
    }
}

pub struct RendererContext<'a> {
    pub windows: &'a mut FxHashMap<WindowId, AppWindow>,
    pub global_contexts: &'a GlobalContexts,
    pub proxy: &'a mut NativeEventProxy,
    pub plugins: &'a mut PluginsManager,
    pub fallback_fonts: &'a mut Vec<Cow<'static, str>>,
    pub font_manager: &'a mut FontMgr,
    pub font_collection: &'a mut FontCollection,
    pub active_event_loop: &'a dyn ActiveEventLoop,
    pub gpu_resource_cache_limit: usize,
    pub graphics_context: &'a mut GraphicsContext,
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
            self.gpu_resource_cache_limit,
            self.graphics_context,
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

    #[cfg(feature = "accessibility")]
    Accessibility(AccessibilityWindowEvent),

    PlatformEvent(PlatformEvent),

    User(UserEvent),
}

/// Proxy wrapper provided to launch tasks so they can post callbacks executed inside the renderer.
#[derive(Clone)]
pub struct LaunchProxy(pub NativeEventProxy);

impl LaunchProxy {
    /// Queue a callback to be run on the renderer thread with access to a [`RendererContext`].
    ///
    /// The call dispatches an event to the winit event loop and returns right away. The
    /// callback runs later, when the event loop picks it up. Its return value is delivered
    /// through the returned oneshot [`Receiver`](futures_channel::oneshot::Receiver), which
    /// can be `.await`ed or dropped.
    ///
    /// The callback runs outside a window context. Use explicit window IDs for window APIs.
    pub fn post_callback<F, T: 'static>(&self, f: F) -> futures_channel::oneshot::Receiver<T>
    where
        F: FnOnce(&mut RendererContext) -> T + 'static,
    {
        let (tx, rx) = futures_channel::oneshot::channel::<T>();
        let cb = Box::new(move |ctx: &mut RendererContext| {
            let res = (f)(ctx);
            let _ = tx.send(res);
        });
        let _ = self
            .0
            .send_event(NativeEvent::Generic(NativeGenericEvent::User(
                GlobalUserEvent::Erased(SingleThreadErasedEvent(Box::new(
                    NativePlatformErasedEventAction::RendererCallback(cb),
                ))),
            )));
        rx
    }
}

pub type RendererCallback = Box<dyn FnOnce(&mut RendererContext) + 'static>;

pub enum NativePlatformErasedEventAction {
    LaunchWindow {
        window_config: Box<WindowConfig>,
        ack: futures_channel::oneshot::Sender<WindowId>,
    },
    CloseWindow(WindowId),
    RendererCallback(RendererCallback),
    ReadClipboard {
        format: DataFormat,
        response: oneshot::Sender<Result<DataValue, ClipboardError>>,
    },
    WriteClipboard {
        data: TransferData,
        response: oneshot::Sender<Result<(), ClipboardError>>,
    },
}

#[derive(Debug)]
pub struct NativeWindowEvent {
    pub window_id: WindowId,
    pub action: NativeWindowEventAction,
}

pub enum NativeGenericEvent {
    PollFutures,
    User(GlobalUserEvent),
}

impl fmt::Debug for NativeGenericEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NativeGenericEvent::PollFutures => f.write_str("PollFutures"),
            NativeGenericEvent::User(event) => f.debug_tuple("User").field(event).finish(),
        }
    }
}

/// # Safety
/// The values are never sent, received or accessed by other threads other than the main thread.
/// This is needed to send `Rc<T>` and other non-Send and non-Sync values.
unsafe impl Send for NativeGenericEvent {}
unsafe impl Sync for NativeGenericEvent {}

#[derive(Debug)]
pub enum NativeEvent {
    Window(NativeWindowEvent),
    #[cfg(feature = "tray")]
    Tray(crate::tray_icon::TrayEvent),
    Generic(NativeGenericEvent),
    Preferences(mundy::Preferences),
}

#[cfg(feature = "accessibility")]
pub(crate) struct AccessibilityEventHandler {
    pub(crate) window_id: WindowId,
    pub(crate) proxy: NativeEventProxy,
}

#[cfg(feature = "accessibility")]
impl accesskit_xplat::EventHandler for AccessibilityEventHandler {
    fn handle_accesskit_event(&self, event: AccessibilityWindowEvent) {
        let _ = self
            .proxy
            .send_event(NativeEvent::Window(NativeWindowEvent {
                window_id: self.window_id,
                action: NativeWindowEventAction::Accessibility(event),
            }));
    }
}

impl ApplicationHandler for WinitRenderer {
    fn can_create_surfaces(&mut self, active_event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        if !self.resumed {
            #[cfg(feature = "tray")]
            {
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
                    &mut self.font_collection,
                    &self.font_manager,
                    &self.fallback_fonts,
                    self.gpu_resource_cache_limit,
                    &mut self.graphics_context,
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

            subscribe_preferences(self.proxy.clone());

            let _ = self
                .proxy
                .send_event(NativeEvent::Generic(NativeGenericEvent::PollFutures));
        } else {
            // [Android] Recreate the GraphicsDriver when the app gets brought into the foreground after being suspended,
            // so we don't end up with a completely black surface with broken rendering.
            let old_windows: Vec<_> = self.windows.drain().collect();
            for (old_id, mut app_window) in old_windows {
                let (new_driver, new_window) = GraphicsDriver::new(
                    active_event_loop,
                    app_window.window_attributes.clone(),
                    self.gpu_resource_cache_limit,
                    app_window.renderer,
                    &mut self.graphics_context,
                );

                let new_id = new_window.id();
                Platform::get().unregister_window(old_id.into_raw() as u64);
                app_window.platform.id = new_id.into_raw() as u64;
                app_window.platform.sender = Rc::new({
                    let proxy = self.proxy.clone();
                    move |event| {
                        let _ = proxy.send_event(NativeEvent::Window(NativeWindowEvent {
                            window_id: new_id,
                            action: NativeWindowEventAction::User(event),
                        }));
                    }
                });
                app_window
                    .runner
                    .provide_root_context(|| CurrentWindowId(app_window.platform.id));
                Platform::get().register_window(app_window.platform.clone());
                app_window.driver = new_driver;
                app_window.window = new_window;
                app_window.process_layout_on_next_render = true;
                app_window.tree.layout.reset();

                self.windows.insert(new_id, app_window);

                self.proxy
                    .send_event(NativeEvent::Window(NativeWindowEvent {
                        window_id: new_id,
                        action: NativeWindowEventAction::PollRunner,
                    }))
                    .ok();
            }
        }
    }

    fn proxy_wake_up(&mut self, active_event_loop: &dyn ActiveEventLoop) {
        while let Ok(event) = self.events.try_recv() {
            self.user_event(active_event_loop, event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.update_control_flow(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &dyn winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.process_window_event(event_loop, window_id, event);
    }
}

impl WinitRenderer {
    /// Processes buffered input, UI updates, and pending native drag requests.
    fn process_pending_updates(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.clipboard_reads
            .retain(|_, pending| !pending.response.is_canceled());
        self.process_pending_events();
        for app in self.windows.values_mut() {
            app.native_drag.process_pending(event_loop);
        }
    }

    fn user_event(
        &mut self,
        active_event_loop: &dyn winit::event_loop::ActiveEventLoop,
        event: NativeEvent,
    ) {
        match event {
            NativeEvent::Generic(NativeGenericEvent::PollFutures) => {
                GlobalTasks::get().poll();
            }
            NativeEvent::Preferences(prefs) => {
                for app in self.windows.values_mut() {
                    app.platform
                        .accent_color
                        .set_if_modified(prefs.accent_color);
                }
            }
            #[cfg(feature = "tray")]
            NativeEvent::Tray(event) => {
                let renderer_context = RendererContext {
                    fallback_fonts: &mut self.fallback_fonts,
                    active_event_loop,
                    windows: &mut self.windows,
                    global_contexts: &self.global_contexts,
                    proxy: &mut self.proxy,
                    plugins: &mut self.plugins,
                    font_manager: &mut self.font_manager,
                    font_collection: &mut self.font_collection,
                    gpu_resource_cache_limit: self.gpu_resource_cache_limit,
                    graphics_context: &mut self.graphics_context,
                };
                if let Some(tray_handler) = &mut self.tray.1 {
                    (tray_handler)(event, renderer_context);
                }
            }
            NativeEvent::Generic(NativeGenericEvent::User(GlobalUserEvent::LoadFont {
                font_name,
                font_data,
            })) => {
                let Some(typeface) = FontMgr::custom_empty()
                    .unwrap_or_default()
                    .new_from_data(SkData::new_copy(&font_data), None)
                else {
                    tracing::error!("Failed to load font {font_name}.");
                    return;
                };

                register_font_typeface(&mut self.font_provider, font_name.as_ref(), typeface);
                self.font_collection.clear_caches();

                for app in self.windows.values_mut() {
                    app.process_layout_on_next_render = true;
                    app.tree.layout.reset();
                    app.tree.text_cache.reset();
                    app.window.request_redraw();
                }
            }
            NativeEvent::Generic(NativeGenericEvent::User(event)) => match event {
                GlobalUserEvent::OpenUrl(url) => {
                    if let Err(error) = open::that(&url) {
                        tracing::error!(%error, %url, "Failed to open URL");
                    }
                }
                GlobalUserEvent::Exit => active_event_loop.exit(),
                GlobalUserEvent::LoadFont { .. } => unreachable!(),
                GlobalUserEvent::Erased(data) => {
                    let action = data
                        .0
                        .downcast::<NativePlatformErasedEventAction>()
                        .expect("Expected NativePlatformErasedEventAction");
                    match *action {
                        NativePlatformErasedEventAction::ReadClipboard { format, response } => {
                            let transfer = match active_event_loop.clipboard() {
                                Ok(Some(transfer)) => transfer,
                                Ok(None) => {
                                    let _ = response.send(Err(ClipboardError::UnsupportedFormat));
                                    return;
                                }
                                Err(_) => {
                                    let _ = response.send(Err(ClipboardError::NotAvailable));
                                    return;
                                }
                            };
                            if !active_event_loop
                                .data_transfer(transfer)
                                .is_ok_and(|data| data.has_type(&format.to_type_hint()))
                            {
                                let _ = response.send(Err(ClipboardError::UnsupportedFormat));
                                return;
                            }
                            match active_event_loop
                                .fetch_data_transfer(transfer, &format.to_type_hint())
                            {
                                Ok(serial) => {
                                    self.clipboard_reads.insert(
                                        serial,
                                        PendingClipboardRead {
                                            transfer,
                                            format,
                                            response,
                                        },
                                    );
                                }
                                Err(_) => {
                                    let _ = response.send(Err(ClipboardError::FailedToRead));
                                }
                            }
                        }
                        NativePlatformErasedEventAction::WriteClipboard { data, response } => {
                            let result = data
                                .into_winit()
                                .map_err(|_| ClipboardError::FailedToSet)
                                .and_then(|data| {
                                    active_event_loop
                                        .set_clipboard(data)
                                        .map_err(|_| ClipboardError::FailedToSet)
                                });
                            let _ = response.send(result);
                        }
                        NativePlatformErasedEventAction::LaunchWindow { window_config, ack } => {
                            let app_window = AppWindow::new(
                                *window_config,
                                active_event_loop,
                                &self.proxy,
                                &mut self.plugins,
                                &mut self.font_collection,
                                &self.font_manager,
                                &self.fallback_fonts,
                                self.gpu_resource_cache_limit,
                                &mut self.graphics_context,
                            );
                            let window_id = app_window.window.id();
                            let _ = self
                                .proxy
                                .send_event(NativeEvent::Window(NativeWindowEvent {
                                    window_id,
                                    action: NativeWindowEventAction::PollRunner,
                                }));
                            self.windows.insert(window_id, app_window);
                            let _ = ack.send(window_id);
                        }
                        NativePlatformErasedEventAction::CloseWindow(window_id) => {
                            Platform::get().unregister_window(window_id.into_raw() as u64);
                            if let Some(app) = self.windows.remove(&window_id) {
                                self.plugins.send(
                                    PluginEvent::WindowClosed {
                                        window: app.window.as_ref(),
                                        tree: &app.tree,
                                    },
                                    PluginHandle::new(&self.proxy),
                                );
                            }
                            #[cfg(feature = "tray")]
                            let has_tray = self.tray.1.is_some();
                            #[cfg(not(feature = "tray"))]
                            let has_tray = false;
                            if self.windows.is_empty() && !has_tray && self.exit_on_close {
                                active_event_loop.exit();
                            }
                        }
                        NativePlatformErasedEventAction::RendererCallback(callback) => {
                            let mut renderer_context = RendererContext {
                                fallback_fonts: &mut self.fallback_fonts,
                                active_event_loop,
                                windows: &mut self.windows,
                                proxy: &mut self.proxy,
                                plugins: &mut self.plugins,
                                font_manager: &mut self.font_manager,
                                font_collection: &mut self.font_collection,
                                gpu_resource_cache_limit: self.gpu_resource_cache_limit,
                                graphics_context: &mut self.graphics_context,
                                global_contexts: &self.global_contexts,
                            };
                            callback(&mut renderer_context);
                        }
                    }
                }
            },
            NativeEvent::Window(NativeWindowEvent { action, window_id }) => {
                if let Some(app) = &mut self.windows.get_mut(&window_id) {
                    match action {
                        NativeWindowEventAction::PollRunner => {
                            if app.poll_runner(&mut self.plugins, PluginHandle::new(&self.proxy)) {
                                app.window.request_redraw();
                            }
                        }
                        #[cfg(feature = "accessibility")]
                        NativeWindowEventAction::Accessibility(
                            AccessibilityWindowEvent::AccessibilityDeactivated,
                        ) => {
                            app.screen_reader.set(false);
                        }
                        #[cfg(feature = "accessibility")]
                        NativeWindowEventAction::Accessibility(
                            AccessibilityWindowEvent::ActionRequested(request),
                        ) => {
                            if app.accessibility.handle_action(
                                request,
                                &mut app.tree,
                                &app.events_sender,
                            ) {
                                app.accessibility_tasks_for_next_render |=
                                    AccessibilityTask::ProcessUpdate { mode: None };
                                app.window.request_redraw();
                            }
                        }
                        #[cfg(feature = "accessibility")]
                        NativeWindowEventAction::Accessibility(
                            AccessibilityWindowEvent::InitialTreeRequested,
                        ) => {
                            app.accessibility_tasks_for_next_render = AccessibilityTask::Init;
                            app.window.request_redraw();
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
                                if let AccessibilityFocusStrategy::Node(id) = &strategy {
                                    app.platform.focused_accessibility_id.set_if_modified(*id);
                                }
                                app.tree.accessibility_diff.request_focus(strategy);
                                app.accessibility_tasks_for_next_render |= task;
                                app.window.request_redraw();
                            }
                            UserEvent::SetCustomScaleFactor(custom_scale_factor) => {
                                app.set_custom_scale_factor(custom_scale_factor);
                            }
                            UserEvent::Erased(command) => {
                                if let Ok(command) = command.0.downcast::<NativeDragCommand>() {
                                    app.handle_native_drag(
                                        *command,
                                        active_event_loop,
                                        &mut self.font_collection,
                                        &self.font_manager,
                                        &self.fallback_fonts,
                                    );
                                }
                            }
                        },
                        NativeWindowEventAction::PlatformEvent(platform_event) => {
                            app.queue_platform_events(vec![platform_event]);
                        }
                    }
                }
            }
        }
    }

    fn update_control_flow(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.process_pending_updates(event_loop);
        if !matches!(event_loop.control_flow(), ControlFlow::Poll) {
            let deadline = self
                .windows
                .values()
                .filter_map(|app| app.event_buffer.deadline())
                .min();
            event_loop.set_control_flow(match deadline {
                Some(deadline) => ControlFlow::WaitUntil(deadline),
                None => ControlFlow::Wait,
            });
        }
    }

    fn process_window_event(
        &mut self,
        event_loop: &dyn winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::DataTransferReceived { id, serial, value } = &event
            && let Some(pending) = self.clipboard_reads.remove(serial)
        {
            let result = if pending.transfer == *id {
                match pending.format.read(value.as_ref()) {
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        self.clipboard_reads.insert(*serial, pending);
                        return;
                    }
                    result => result.map_err(|_| ClipboardError::FailedToRead),
                }
            } else {
                Err(ClipboardError::FailedToRead)
            };
            let _ = pending.response.send(result);
            return;
        }

        let mut needs_recovery = false;
        if let Some(app) = &mut self.windows.get_mut(&window_id) {
            #[cfg(feature = "accessibility")]
            app.process_accessibility_window_event(&event);
            match event {
                WindowEvent::DragEntered { id, position } => {
                    if let Some(position) = position {
                        app.position = CursorPoint::new(position.x, position.y);
                    }
                    app.native_drag.expire();
                    let Ok(transfer) = event_loop.data_transfer(id) else {
                        return;
                    };
                    let formats = [
                        DataFormat::Files,
                        DataFormat::Text,
                        DataFormat::Html,
                        DataFormat::Rtf,
                        DataFormat::Image,
                        DataFormat::Audio,
                    ]
                    .into_iter()
                    .filter(|format| transfer.has_type(&format.to_type_hint()))
                    .collect();
                    let data = DragDataHandle::new(formats);
                    app.native_drag.incoming = Some(IncomingDrag {
                        id,
                        data,
                        decision: None,
                        accepted: None,
                        dropped: false,
                    });
                    if let Some(event) = app.native_drag.prepare_drag_over(app.position) {
                        app.queue_platform_events(vec![event]);
                    }
                    self.process_pending_updates(event_loop);
                }
                WindowEvent::DragPosition { id, position, .. } => {
                    app.position = CursorPoint::new(position.x, position.y);
                    if !app
                        .native_drag
                        .incoming
                        .as_ref()
                        .is_some_and(|incoming| incoming.id == id)
                    {
                        return;
                    }
                    if let Some(event) = app.native_drag.prepare_drag_over(app.position) {
                        app.queue_platform_events(vec![event]);
                    }
                    self.process_pending_updates(event_loop);
                }
                WindowEvent::DragDropped { id, .. } => {
                    let Some(incoming) = app
                        .native_drag
                        .incoming
                        .as_mut()
                        .filter(|incoming| incoming.id == id)
                    else {
                        return;
                    };
                    incoming.dropped = true;
                    let target = incoming.accepted;
                    let events = vec![
                        PlatformEvent::Drag {
                            name: DragEventName::Drop,
                            cursor: app.position,
                            data: incoming.data.clone(),
                            decision: None,
                            target,
                        },
                        PlatformEvent::Drag {
                            name: DragEventName::Leave,
                            cursor: app.position,
                            data: incoming.data.clone(),
                            decision: None,
                            target: None,
                        },
                    ];
                    app.queue_platform_events(events);
                    self.process_pending_updates(event_loop);
                }
                WindowEvent::DragLeft { id } => {
                    let Some(incoming) = app
                        .native_drag
                        .incoming
                        .as_ref()
                        .filter(|incoming| incoming.id == id && !incoming.dropped)
                    else {
                        return;
                    };
                    let event = PlatformEvent::Drag {
                        name: DragEventName::Leave,
                        cursor: app.position,
                        data: incoming.data.clone(),
                        decision: None,
                        target: None,
                    };
                    app.native_drag.expire();
                    app.queue_platform_events(vec![event]);
                    self.process_pending_updates(event_loop);
                }
                WindowEvent::DataTransferReceived { id, serial, value } => {
                    app.native_drag.receive(id, serial, value.as_ref());
                }
                WindowEvent::OutgoingDragDropped { id, action } => {
                    let Some((_, response)) = app
                        .native_drag
                        .outgoing
                        .take_if(|(outgoing_id, _)| *outgoing_id == id)
                    else {
                        return;
                    };
                    let action = action.and_then(|action| match action {
                        DndAction::Copy => Some(DropAction::Copy),
                        DndAction::Move => Some(DropAction::Move),
                        DndAction::Link => Some(DropAction::Link),
                        _ => None,
                    });
                    let _ = response.send(Ok(NativeDragResult::Dropped(action)));
                    app.mouse_state = ElementState::Released;
                    app.queue_platform_events(vec![PlatformEvent::PointerCancel {
                        cursor: app.position,
                    }]);
                }
                WindowEvent::OutgoingDragCanceled { id } => {
                    let Some((_, response)) = app
                        .native_drag
                        .outgoing
                        .take_if(|(outgoing_id, _)| *outgoing_id == id)
                    else {
                        return;
                    };
                    let _ = response.send(Ok(NativeDragResult::Cancelled));
                    app.mouse_state = ElementState::Released;
                    app.queue_platform_events(vec![PlatformEvent::PointerCancel {
                        cursor: app.position,
                    }]);
                }
                WindowEvent::ThemeChanged(theme) => {
                    app.platform.preferred_theme.set(match theme {
                        Theme::Light => PreferredTheme::Light,
                        Theme::Dark => PreferredTheme::Dark,
                    });
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    app.scale_factor_changed();
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
                            global_contexts: &self.global_contexts,
                            proxy: &mut self.proxy,
                            plugins: &mut self.plugins,
                            font_manager: &mut self.font_manager,
                            font_collection: &mut self.font_collection,
                            gpu_resource_cache_limit: self.gpu_resource_cache_limit,
                            graphics_context: &mut self.graphics_context,
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
                        Platform::get().unregister_window(window_id.into_raw() as u64);
                        if let Some(app) = self.windows.remove(&window_id) {
                            self.plugins.send(
                                PluginEvent::WindowClosed {
                                    window: app.window.as_ref(),
                                    tree: &app.tree,
                                },
                                PluginHandle::new(&self.proxy),
                            );
                        }
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
                        if !has_windows && !has_tray && self.exit_on_close {
                            event_loop.exit();
                        }
                    }
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    app.modifiers_state = modifiers.state();
                }
                WindowEvent::Focused(is_focused) => {
                    app.platform.is_app_focused.set_if_modified(is_focused);

                    // Window switches (e.g. Alt+Tab) swallow key releases, so release held keys on focus loss.
                    if !is_focused && !app.pressed_keys.is_empty() {
                        let modifiers = winit_mappings::map_winit_modifiers(app.modifiers_state);
                        let platform_events: Vec<_> = app
                            .pressed_keys
                            .drain(..)
                            .map(|(key, code)| PlatformEvent::Keyboard {
                                name: KeyboardEventName::KeyUp,
                                key,
                                code,
                                modifiers,
                            })
                            .collect();
                        app.queue_platform_events(platform_events);
                    }
                }
                WindowEvent::RedrawRequested => {
                    let scale_factor = app.effective_scale_factor();
                    hotpath::measure_block!("RedrawRequested", {
                        app.measure_layout(
                            &mut self.font_collection,
                            &self.font_manager,
                            &self.fallback_fonts,
                            &mut self.plugins,
                            PluginHandle::new(&self.proxy),
                        );

                        let resource_cache = app.driver.resource_cache_usage();
                        let present_result = app.driver.present(
                            app.window.surface_size().cast(),
                            app.window.as_ref(),
                            |surface| {
                                self.plugins.send(
                                    PluginEvent::BeforeRender {
                                        window: app.window.as_ref(),
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
                                    scale_factor,
                                    background: app.background,
                                };

                                render_pipeline.render();

                                let cached_assets = app.runner.with_root_context(|| {
                                    AssetCacher::try_get()
                                        .map(|asset_cacher| asset_cacher.cached_size())
                                        .unwrap_or_default()
                                });
                                let metrics = Metrics::new(
                                    &app.runner,
                                    &app.tree,
                                    cached_assets,
                                    resource_cache,
                                );

                                self.plugins.send(
                                    PluginEvent::AfterRender {
                                        window: app.window.as_ref(),
                                        canvas: surface.canvas(),
                                        font_collection: &self.font_collection,
                                        tree: &app.tree,
                                        animation_clock: &app.animation_clock,
                                        metrics,
                                    },
                                    PluginHandle::new(&self.proxy),
                                );

                                for render_callback in app.render_callbacks.drain(..) {
                                    render_callback(&mut *surface);
                                }

                                self.plugins.send(
                                    PluginEvent::BeforePresenting {
                                        window: app.window.as_ref(),
                                        font_collection: &self.font_collection,
                                        tree: &app.tree,
                                    },
                                    PluginHandle::new(&self.proxy),
                                );
                            },
                        );
                        if let Err(error) = present_result {
                            tracing::warn!(
                                "Graphics driver lost ({error:?}), rebuilding on the same window"
                            );
                            needs_recovery = true;
                        }

                        self.plugins.send(
                            PluginEvent::AfterPresenting {
                                window: app.window.as_ref(),
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );

                        self.plugins.send(
                            PluginEvent::BeforeAccessibility {
                                window: app.window.as_ref(),
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );

                        match app.accessibility_tasks_for_next_render.take() {
                            AccessibilityTask::ProcessUpdate { mode } => {
                                app.process_accessibility_update(mode);
                            }
                            #[cfg(feature = "accessibility")]
                            AccessibilityTask::Init => {
                                let title = app.window.title();
                                let update = app.accessibility.init(&mut app.tree, &title);
                                app.platform
                                    .focused_accessibility_id
                                    .set_if_modified(update.focus);
                                let node_id = app.accessibility.focused_node_id().unwrap();
                                let layout_node = app.tree.layout.get(&node_id).unwrap();
                                let focused_node = AccessibilityTree::create_node(
                                    node_id,
                                    layout_node,
                                    &app.tree,
                                    &title,
                                );
                                let ime_allowed = is_ime_role(focused_node.role());
                                app.platform
                                    .focused_accessibility_node
                                    .set_if_modified(focused_node);

                                let area = layout_node.visible_area();
                                crate::integration::update_ime(
                                    app.window.as_ref(),
                                    ime_allowed,
                                    LogicalPosition::new(area.min_x(), area.min_y()),
                                    LogicalSize::new(area.width(), area.height()),
                                );

                                #[cfg(feature = "accessibility")]
                                {
                                    app.screen_reader.set(true);
                                    app.accessibility_adapter.update_if_active(|| update);
                                }
                            }
                            AccessibilityTask::None => {}
                        }

                        self.plugins.send(
                            PluginEvent::AfterAccessibility {
                                window: app.window.as_ref(),
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );

                        app.ticker_sender.notify();

                        self.plugins.send(
                            PluginEvent::AfterRedraw {
                                window: app.window.as_ref(),
                                font_collection: &self.font_collection,
                                tree: &app.tree,
                            },
                            PluginHandle::new(&self.proxy),
                        );
                    });
                }
                WindowEvent::SurfaceResized(size) => {
                    if let Err(error) = app.driver.resize(size) {
                        tracing::warn!(
                            "Graphics driver lost while resizing ({error:?}), rebuilding on the same window"
                        );
                        needs_recovery = true;
                    }

                    app.window.request_redraw();

                    app.process_layout_on_next_render = true;
                    app.tree.layout.clear_dirty();
                    app.tree.layout.invalidate(NodeId::ROOT);
                }

                WindowEvent::PointerButton {
                    state,
                    position,
                    button: ButtonSource::Touch { finger_id, force },
                    ..
                } => {
                    let finger_id = finger_id.into_raw() as u64;
                    app.position = CursorPoint::from((position.x, position.y));
                    let phase = if state.is_pressed() {
                        app.active_touches.insert(finger_id, app.position);
                        TouchPhase::Started
                    } else {
                        app.active_touches.remove(&finger_id);
                        TouchPhase::Ended
                    };
                    app.queue_platform_events(vec![PlatformEvent::Touch {
                        name: if state.is_pressed() {
                            TouchEventName::TouchStart
                        } else {
                            TouchEventName::TouchEnd
                        },
                        location: app.position,
                        finger_id,
                        phase: map_winit_touch_phase(phase),
                        force: force.map(map_winit_touch_force),
                    }]);
                }
                WindowEvent::PointerButton {
                    state,
                    button,
                    position,
                    ..
                } => {
                    let Some(button) = button.mouse_button() else {
                        return;
                    };
                    app.position = CursorPoint::from((position.x, position.y));
                    app.mouse_state = state;
                    app.platform
                        .navigation_mode
                        .set(NavigationMode::NotKeyboard);

                    let name = if state == ElementState::Pressed {
                        MouseEventName::MouseDown
                    } else {
                        MouseEventName::MouseUp
                    };
                    app.queue_platform_events(vec![PlatformEvent::Mouse {
                        name,
                        cursor: (app.position.x, app.position.y).into(),
                        button: Some(map_winit_mouse_button(button)),
                    }]);
                }

                WindowEvent::KeyboardInput {
                    event,
                    is_synthetic,
                    ..
                } => {
                    // Ignore synthetic presses (e.g. Tab on alt-tab) but keep synthetic releases so keys don't get stuck.
                    if is_synthetic && event.state == ElementState::Pressed {
                        return;
                    }

                    let name = match event.state {
                        ElementState::Pressed => KeyboardEventName::KeyDown,
                        ElementState::Released => KeyboardEventName::KeyUp,
                    };
                    let key = winit_mappings::map_winit_key(&event.logical_key);
                    let code = event.physical_key.into();
                    let modifiers = winit_mappings::map_winit_modifiers(app.modifiers_state);

                    app.pressed_keys
                        .retain(|(_, pressed_code)| *pressed_code != code);
                    if event.state.is_pressed() {
                        app.pressed_keys.push((key.clone(), code));
                    }

                    self.plugins.send(
                        PluginEvent::KeyboardInput {
                            window: app.window.as_ref(),
                            key: key.clone(),
                            code,
                            modifiers,
                            is_pressed: event.state.is_pressed(),
                        },
                        PluginHandle::new(&self.proxy),
                    );

                    app.queue_platform_events(vec![PlatformEvent::Keyboard {
                        name,
                        key,
                        code,
                        modifiers,
                    }]);
                }

                WindowEvent::MouseWheel { delta, phase, .. } => {
                    const WHEEL_SPEED_MODIFIER: f64 = 53.0;
                    const TOUCHPAD_SPEED_MODIFIER: f64 = 2.0;

                    let (delta_x, delta_y, source) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (
                            x as f64 * WHEEL_SPEED_MODIFIER,
                            y as f64 * WHEEL_SPEED_MODIFIER,
                            WheelSource::Line,
                        ),
                        MouseScrollDelta::PixelDelta(position) => (
                            position.x * TOUCHPAD_SPEED_MODIFIER,
                            position.y * TOUCHPAD_SPEED_MODIFIER,
                            WheelSource::Pixel,
                        ),
                        _ => return,
                    };

                    app.queue_platform_events(vec![PlatformEvent::Wheel {
                        name: WheelEventName::Wheel,
                        scroll: (delta_x, delta_y).into(),
                        cursor: app.position,
                        source,
                        phase: map_winit_touch_phase(phase),
                    }]);
                }

                WindowEvent::PointerLeft {
                    kind: PointerKind::Touch(finger_id),
                    ..
                } => {
                    let finger_id = finger_id.into_raw() as u64;
                    if let Some(position) = app.active_touches.remove(&finger_id) {
                        app.queue_platform_events(vec![PlatformEvent::Touch {
                            name: TouchEventName::TouchCancel,
                            location: position,
                            finger_id,
                            phase: map_winit_touch_phase(TouchPhase::Cancelled),
                            force: None,
                        }]);
                    }
                }
                WindowEvent::PointerLeft { .. } => {
                    if std::mem::replace(&mut app.mouse_state, ElementState::Released)
                        == ElementState::Released
                    {
                        app.cursor_in_window = false;
                        app.queue_platform_events(vec![PlatformEvent::PointerExit {
                            cursor: app.position,
                        }]);
                    }
                }
                WindowEvent::PointerMoved {
                    position,
                    source: PointerSource::Touch { finger_id, force },
                    ..
                } => {
                    let finger_id = finger_id.into_raw() as u64;
                    app.position = CursorPoint::from((position.x, position.y));
                    app.active_touches.insert(finger_id, app.position);
                    app.queue_platform_events(vec![PlatformEvent::Touch {
                        name: TouchEventName::TouchMove,
                        location: app.position,
                        finger_id,
                        phase: map_winit_touch_phase(TouchPhase::Moved),
                        force: force.map(map_winit_touch_force),
                    }]);
                }
                WindowEvent::PointerMoved { position, .. } => {
                    app.position = CursorPoint::from((position.x, position.y));
                    app.cursor_in_window = true;

                    app.queue_platform_events(vec![PlatformEvent::Mouse {
                        name: MouseEventName::MouseMove,
                        cursor: app.position,
                        button: None,
                    }]);
                }

                WindowEvent::Ime(Ime::Commit(text)) => {
                    app.queue_platform_events(vec![PlatformEvent::Keyboard {
                        name: KeyboardEventName::KeyDown,
                        key: keyboard_types::Key::Character(text),
                        code: keyboard_types::Code::Unidentified,
                        modifiers: winit_mappings::map_winit_modifiers(app.modifiers_state),
                    }]);
                }
                WindowEvent::Ime(Ime::Preedit(text, pos)) => {
                    app.queue_platform_events(vec![PlatformEvent::ImePreedit {
                        name: ImeEventName::Preedit,
                        text,
                        cursor: pos,
                    }]);
                }
                _ => {}
            }
        }

        // Rebuild on the same window.
        if needs_recovery && let Some(mut app) = self.windows.remove(&window_id) {
            // Drop the lost driver first to release its GPU surface.
            drop(app.driver);
            app.driver = GraphicsDriver::recover_reusing_window(
                event_loop,
                app.window.as_ref(),
                self.gpu_resource_cache_limit,
                app.window_attributes.transparent(),
            );
            tracing::info!("Recovered onto the {} driver", app.driver.name());
            self.plugins.send(
                PluginEvent::GraphicsDriverChanged {
                    window: app.window.as_ref(),
                    graphics_driver: app.driver.name(),
                    gpu_name: app.driver.gpu_name(),
                },
                PluginHandle::new(&self.proxy),
            );
            app.window.request_redraw();
            self.windows.insert(window_id, app);
        }
    }
}

fn subscribe_preferences(proxy: NativeEventProxy) {
    let subscription = mundy::Preferences::subscribe(mundy::Interest::AccentColor, move |prefs| {
        let _ = proxy.send_event(NativeEvent::Preferences(prefs));
    });
    std::mem::forget(subscription);
}
