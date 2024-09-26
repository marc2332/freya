use std::sync::Arc;

use dioxus_core::{
    Template,
    VirtualDom,
};
use freya_core::prelude::*;
use freya_engine::prelude::*;
use freya_native_core::prelude::NodeImmutableDioxusExt;
use futures_task::Waker;
use futures_util::Future;
use pin_utils::pin_mut;
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
    accessibility::AccessKitManager,
    devtools::Devtools,
    size::WinitSize,
    winit_waker::winit_waker,
    EmbeddedFonts,
    HoveredNode,
};

/// Manages the Application lifecycle
pub struct Application {
    pub(crate) sdom: SafeDOM,
    pub(crate) vdom: VirtualDom,
    pub(crate) compositor: Compositor,
    pub(crate) events: EventsQueue,
    pub(crate) vdom_waker: Waker,
    pub(crate) proxy: EventLoopProxy<EventMessage>,
    pub(crate) devtools: Option<Devtools>,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) nodes_state: NodesState,
    pub(crate) platform_sender: NativePlatformSender,
    pub(crate) platform_receiver: NativePlatformReceiver,
    pub(crate) accessibility: AccessKitManager,
    pub(crate) font_collection: FontCollection,
    pub(crate) font_mgr: FontMgr,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) plugins: PluginsManager,
    pub(crate) measure_layout_on_next_render: bool,
    pub(crate) init_accessibility_on_next_render: bool,
    pub(crate) default_fonts: Vec<String>,
}

impl Application {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventMessage>,
        devtools: Option<Devtools>,
        window: &Window,
        fonts_config: EmbeddedFonts,
        plugins: PluginsManager,
        default_fonts: Vec<String>,
    ) -> Self {
        let accessibility = AccessKitManager::new(window, proxy.clone());

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
            focused_id: ACCESSIBILITY_ROOT_ID,
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
            measure_layout_on_next_render: false,
            init_accessibility_on_next_render: false,
            default_fonts,
            compositor: Compositor::default(),
        };

        app.plugins.send(
            PluginEvent::WindowCreated(window),
            PluginHandle::new(&app.proxy),
        );

        app
    }

    /// Provide the launch state and few other utilities like the EventLoopProxy
    pub fn provide_vdom_contexts<State: 'static>(&mut self, app_state: Option<State>) {
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
    }

    /// Make the first build of the VirtualDOM and sync it with the RealDOM.
    pub fn init_doms<State: 'static>(&mut self, scale_factor: f32, app_state: Option<State>) {
        self.plugins.send(
            PluginEvent::StartedUpdatingDOM,
            PluginHandle::new(&self.proxy),
        );

        self.provide_vdom_contexts(app_state);

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

        if repaint {
            if let Some(devtools) = &self.devtools {
                devtools.update(&self.sdom.get());
            }
        }

        (repaint, relayout)
    }

    /// Poll the VirtualDOM for any new change
    pub fn poll_vdom(&mut self, window: &Window) {
        let mut cx = std::task::Context::from_waker(&self.vdom_waker);

        {
            let fut = async {
                select! {
                    Some(events) = self.event_receiver.recv() => {
                        let fdom = self.sdom.get();
                        let rdom = fdom.rdom();
                        for event in events {
                            if let Some(element_id) = rdom
                                .get(event.node_id)
                                .and_then(|node| node.mounted_id())
                            {
                                let name = event.name.into();
                                let data = event.data.any();
                                self.vdom
                                    .handle_event(name, data, element_id, event.bubbles);
                                self.vdom.process_events();
                            }
                        }
                    },
                    _ = self.vdom.wait_for_work() => {},
                }
            };
            pin_mut!(fut);

            match fut.poll(&mut cx) {
                std::task::Poll::Ready(_) => {
                    self.proxy.send_event(EventMessage::PollVDOM).ok();
                }
                std::task::Poll::Pending => return,
            }
        }

        let (must_repaint, must_relayout) = self.render_mutations(window.scale_factor() as f32);

        if must_relayout {
            self.measure_layout_on_next_render = true;
        }

        if must_relayout || must_repaint {
            window.request_redraw();
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self, scale_factor: f64) {
        let focus_id = self.accessibility.focused_node_id();
        process_events(
            &self.sdom.get(),
            &mut self.events,
            &self.event_emitter,
            &mut self.nodes_state,
            scale_factor,
            focus_id,
        )
    }

    pub fn init_accessibility(&mut self) {
        {
            let fdom = self.sdom.get();
            let rdom = fdom.rdom();
            let layout = fdom.layout();
            let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
            self.accessibility
                .init_accessibility(rdom, &layout, &mut dirty_accessibility_tree);
        }
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
        );
    }

    /// Send an event
    pub fn send_event(&mut self, event: PlatformEvent, scale_factor: f64) {
        self.events.push(event);
        self.process_events(scale_factor);
    }

    /// Replace a VirtualDOM Template
    pub fn vdom_replace_template(&mut self, template: Template) {
        self.vdom.replace_template(template);
    }

    /// Render the App into the Window Canvas
    pub fn render(
        &mut self,
        hovered_node: &HoveredNode,
        background: Color,
        surface: &mut Surface,
        dirty_surface: &mut Surface,
        window: &Window,
    ) {
        self.plugins.send(
            PluginEvent::BeforeRender {
                canvas: surface.canvas(),
                font_collection: &self.font_collection,
                freya_dom: &self.sdom.get(),
            },
            PluginHandle::new(&self.proxy),
        );

        self.start_render(
            hovered_node,
            background,
            surface,
            dirty_surface,
            window.inner_size(),
            window.scale_factor() as f32,
        );

        self.plugins.send(
            PluginEvent::AfterRender {
                canvas: surface.canvas(),
                font_collection: &self.font_collection,
                freya_dom: &self.sdom.get(),
            },
            PluginHandle::new(&self.proxy),
        );
    }

    /// Resize the Window
    pub fn resize(&mut self, window: &Window) {
        self.measure_layout_on_next_render = true;
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

    pub fn focus_node(&mut self, node_id: AccessibilityId, window: &Window) {
        let fdom = self.sdom.get();
        let layout = fdom.layout();
        self.accessibility
            .focus_node(node_id, &self.platform_sender, window, &layout)
    }

    pub fn focus_next_node(&mut self, direction: AccessibilityFocusStrategy, window: &Window) {
        let fdom = self.sdom.get();
        let rdom = fdom.rdom();
        let layout = fdom.layout();
        self.accessibility
            .focus_next_node(rdom, direction, &self.platform_sender, window, &layout);
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
        {
            let fdom = self.sdom.get();

            self.plugins.send(
                PluginEvent::StartedLayout(&fdom.layout()),
                PluginHandle::new(&self.proxy),
            );

            process_layout(
                &fdom,
                Area::from_size(window_size.to_torin()),
                &mut self.font_collection,
                scale_factor as f32,
                &self.default_fonts,
            );

            self.plugins.send(
                PluginEvent::FinishedLayout(&fdom.layout()),
                PluginHandle::new(&self.proxy),
            );
        }

        if let Some(devtools) = &self.devtools {
            devtools.update(&self.sdom.get())
        }

        #[cfg(debug_assertions)]
        {
            let fdom = self.sdom.get();
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
        hovered_node: &HoveredNode,
        background: Color,
        surface: &mut Surface,
        dirty_surface: &mut Surface,
        window_size: PhysicalSize<u32>,
        scale_factor: f32,
    ) {
        let fdom = self.sdom.get();
        let hovered_node = hovered_node
            .as_ref()
            .and_then(|hovered_node| *hovered_node.lock().unwrap());

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
            selected_node: hovered_node,
            font_collection: &mut self.font_collection,
            font_manager: &self.font_mgr,
            default_fonts: &self.default_fonts,
        };
        render_pipeline.run();
    }
}
