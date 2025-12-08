use std::{
    borrow::Cow,
    path::PathBuf,
    sync::Arc,
    task::Waker,
};

use accesskit_winit::Adapter;
use freya_components::{
    cache::AssetCacher,
    keyboard_navigator::keyboard_navigator,
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
        Window,
        WindowId,
    },
};

use crate::{
    accessibility::AccessibilityTask,
    config::WindowConfig,
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

    pub(crate) events_receiver: futures_channel::mpsc::UnboundedReceiver<EventsChunk>,
    pub(crate) events_sender: futures_channel::mpsc::UnboundedSender<EventsChunk>,

    pub(crate) accessibility: AccessibilityTree,
    pub(crate) accessibility_adapter: accesskit_winit::Adapter,
    pub(crate) accessibility_tasks_for_next_render: AccessibilityTask,

    pub(crate) process_layout_on_next_render: bool,

    pub(crate) waker: Waker,

    pub(crate) ticker_sender: RenderingTickerSender,

    pub(crate) platform_state: PlatformState,

    pub(crate) animation_clock: AnimationClock,

    pub(crate) background: Color,

    pub(crate) dropped_file_paths: Vec<PathBuf>,
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
            window_attributes = window_attributes_hook(window_attributes);
        }
        let (driver, mut window) =
            GraphicsDriver::new(active_event_loop, window_attributes, &window_config);

        if let Some(window_handle_hook) = window_config.window_handle_hook.take() {
            window_handle_hook(&mut window);
        }

        let (events_sender, events_receiver) = futures_channel::mpsc::unbounded();

        let mut runner =
            Runner::new(move || keyboard_navigator(window_config.app.clone()).into_element());

        runner.provide_root_context(|| screen_reader);

        let (mut ticker_sender, ticker) = RenderingTicker::new();
        ticker_sender.set_overflow(true);
        runner.provide_root_context(|| ticker);

        let animation_clock = AnimationClock::new();
        runner.provide_root_context(|| animation_clock.clone());

        let event_notifier = EventNotifier::new({
            let event_loop_proxy = event_loop_proxy.clone();
            let window_id = window.id();
            move |user_event| {
                event_loop_proxy
                    .send_event(NativeEvent::Window(NativeWindowEvent {
                        window_id,
                        action: NativeWindowEventAction::User(user_event),
                    }))
                    .unwrap();
            }
        });
        runner.provide_root_context(|| event_notifier);

        runner.provide_root_context(AssetCacher::create);

        let window_size = window.inner_size();
        let platform_state = runner.provide_root_context(|| PlatformState {
            focused_accessibility_id: State::create(ACCESSIBILITY_ROOT_ID),
            focused_accessibility_node: State::create(accesskit::Node::new(
                accesskit::Role::Window,
            )),
            root_size: State::create(Size2D::new(
                window_size.width as f32,
                window_size.height as f32,
            )),
            navigation_mode: State::create(NavigationMode::NotKeyboard),
        });

        let mut tree = Tree::default();

        runner.provide_root_context(|| tree.accessibility_generator.clone());

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

        struct DomHandle(EventLoopProxy<NativeEvent>, WindowId);

        impl ArcWake for DomHandle {
            fn wake_by_ref(arc_self: &Arc<Self>) {
                _ = arc_self
                    .0
                    .send_event(NativeEvent::Window(NativeWindowEvent {
                        window_id: arc_self.1,
                        action: NativeWindowEventAction::PollRunner,
                    }));
            }
        }

        let waker = waker(Arc::new(DomHandle(event_loop_proxy.clone(), window.id())));

        plugins.send(
            PluginEvent::WindowCreated {
                window: &window,
                font_collection,
                tree: &tree,
                animation_clock: &animation_clock,
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

            events_receiver,
            events_sender,

            accessibility: AccessibilityTree::default(),
            accessibility_adapter,
            accessibility_tasks_for_next_render: AccessibilityTask::ProcessUpdate { mode: None },

            process_layout_on_next_render: true,

            waker,

            ticker_sender,

            platform_state,

            animation_clock,

            background: window_config.background,

            dropped_file_paths: Vec::new(),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}
