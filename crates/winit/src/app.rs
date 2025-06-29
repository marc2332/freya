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
    dpi::PhysicalSize,
    event_loop::EventLoopProxy,
    window::Window,
};

use crate::{
    accessibility::WinitAcessibilityTree,
    devtools::Devtools,
    drivers::GraphicsDriver,
    size::WinitSize,
    winit_waker::winit_waker,
    EmbeddedFonts,
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
    pub(crate) proxy: EventLoopProxy<EventLoopMessage>,
    pub(crate) devtools: Option<Devtools>,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) nodes_state: NodesState<NodeId>,
    pub(crate) platform_sender: NativePlatformSender,
    pub(crate) platform_receiver: NativePlatformReceiver,
    pub(crate) accessibility: WinitAcessibilityTree,
    pub(crate) font_collection: FontCollection,
    pub(crate) font_mgr: FontMgr,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) plugins: PluginsManager,
    pub(crate) process_layout_on_next_render: bool,
    pub(crate) accessibility_tasks_for_next_render: Option<AccessibilityTask>,
    pub(crate) init_accessibility_on_next_render: bool,
    pub(crate) fallback_fonts: Vec<String>,
}

impl Application {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventLoopMessage>,
        devtools: Option<Devtools>,
        window: &Window,
        fonts_config: EmbeddedFonts,
        plugins: PluginsManager,
        fallback_fonts: Vec<String>,
        accessibility: WinitAcessibilityTree,
    ) -> Self {
        let mut font_collection = FontCollection::new();
        let def_mgr = FontMgr::default();

        let mut provider = TypefaceFontProvider::new();

        for (font_name, font_data) in fonts_config {
            let ft_type = def_mgr.new_from_data(font_data, None).unwrap();
            provider.register_typeface(ft_type, Some(font_name));
        }

        let font_mgr: FontMgr = provider.into();
        font_collection.set_default_font_manager(def_mgr, None);
        font_collection.set_dynamic_font_manager(font_mgr.clone());

        let (event_emitter, event_receiver) = mpsc::unbounded_channel();
        let (platform_sender, platform_receiver) = watch::channel(NativePlatformState {
            focused_accessibility_id: ACCESSIBILITY_ROOT_ID,
            focused_accessibility_node: Node::new(Role::Window),
            preferred_theme: window.theme().map(|theme| theme.into()).unwrap_or_default(),
            navigation_mode: NavigationMode::default(),
            information: PlatformInformation::from_winit(window),
            scale_factor: window.scale_factor(),
        });

        let mut app = Self {
            sdom,
            vdom,
            events: EventsQueue::new(),
            vdom_waker: winit_waker(proxy),
            proxy: proxy.clone(),
            devtools,
            event_emitter,
            event_receiver,
            nodes_state: NodesState::default(),
            accessibility,
            platform_sender,
            platform_receiver,
            font_collection,
            font_mgr,
            ticker_sender: broadcast::channel(5).0,
            plugins,
            process_layout_on_next_render: false,
            accessibility_tasks_for_next_render: None,
            init_accessibility_on_next_render: false,
            fallback_fonts,
            compositor: Compositor::default(),
        };

        app.plugins.send(
            PluginEvent::WindowCreated(window),
            PluginHandle::new(&app.proxy),
        );

        app
    }

    /// Sync the RealDOM with the VirtualDOM
    pub fn init_doms<State: 'static>(&mut self, scale_factor: f32, app_state: Option<State>) {
        self.plugins.send(
            PluginEvent::StartedUpdatingDOM,
            PluginHandle::new(&self.proxy),
        );

        // Insert built-in VirtualDOM contexts
        if let Some(state) = app_state {
            self.vdom.insert_any_root_context(Box::new(state));
        }
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

        // Init the RealDOM
        self.sdom.get_mut().init_dom(&mut self.vdom, scale_factor);

        self.plugins.send(
            PluginEvent::FinishedUpdatingDOM,
            PluginHandle::new(&self.proxy),
        );
    }

    /// Update the RealDOM, layout and others with the latest changes from the VirtualDOM
    pub fn render_mutations(&mut self, scale_factor: f32) -> (bool, bool) {
        self.plugins.send(
            PluginEvent::StartedUpdatingDOM,
            PluginHandle::new(&self.proxy),
        );

        let (repaint, relayout) = self
            .sdom
            .get_mut()
            .render_mutations(&mut self.vdom, scale_factor);

        self.plugins.send(
            PluginEvent::FinishedUpdatingDOM,
            PluginHandle::new(&self.proxy),
        );

        (repaint, relayout)
    }

    /// Poll the VirtualDOM for any new change
    pub fn poll_vdom(&mut self, window: &Window) {
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
                    self.proxy.send_event(EventLoopMessage::PollVDOM).ok();
                }
                std::task::Poll::Pending => return,
            }
        }

        let (must_repaint, must_relayout) = self.render_mutations(window.scale_factor() as f32);

        if must_relayout {
            self.process_layout_on_next_render = true;
            self.accessibility_tasks_for_next_render
                .replace(AccessibilityTask::ProcessUpdate);
        } else if must_repaint {
            // If there was no relayout but there was a repaint then we can update the devtools now,
            // otherwise if there was a relayout the devtools will get updated on next render
            if let Some(devtools) = &self.devtools {
                devtools.update(&self.sdom.get());
            }
        }

        if must_relayout || must_repaint {
            window.request_redraw();
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self, scale_factor: f64) {
        let sdom = self.sdom.get();
        let rdom = sdom.rdom();
        let layout = sdom.layout();
        let layers = sdom.layers();

        let focus_id = self.accessibility.focused_node_id();
        self.plugins.send(
            PluginEvent::StartedMeasuringEvents,
            PluginHandle::new(&self.proxy),
        );
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

        self.plugins.send(
            PluginEvent::FinishedMeasuringEvents,
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

    pub fn process_accessibility(&mut self, window: &Window) {
        let fdom = self.sdom.get();
        let rdom = fdom.rdom();
        let layout = fdom.layout();
        let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
        self.accessibility.process_updates(
            rdom,
            &layout,
            &self.platform_sender,
            window,
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
        background: Color,
        graphics_driver: &mut GraphicsDriver,
        window: &Window,
        scale_factor: f64,
    ) {
        graphics_driver.present(
            window.inner_size().cast(),
            window,
            |surface, dirty_surface| {
                self.plugins.send(
                    PluginEvent::BeforeRender {
                        font_collection: &self.font_collection,
                        freya_dom: &self.sdom.get(),
                    },
                    PluginHandle::new(&self.proxy),
                );
                self.start_render(
                    background,
                    surface,
                    dirty_surface,
                    window.inner_size(),
                    scale_factor as f32,
                );
                self.plugins.send(
                    PluginEvent::AfterRender {
                        canvas: surface.canvas(),
                        font_collection: &self.font_collection,
                        freya_dom: &self.sdom.get(),
                    },
                    PluginHandle::new(&self.proxy),
                );
            },
        );

        self.plugins.send(
            PluginEvent::AfterPresenting {
                font_collection: &self.font_collection,
                freya_dom: &self.sdom.get(),
            },
            PluginHandle::new(&self.proxy),
        );
    }

    /// Resize the Window
    pub fn resize(&mut self, window: &Window) {
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
                window.inner_size().to_torin(),
            ));
        self.sdom.get().layout().reset();
        self.platform_sender.send_modify(|state| {
            state.information = PlatformInformation::from_winit(window);
        })
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
    pub fn process_layout(&mut self, window_size: PhysicalSize<u32>, scale_factor: f64) {
        let fdom = self.sdom.get();
        let rdom = fdom.rdom();
        let mut layout = fdom.layout();
        let mut images_cache = fdom.images_cache();
        let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
        let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();
        let mut compositor_dirty_area = fdom.compositor_dirty_area();

        self.plugins.send(
            PluginEvent::StartedMeasuringLayout(&layout),
            PluginHandle::new(&self.proxy),
        );

        process_layout(
            rdom,
            &mut layout,
            &mut images_cache,
            &mut dirty_accessibility_tree,
            &mut compositor_dirty_nodes,
            &mut compositor_dirty_area,
            Area::from_size(window_size.to_torin()),
            &mut self.font_collection,
            scale_factor as f32,
            &self.fallback_fonts,
        );

        self.plugins.send(
            PluginEvent::FinishedMeasuringLayout(&layout),
            PluginHandle::new(&self.proxy),
        );

        if let Some(devtools) = &self.devtools {
            devtools.update(&fdom)
        }

        #[cfg(debug_assertions)]
        {
            tracing::info!(
                "Processed {} layers and {} group of paragraph elements",
                fdom.layers().len(),
                fdom.paragraphs().len()
            );
        }
    }

    /// Start rendering the RealDOM to Window
    pub fn start_render(
        &mut self,
        background: Color,
        surface: &mut Surface,
        dirty_surface: &mut Surface,
        window_size: PhysicalSize<u32>,
        scale_factor: f32,
    ) {
        let fdom = self.sdom.get();
        let highlighted_node = self
            .devtools
            .as_ref()
            .and_then(|devtools| *devtools.highlighted_node.lock().unwrap());

        let mut render_pipeline = RenderPipeline {
            canvas_area: Area::from_size(window_size.to_torin()),
            rdom: fdom.rdom(),
            compositor_dirty_area: &mut fdom.compositor_dirty_area(),
            compositor_dirty_nodes: &mut fdom.compositor_dirty_nodes(),
            compositor_cache: &mut fdom.compositor_cache(),
            layers: &mut fdom.layers(),
            layout: &mut fdom.layout(),
            background,
            surface,
            dirty_surface,
            compositor: &mut self.compositor,
            scale_factor,
            highlighted_node,
            font_collection: &mut self.font_collection,
            font_manager: &self.font_mgr,
            fallback_fonts: &self.fallback_fonts,
            images_cache: &mut fdom.images_cache(),
        };
        render_pipeline.run();
    }
}
