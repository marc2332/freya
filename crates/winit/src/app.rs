use std::sync::Arc;

use accesskit::{
    Node,
    Role,
};
use dioxus_core::VirtualDom;
use freya_core::{
    accessibility::{
        AccessibilityFocusStrategy,
        ACCESSIBILITY_ROOT_ID,
    },
    dom::SafeDOM,
    event_loop_messages::{
        EventLoopMessage,
        EventLoopMessageAction,
        TextGroupMeasurement,
    },
    events::{
        EventsExecutorAdapter,
        EventsMeasurerAdapter,
        PlatformEvent,
    },
    layout::process_layout,
    platform_state::{
        NativePlatformState,
        NavigationMode,
        PlatformInformation,
    },
    plugins::{
        PluginEvent,
        PluginHandle,
        PluginsManager,
    },
    render::{
        Compositor,
        RenderPipeline,
    },
    types::{
        EventEmitter,
        EventReceiver,
        EventsQueue,
        NativePlatformReceiver,
        NativePlatformSender,
    },
    values::Color,
    window_config::WindowConfig,
};
use freya_engine::prelude::*;
use freya_native_core::NodeId;
use futures_task::Waker;
use futures_util::Future;
use ragnarok::{
    EventsExecutorRunner,
    EventsMeasurerRunner,
    NodesState,
};
use tokio::{
    select,
    sync::{
        broadcast,
        mpsc,
        watch,
    },
};
use torin::geometry::Area;
use winit::{
    event_loop::EventLoopProxy,
    window::Window,
};

use crate::{
    accessibility::WinitAcessibilityTree,
    drivers::GraphicsDriver,
    size::WinitSize,
    winit_waker::winit_waker,
};

#[derive(Hash, PartialEq, Eq)]
pub enum AccessibilityTask {
    ProcessUpdate,
    ProcessWithMode(NavigationMode),
}

/// Manages the Application lifecycle
pub struct Application {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) compositor: Compositor,
    pub(crate) events: EventsQueue,
    pub(crate) vdom_waker: Waker,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) nodes_state: NodesState<NodeId>,
    pub(crate) platform_sender: NativePlatformSender,
    pub(crate) platform_receiver: NativePlatformReceiver,
    pub(crate) accessibility: WinitAcessibilityTree,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) process_layout_on_next_render: bool,
    pub(crate) accessibility_tasks_for_next_render: Option<AccessibilityTask>,
    pub(crate) init_accessibility_on_next_render: bool,

    pub(crate) surface: Surface,
    pub(crate) dirty_surface: Surface,
    pub(crate) graphics_driver: GraphicsDriver,
    pub(crate) window: Window,
    pub(crate) is_window_focused: bool,
    pub(crate) proxy: EventLoopProxy<EventLoopMessage>,
    pub(crate) plugins: PluginsManager,

    pub(crate) window_config: WindowConfig,
}

impl Application {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventLoopMessage>,
        window: Window,
        accessibility: WinitAcessibilityTree,
        surface: Surface,
        dirty_surface: Surface,
        graphics_driver: GraphicsDriver,
        window_config: WindowConfig,
        plugins: PluginsManager,
    ) -> Self {
        let (event_emitter, event_receiver) = mpsc::unbounded_channel();
        let (platform_sender, platform_receiver) = watch::channel(NativePlatformState {
            focused_accessibility_id: ACCESSIBILITY_ROOT_ID,
            focused_accessibility_node: Node::new(Role::Window),
            preferred_theme: window.theme().map(|theme| theme.into()).unwrap_or_default(),
            navigation_mode: NavigationMode::default(),
            information: PlatformInformation::from_winit(&window),
            scale_factor: window.scale_factor(),
        });

        Self {
            sdom,
            vdom,
            events: EventsQueue::new(),
            vdom_waker: winit_waker(proxy, window.id()),
            event_emitter,
            event_receiver,
            nodes_state: NodesState::default(),
            accessibility,
            platform_sender,
            platform_receiver,
            ticker_sender: broadcast::channel(5).0,
            process_layout_on_next_render: false,
            accessibility_tasks_for_next_render: None,
            init_accessibility_on_next_render: false,
            compositor: Compositor::default(),
            dirty_surface,
            surface,
            graphics_driver,
            is_window_focused: false,
            window,
            proxy: proxy.clone(),
            plugins,
            window_config,
        }
    }

    /// Sync the RealDOM with the VirtualDOM
    pub fn init_doms(&mut self, scale_factor: f32) {
        // Insert built-in VirtualDOM contexts
        self.vdom
            .insert_any_root_context(Box::new(self.proxy.clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.platform_receiver.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        self.vdom
            .insert_any_root_context(Box::new(self.sdom.get().accessibility_generator().clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.sdom.get().animation_clock().clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.window.id()));

        // Init the RealDOM
        self.sdom.get_mut().init_dom(&mut self.vdom, scale_factor);
    }

    /// Update the RealDOM, layout and others with the latest changes from the VirtualDOM
    pub fn render_mutations(&mut self, scale_factor: f32) -> (bool, bool) {
        let fdom = &mut self.sdom.get_mut();
        self.plugins.send(
            PluginEvent::StartedUpdatingDOM {
                window: &self.window,
                fdom,
            },
            PluginHandle::new(&self.proxy),
        );

        let (repaint, relayout) = fdom.render_mutations(&mut self.vdom, scale_factor);

        self.plugins.send(
            PluginEvent::FinishedUpdatingDOM {
                window: &self.window,
                fdom,
            },
            PluginHandle::new(&self.proxy),
        );

        (repaint, relayout)
    }

    /// Poll the VirtualDOM for any new change
    pub fn poll_vdom(&mut self) {
        let mut cx = std::task::Context::from_waker(&self.vdom_waker);

        {
            let fut = std::pin::pin!(async {
                select! {
                    Some(processed_events) = self.event_receiver.recv() => {
                        let fdom = self.sdom.get();
                        let rdom = fdom.rdom();
                        let events_executor_adapter = EventsExecutorAdapter {
                            rdom,
                            vdom: &mut self.vdom,
                        };
                        events_executor_adapter.run(&mut self.nodes_state,
                            processed_events);
                    },
                    _ = self.vdom.wait_for_work() => {},
                }
            });

            match fut.poll(&mut cx) {
                std::task::Poll::Ready(_) => {
                    self.proxy
                        .send_event(EventLoopMessage {
                            window_id: Some(self.window.id()),
                            action: EventLoopMessageAction::PollVDOM,
                        })
                        .ok();
                }
                std::task::Poll::Pending => return,
            }
        }

        let (must_repaint, must_relayout) =
            self.render_mutations(self.window.scale_factor() as f32);

        if must_relayout {
            self.process_layout_on_next_render = true;
            self.accessibility_tasks_for_next_render
                .replace(AccessibilityTask::ProcessUpdate);
        }

        if must_relayout || must_repaint {
            self.window.request_redraw();
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self, scale_factor: f64) {
        let fdom = self.sdom.get();

        self.plugins.send(
            PluginEvent::StartedMeasuringEvents {
                window: &self.window,
                fdom: &fdom,
            },
            PluginHandle::new(&self.proxy),
        );

        {
            let rdom = fdom.rdom();
            let layout = fdom.layout();
            let layers = fdom.layers();

            let focus_id = self.accessibility.focused_node_id();
            let mut events_measurer_adapter = EventsMeasurerAdapter {
                rdom,
                layers: &layers,
                layout: &layout,
                vdom: &mut self.vdom,
                scale_factor,
            };
            let processed_events =
                events_measurer_adapter.run(&mut self.events, &mut self.nodes_state, focus_id);
            self.event_emitter.send(processed_events).unwrap();
        }

        self.plugins.send(
            PluginEvent::FinishedMeasuringEvents {
                window: &self.window,
                fdom: &fdom,
            },
            PluginHandle::new(&self.proxy),
        );
    }

    pub fn init_accessibility(&mut self) {
        let fdom = self.sdom.get();
        let rdom = fdom.rdom();
        let layout = fdom.layout();
        let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
        self.accessibility
            .init_accessibility(rdom, &layout, &mut dirty_accessibility_tree);
    }

    pub fn process_accessibility(&mut self) {
        let fdom = self.sdom.get();
        let rdom = fdom.rdom();
        let layout = fdom.layout();
        let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
        self.accessibility.process_updates(
            rdom,
            &layout,
            &self.platform_sender,
            &self.window,
            &mut dirty_accessibility_tree,
            &self.event_emitter,
        );
    }

    /// Send an event
    pub fn send_event(&mut self, event: PlatformEvent, scale_factor: f64) {
        self.events.push(event);
        self.process_events(scale_factor);
    }

    /// Render the App into the Window Canvas
    pub fn render(
        &mut self,
        scale_factor: f32,
        font_collection: &mut FontCollection,
        font_manager: &mut FontMgr,
        fallback_fonts: &[String],
    ) {
        self.plugins.send(
            PluginEvent::BeforeRender {
                window: &self.window,
                canvas: self.surface.canvas(),
                font_collection,
                fdom: &self.sdom.get(),
            },
            PluginHandle::new(&self.proxy),
        );

        self.render_with_pipeline(
            scale_factor,
            self.window_config.background,
            font_collection,
            font_manager,
            fallback_fonts,
        );

        self.plugins.send(
            PluginEvent::AfterRender {
                window: &self.window,
                canvas: self.surface.canvas(),
                font_collection,
                fdom: &self.sdom.get(),
            },
            PluginHandle::new(&self.proxy),
        );
    }

    /// Resize the Window
    pub fn resize(&mut self) {
        self.process_layout_on_next_render = true;
        self.accessibility_tasks_for_next_render
            .replace(AccessibilityTask::ProcessUpdate);
        self.init_accessibility_on_next_render = true;
        self.compositor.reset();
        self.sdom
            .get()
            .compositor_dirty_area()
            .unite_or_insert(&Area::new(
                (0.0, 0.0).into(),
                self.window.inner_size().to_torin(),
            ));
        self.sdom.get().layout().reset();

        self.platform_sender.send_modify(|state| {
            state.information = PlatformInformation::from_winit(&self.window);
        });

        self.window.request_redraw();
    }

    /// Measure the a text group given it's ID.
    pub fn measure_text_group(&self, text_measurement: TextGroupMeasurement, scale_factor: f64) {
        self.sdom
            .get()
            .measure_paragraphs(text_measurement, scale_factor);
    }

    pub fn request_focus_node(&mut self, focus_strategy: AccessibilityFocusStrategy) {
        let task = match focus_strategy {
            AccessibilityFocusStrategy::Backward | AccessibilityFocusStrategy::Forward => {
                AccessibilityTask::ProcessWithMode(NavigationMode::Keyboard)
            }
            _ => AccessibilityTask::ProcessUpdate,
        };

        let fdom = self.sdom.get();
        fdom.accessibility_dirty_nodes()
            .request_focus(focus_strategy);
        self.accessibility_tasks_for_next_render.replace(task);

        self.window.request_redraw();
    }

    /// Notify components subscribed to event loop ticks.
    pub fn event_loop_tick(&self) {
        self.ticker_sender.send(()).ok();
    }

    /// Update the [NavigationMode].
    pub fn set_navigation_mode(&mut self, navigation_mode: NavigationMode) {
        self.platform_sender.send_modify(|state| {
            state.navigation_mode = navigation_mode;
        })
    }

    /// Measure the layout
    pub fn process_layout(
        &mut self,
        scale_factor: f64,
        font_collection: &mut FontCollection,
        fallback_fonts: &[String],
    ) {
        let fdom = self.sdom.get();

        self.plugins.send(
            PluginEvent::StartedMeasuringLayout {
                fdom: &fdom,
                window: &self.window,
            },
            PluginHandle::new(&self.proxy),
        );

        {
            let rdom = fdom.rdom();
            let mut layout = fdom.layout();
            let mut images_cache = fdom.images_cache();
            let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
            let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();
            let mut compositor_dirty_area = fdom.compositor_dirty_area();

            process_layout(
                rdom,
                &mut layout,
                &mut images_cache,
                &mut dirty_accessibility_tree,
                &mut compositor_dirty_nodes,
                &mut compositor_dirty_area,
                Area::from_size(self.window.inner_size().to_torin()),
                font_collection,
                scale_factor as f32,
                fallback_fonts,
            );
        }

        self.plugins.send(
            PluginEvent::FinishedMeasuringLayout {
                fdom: &fdom,
                window: &self.window,
            },
            PluginHandle::new(&self.proxy),
        );

        #[cfg(debug_assertions)]
        {
            tracing::info!(
                "Processed {} layers and {} group of paragraph elements",
                fdom.layers().len(),
                fdom.paragraphs().len()
            );
        }
    }

    /// Render the DOM using the [RenderPipeline].
    pub fn render_with_pipeline(
        &mut self,
        scale_factor: f32,
        background: Color,
        font_collection: &mut FontCollection,
        font_manager: &mut FontMgr,
        fallback_fonts: &[String],
    ) {
        let fdom = self.sdom.get();

        let mut render_pipeline = RenderPipeline {
            canvas_area: Area::from_size(self.window.inner_size().to_torin()),
            rdom: fdom.rdom(),
            compositor_dirty_area: &mut fdom.compositor_dirty_area(),
            compositor_dirty_nodes: &mut fdom.compositor_dirty_nodes(),
            compositor_cache: &mut fdom.compositor_cache(),
            layers: &mut fdom.layers(),
            layout: &mut fdom.layout(),
            background,
            surface: &mut self.surface,
            dirty_surface: &mut self.dirty_surface,
            compositor: &mut self.compositor,
            scale_factor,
            font_collection,
            font_manager,
            fallback_fonts,
            images_cache: &mut fdom.images_cache(),
        };
        render_pipeline.run();
    }
}
