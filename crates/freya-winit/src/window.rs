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
    prelude::Color,
};
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use futures_util::task::{
    ArcWake,
    waker,
};
use ragnarok::NodesState;
use raw_window_handle::HasDisplayHandle;
#[cfg(target_os = "linux")]
use raw_window_handle::RawDisplayHandle;
use torin::prelude::{
    CursorPoint,
    Size2D,
};
use winit::{
    dpi::LogicalSize,
    event::ElementState,
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    keyboard::ModifiersState,
    window::{
        Theme,
        Window,
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
    pub(crate) just_focused: bool,

    pub(crate) events_receiver: futures_channel::mpsc::UnboundedReceiver<EventsChunk>,
    pub(crate) events_sender: futures_channel::mpsc::UnboundedSender<EventsChunk>,

    pub(crate) accessibility: AccessibilityTree,
    pub(crate) accessibility_adapter: accesskit_winit::Adapter,
    pub(crate) accessibility_tasks_for_next_render: AccessibilityTask,

    pub(crate) process_layout_on_next_render: bool,

    pub(crate) waker: Waker,

    pub(crate) ticker_sender: RenderingTickerSender,

    pub(crate) platform: Platform,

    pub(crate) animation_clock: AnimationClock,

    pub(crate) background: Color,

    pub(crate) dropped_file_paths: Vec<PathBuf>,

    pub(crate) on_close: Option<OnCloseHook>,
}

impl AppWindow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mut window_config: WindowConfig,
        active_event_loop: &ActiveEventLoop,
        event_loop_proxy: &EventLoopProxy<NativeEvent>,
        plugins: &mut PluginsManager,
        font_collection: &FontCollection,
        font_manager: &FontMgr,
        fallback_fonts: &[Cow<'static, str>],
        screen_reader: ScreenReader,
    ) -> Self {
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
        if let Some(window_attributes_hook) = window_config.window_attributes_hook.take() {
            window_attributes = window_attributes_hook(window_attributes, active_event_loop);
        }
        let (driver, mut window) =
            GraphicsDriver::new(active_event_loop, window_attributes, &window_config);

        if let Some(window_handle_hook) = window_config.window_handle_hook.take() {
            window_handle_hook(&mut window);
        }

        let on_close = window_config.on_close.take();

        let (events_sender, events_receiver) = futures_channel::mpsc::unbounded();

        let mut runner = Runner::new(move || integration(window_config.app.clone()).into_element());

        runner.provide_root_context(|| screen_reader);

        let (mut ticker_sender, ticker) = RenderingTicker::new();
        ticker_sender.set_overflow(true);
        runner.provide_root_context(|| ticker);

        let animation_clock = AnimationClock::new();
        runner.provide_root_context(|| animation_clock.clone());

        runner.provide_root_context(AssetCacher::create);
        let mut tree = Tree::default();

        let window_size = window.inner_size();
        let platform = runner.provide_root_context({
            let event_loop_proxy = event_loop_proxy.clone();
            let window_id = window.id();
            let theme = match window.theme() {
                Some(Theme::Dark) => PreferredTheme::Dark,
                _ => PreferredTheme::Light,
            };
            move || Platform {
                focused_accessibility_id: State::create(ACCESSIBILITY_ROOT_ID),
                focused_accessibility_node: State::create(accesskit::Node::new(
                    accesskit::Role::Window,
                )),
                root_size: State::create(Size2D::new(
                    window_size.width as f32,
                    window_size.height as f32,
                )),
                navigation_mode: State::create(NavigationMode::NotKeyboard),
                preferred_theme: State::create(theme),
                sender: Rc::new(move |user_event| {
                    event_loop_proxy
                        .send_event(NativeEvent::Window(NativeWindowEvent {
                            window_id,
                            action: NativeWindowEventAction::User(user_event),
                        }))
                        .unwrap();
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
        tree.apply_mutations(mutations);
        tree.measure_layout(
            (
                window.inner_size().width as f32,
                window.inner_size().height as f32,
            )
                .into(),
            font_collection,
            font_manager,
            &events_sender,
            window.scale_factor(),
            fallback_fonts,
        );

        let nodes_state = NodesState::default();

        let accessibility_adapter =
            Adapter::with_event_loop_proxy(active_event_loop, &window, event_loop_proxy.clone());

        window.set_visible(true);
        window.set_ime_allowed(true);

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

        plugins.send(
            PluginEvent::WindowCreated {
                window: &window,
                font_collection,
                tree: &tree,
                animation_clock: &animation_clock,
                runner: &mut runner,
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
            just_focused: false,

            events_receiver,
            events_sender,

            accessibility: AccessibilityTree::default(),
            accessibility_adapter,
            accessibility_tasks_for_next_render: AccessibilityTask::ProcessUpdate { mode: None },

            process_layout_on_next_render: true,

            waker,

            ticker_sender,

            platform,

            animation_clock,

            background: window_config.background,

            dropped_file_paths: Vec::new(),

            on_close,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}
