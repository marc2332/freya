use std::{
    borrow::Cow,
    path::PathBuf,
    rc::Rc,
    sync::Arc,
    task::Waker,
};

use accesskit_winit::Adapter;
use freya_clipboard::copypasta::{
    ClipboardContext,
    ClipboardProvider,
};
use freya_components::{
    cache::AssetCacher,
    integration::integration,
};
use freya_core::{
    integration::*,
    prelude::{
        Color,
        CursorIcon,
    },
};
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use futures_util::task::{
    ArcWake,
    waker,
};
use keyboard_types::{
    Code,
    Key,
};
use ragnarok::{
    EventsMeasurerRunner,
    NodesState,
};
use raw_window_handle::HasDisplayHandle;
#[cfg(target_os = "linux")]
use raw_window_handle::RawDisplayHandle;
use torin::prelude::{
    CursorPoint,
    Size2D,
};
use winit::{
    dpi::{
        LogicalPosition,
        LogicalSize,
    },
    event::ElementState,
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    keyboard::ModifiersState,
    window::{
        Theme,
        Window,
        WindowAttributes,
        WindowId,
    },
};

use crate::{
    accessibility::AccessibilityTask,
    config::{
        OnCloseHook,
        WindowConfig,
    },
    drivers::GraphicsDriver,
    integration::is_ime_role,
    plugins::{
        PluginEvent,
        PluginHandle,
        PluginsManager,
    },
    renderer::{
        NativeEvent,
        NativeWindowEvent,
        NativeWindowEventAction,
    },
};

pub struct AppWindow {
    pub(crate) runner: Runner,
    pub(crate) tree: Tree,
    pub(crate) driver: GraphicsDriver,
    pub(crate) window: Window,
    pub(crate) nodes_state: NodesState<NodeId>,

    pub(crate) position: CursorPoint,
    pub(crate) mouse_state: ElementState,
    pub(crate) modifiers_state: ModifiersState,
    pub(crate) cursor_icon: CursorIcon,
    pub(crate) pressed_keys: Vec<(Key, Code)>,

    pub(crate) events_receiver: futures_channel::mpsc::UnboundedReceiver<EventsChunk>,
    pub(crate) events_sender: futures_channel::mpsc::UnboundedSender<EventsChunk>,

    pub(crate) accessibility: AccessibilityTree,
    pub(crate) accessibility_adapter: accesskit_winit::Adapter,
    pub(crate) accessibility_tasks_for_next_render: AccessibilityTask,
    pub(crate) screen_reader: ScreenReader,

    pub(crate) process_layout_on_next_render: bool,
    pub(crate) send_mouse_move_on_next_layout: bool,

    pub(crate) waker: Waker,

    pub(crate) ticker_sender: RenderingTickerSender,

    pub(crate) platform: Platform,

    pub(crate) animation_clock: AnimationClock,

    pub(crate) background: Color,

    pub(crate) dropped_file_paths: Vec<PathBuf>,

    pub(crate) on_close: Option<OnCloseHook>,

    pub(crate) window_attributes: WindowAttributes,

    #[cfg(feature = "hotreload")]
    pub(crate) hot_reload_pending: Arc<std::sync::atomic::AtomicBool>,
}

const MIN_CUSTOM_SCALE_FACTOR: f64 = 0.25;
const MAX_CUSTOM_SCALE_FACTOR: f64 = 5.0;

fn clamp_custom_scale_factor(custom_scale_factor: f64) -> f64 {
    custom_scale_factor.clamp(MIN_CUSTOM_SCALE_FACTOR, MAX_CUSTOM_SCALE_FACTOR)
}

impl AppWindow {
    pub(crate) fn process_accessibility_update(&mut self, mode: Option<NavigationMode>) {
        let title = self.window.title();
        let update =
            self.accessibility
                .process_updates(&mut self.tree, &self.events_sender, &title);
        self.platform
            .focused_accessibility_id
            .set_if_modified(update.focus);
        let node_id = self.accessibility.focused_node_id().unwrap();
        let layout_node = self.tree.layout.get(&node_id).unwrap();
        let focused_node = AccessibilityTree::create_node(node_id, layout_node, &self.tree, &title);
        self.window
            .set_ime_allowed(is_ime_role(focused_node.role()));
        self.platform
            .focused_accessibility_node
            .set_if_modified(focused_node);
        if let Some(mode) = mode {
            self.platform.navigation_mode.set(mode);
        }

        let area = layout_node.visible_area();
        self.window.set_ime_cursor_area(
            LogicalPosition::new(area.min_x(), area.min_y()),
            LogicalSize::new(area.width(), area.height()),
        );

        if self.screen_reader.is_on() {
            self.accessibility_adapter.update_if_active(|| update);
        }
    }

    /// Set the window title and refresh the accessibility label of the root node.
    pub fn set_title(&mut self, title: &str) {
        if self.window.title() == title {
            return;
        }
        self.window.set_title(title);
        self.tree.accessibility_diff.add_or_update(NodeId::ROOT);
        self.accessibility_tasks_for_next_render |= AccessibilityTask::ProcessUpdate { mode: None };
        self.window.request_redraw();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mut window_config: WindowConfig,
        active_event_loop: &ActiveEventLoop,
        event_loop_proxy: &EventLoopProxy<NativeEvent>,
        plugins: &mut PluginsManager,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        fallback_fonts: &[Cow<'static, str>],
        gpu_resource_cache_limit: usize,
        global_contexts: &GlobalContexts,
    ) -> Self {
        #[cfg(feature = "hotreload")]
        let hot_reload_pending = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut window_attributes = Window::default_attributes()
            .with_resizable(window_config.resizable)
            .with_window_icon(window_config.icon.take())
            .with_visible(false)
            .with_title(window_config.title)
            .with_decorations(window_config.decorations)
            .with_transparent(window_config.transparent)
            .with_inner_size(LogicalSize::<f64>::from(window_config.size));

        if let Some(min_size) = window_config.min_size {
            window_attributes =
                window_attributes.with_min_inner_size(LogicalSize::<f64>::from(min_size));
        }
        if let Some(max_size) = window_config.max_size {
            window_attributes =
                window_attributes.with_max_inner_size(LogicalSize::<f64>::from(max_size));
        }
        #[cfg(target_os = "linux")]
        if let Some(app_id) = window_config.app_id.take() {
            use winit::platform::wayland::WindowAttributesExtWayland;
            window_attributes = window_attributes.with_name(&app_id, &app_id);
        }
        if let Some(window_attributes_hook) = window_config.window_attributes_hook.take() {
            window_attributes = window_attributes_hook(window_attributes, active_event_loop);
        }
        let (driver, mut window) = GraphicsDriver::new(
            active_event_loop,
            window_attributes.clone(),
            gpu_resource_cache_limit,
        );

        if let Some(window_handle_hook) = window_config.window_handle_hook.take() {
            window_handle_hook(&mut window);
        }

        let on_close = window_config.on_close.take();

        let (events_sender, events_receiver) = futures_channel::mpsc::unbounded();

        let app = window_config.app.clone();
        let mut runner = Runner::new({
            let plugins = plugins.clone();
            move || {
                let el = integration(app.clone()).into_element();
                plugins.wrap_root(el)
            }
        });

        runner.provide_root_context(|| global_contexts.clone());

        let screen_reader = ScreenReader::new();
        runner.provide_root_context(|| screen_reader.clone());

        let (ticker_sender, ticker) = RenderingTicker::new();
        runner.provide_root_context(|| ticker);

        let animation_clock = AnimationClock::new();
        runner.provide_root_context(|| animation_clock.clone());

        runner.provide_root_context(AssetCacher::create);
        let mut tree = Tree::default();

        let custom_scale_factor = clamp_custom_scale_factor(window_config.custom_scale_factor);
        let scale_factor = window.scale_factor() * custom_scale_factor;

        let window_size = window.inner_size();
        let accent_color_preference = accent_color_preference();
        let platform = runner.provide_root_context({
            let event_loop_proxy = event_loop_proxy.clone();
            let window_id = window.id();
            let theme = match window.theme() {
                Some(Theme::Dark) => PreferredTheme::Dark,
                _ => PreferredTheme::Light,
            };
            let is_app_focused = window.has_focus();
            move || Platform {
                focused_accessibility_id: State::create(ACCESSIBILITY_ROOT_ID),
                focused_accessibility_node: State::create(accesskit::Node::new(
                    accesskit::Role::Window,
                )),
                root_size: State::create(Size2D::new(
                    window_size.width as f32,
                    window_size.height as f32,
                )),
                scale_factor: State::create(scale_factor),
                custom_scale_factor: State::create(custom_scale_factor),
                navigation_mode: State::create(NavigationMode::NotKeyboard),
                preferred_theme: State::create(theme),
                is_app_focused: State::create(is_app_focused),
                accent_color: State::create(accent_color_preference.accent_color),
                sender: Rc::new(move |user_event| {
                    let _ = event_loop_proxy.send_event(NativeEvent::Window(NativeWindowEvent {
                        window_id,
                        action: NativeWindowEventAction::User(user_event),
                    }));
                }),
            }
        });

        let clipboard = {
            if let Ok(handle) = window.display_handle() {
                #[allow(clippy::match_single_binding)]
                match handle.as_raw() {
                    #[cfg(target_os = "linux")]
                    RawDisplayHandle::Wayland(handle) => {
                        let (_primary, clipboard) = unsafe {
                            use freya_clipboard::copypasta::wayland_clipboard;

                            wayland_clipboard::create_clipboards_from_external(
                                handle.display.as_ptr(),
                            )
                        };
                        let clipboard: Box<dyn ClipboardProvider> = Box::new(clipboard);
                        Some(clipboard)
                    }
                    _ => ClipboardContext::new().ok().map(|c| {
                        let clipboard: Box<dyn ClipboardProvider> = Box::new(c);
                        clipboard
                    }),
                }
            } else {
                None
            }
        };

        runner.provide_root_context(|| State::create(clipboard));

        runner.provide_root_context(|| tree.accessibility_generator.clone());

        runner.provide_root_context(|| tree.accessibility_generator.clone());

        runner.provide_root_context(|| font_collection.clone());

        plugins.send(
            PluginEvent::RunnerCreated {
                runner: &mut runner,
            },
            PluginHandle::new(event_loop_proxy),
        );

        let mutations = runner.sync_and_update();
        let result = tree.apply_mutations(mutations);
        if let Some(strategy) = result.auto_focus {
            tree.accessibility_diff.request_focus(strategy);
        }
        tree.measure_layout(
            (
                window.inner_size().width as f32,
                window.inner_size().height as f32,
            )
                .into(),
            font_collection,
            font_manager,
            &events_sender,
            scale_factor,
            fallback_fonts,
        );

        let nodes_state = NodesState::default();

        let accessibility_adapter =
            Adapter::with_event_loop_proxy(active_event_loop, &window, event_loop_proxy.clone());

        window.set_visible(true);

        struct TreeHandle(EventLoopProxy<NativeEvent>, WindowId);

        impl ArcWake for TreeHandle {
            fn wake_by_ref(arc_self: &Arc<Self>) {
                _ = arc_self
                    .0
                    .send_event(NativeEvent::Window(NativeWindowEvent {
                        window_id: arc_self.1,
                        action: NativeWindowEventAction::PollRunner,
                    }));
            }
        }

        let waker = waker(Arc::new(TreeHandle(event_loop_proxy.clone(), window.id())));

        #[cfg(feature = "hotreload")]
        {
            let event_loop_proxy = event_loop_proxy.clone();
            let window_id = window.id();
            let hot_reload_pending_handler = hot_reload_pending.clone();
            freya_core::hotreload::subsecond::register_handler(Arc::new(move || {
                hot_reload_pending_handler.store(true, std::sync::atomic::Ordering::Release);
                let _ = event_loop_proxy.send_event(NativeEvent::Window(NativeWindowEvent {
                    window_id,
                    action: NativeWindowEventAction::PollRunner,
                }));
            }));
        }

        plugins.send(
            PluginEvent::WindowCreated {
                window: &window,
                font_collection,
                tree: &tree,
                animation_clock: &animation_clock,
                runner: &mut runner,
                graphics_driver: driver.name(),
                gpu_name: driver.gpu_name(),
            },
            PluginHandle::new(event_loop_proxy),
        );

        AppWindow {
            runner,
            tree,
            driver,
            window,
            nodes_state,

            mouse_state: ElementState::Released,
            position: CursorPoint::default(),
            modifiers_state: ModifiersState::default(),
            cursor_icon: CursorIcon::default(),
            pressed_keys: Vec::new(),

            events_receiver,
            events_sender,

            accessibility: AccessibilityTree::default(),
            accessibility_adapter,
            accessibility_tasks_for_next_render: AccessibilityTask::ProcessUpdate { mode: None },
            screen_reader,

            process_layout_on_next_render: true,
            send_mouse_move_on_next_layout: false,

            waker,

            ticker_sender,

            platform,

            animation_clock,

            background: window_config.background,

            dropped_file_paths: Vec::new(),

            on_close,

            window_attributes,

            #[cfg(feature = "hotreload")]
            hot_reload_pending,
        }
    }

    /// Resolve the cursor icon from the hovered nodes and update the window cursor if it changed.
    pub(crate) fn update_cursor_icon(&mut self) {
        if self.mouse_state == ElementState::Pressed
            || self.position == CursorPoint::from((-1., -1.))
        {
            return;
        }
        let cursor_icon = self.tree.cursor_icon(&self.nodes_state);
        if cursor_icon != self.cursor_icon {
            self.cursor_icon = cursor_icon;
            self.window.set_cursor(cursor_icon);
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn effective_scale_factor(&self) -> f64 {
        self.window.scale_factor() * *self.platform.custom_scale_factor.peek()
    }

    /// Syncs the effective scale factor on [`Platform`] and invalidates layout.
    pub fn scale_factor_changed(&mut self) {
        self.platform
            .scale_factor
            .set(self.effective_scale_factor());
        self.process_layout_on_next_render = true;
        self.tree.layout.reset();
        self.tree.text_cache.reset();
        self.window.request_redraw();
    }

    /// Measures the given platform events and emits the results.
    /// Wheel events schedule a mouse move to refresh hover states.
    pub(crate) fn process_platform_events(&mut self, mut platform_events: Vec<PlatformEvent>) {
        if platform_events
            .iter()
            .any(|platform_event| matches!(platform_event, PlatformEvent::Wheel { .. }))
        {
            self.send_mouse_move_on_next_layout = true;
        }

        let mut events_measurer_adapter = EventsMeasurerAdapter {
            scale_factor: self.effective_scale_factor(),
            tree: &mut self.tree,
        };
        let processed_events = events_measurer_adapter.run(
            &mut platform_events,
            &mut self.nodes_state,
            self.accessibility.focused_node_id(),
        );
        self.events_sender
            .unbounded_send(EventsChunk::Processed(processed_events))
            .unwrap();
    }

    /// Sets the custom scale factor, clamped to a reasonable range.
    pub fn set_custom_scale_factor(&mut self, custom_scale_factor: f64) {
        let clamped = clamp_custom_scale_factor(custom_scale_factor);
        if (clamped - *self.platform.custom_scale_factor.peek()).abs() < f64::EPSILON {
            return;
        }
        self.platform.custom_scale_factor.set(clamped);
        self.scale_factor_changed();
    }
}

fn accent_color_preference() -> mundy::Preferences {
    use std::sync::OnceLock;
    static PREFERENCE: OnceLock<mundy::Preferences> = OnceLock::new();
    *PREFERENCE.get_or_init(|| {
        mundy::Preferences::once_blocking(
            mundy::Interest::AccentColor,
            std::time::Duration::from_millis(200),
        )
        .unwrap_or_default()
    })
}
