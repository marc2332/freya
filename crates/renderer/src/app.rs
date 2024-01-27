use std::{sync::Arc, task::Waker};

use dioxus_core::{Template, VirtualDom};
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_dom::prelude::SafeDOM;
use freya_engine::prelude::*;
use futures_task::{waker, ArcWake};
use futures_util::FutureExt;
use pin_utils::pin_mut;
use tokio::sync::broadcast;
use tokio::{
    select,
    sync::{mpsc, watch, Notify},
};
use tracing::info;
use uuid::Uuid;
use winit::event::WindowEvent;
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy};

use crate::accessibility::NativeAccessibility;
use crate::{FontsConfig, HoveredNode, WindowEnv};

fn winit_waker(proxy: &EventLoopProxy<EventMessage>) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<EventMessage>);

    unsafe impl Send for DomHandle {}
    unsafe impl Sync for DomHandle {}

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(EventMessage::PollVDOM);
        }
    }

    waker(Arc::new(DomHandle(proxy.clone())))
}

/// Manages the Application lifecycle
pub struct App<State: 'static + Clone> {
    sdom: SafeDOM,
    vdom: VirtualDom,
    events: EventsQueue,
    vdom_waker: Waker,
    proxy: EventLoopProxy<EventMessage>,
    mutations_notifier: Option<Arc<Notify>>,
    event_emitter: EventEmitter,
    event_receiver: EventReceiver,
    window_env: WindowEnv<State>,
    layers: Layers,
    elements_state: ElementsState,
    viewports: Viewports,
    focus_sender: FocusSender,
    focus_receiver: FocusReceiver,
    accessibility: NativeAccessibility,
    font_collection: FontCollection,
    ticker_sender: broadcast::Sender<()>,
    plugins: PluginsManager,
    navigator_state: NavigatorState,
}

impl<State: 'static + Clone> App<State> {
    pub fn new(
        sdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventMessage>,
        mutations_notifier: Option<Arc<Notify>>,
        mut window_env: WindowEnv<State>,
        fonts_config: FontsConfig,
        mut plugins: PluginsManager,
    ) -> Self {
        let accessibility = NativeAccessibility::new(&window_env.window, proxy.clone());

        window_env.window_mut().set_visible(true);

        let mut font_collection = FontCollection::new();
        let def_mgr = FontMgr::default();

        let mut provider = TypefaceFontProvider::new();

        for (font_name, font_data) in fonts_config {
            let ft_type = def_mgr.new_from_data(font_data, None).unwrap();
            provider.register_typeface(ft_type, Some(font_name));
        }

        let mgr: FontMgr = provider.into();
        font_collection.set_default_font_manager(def_mgr, "Fira Sans");
        font_collection.set_dynamic_font_manager(mgr);

        let (event_emitter, event_receiver) = mpsc::unbounded_channel::<DomEvent>();
        let (focus_sender, focus_receiver) = watch::channel(None);

        plugins.send(PluginEvent::WindowCreated(window_env.window_mut()));

        Self {
            sdom,
            vdom,
            events: EventsQueue::new(),
            vdom_waker: winit_waker(proxy),
            proxy: proxy.clone(),
            mutations_notifier,
            event_emitter,
            event_receiver,
            window_env,
            layers: Layers::default(),
            elements_state: ElementsState::default(),
            viewports: Viewports::default(),
            accessibility,
            focus_sender,
            focus_receiver,
            font_collection,
            ticker_sender: broadcast::channel(5).0,
            plugins,
            navigator_state: NavigatorState::new(NavigationMode::NotKeyboard),
        }
    }

    /// Provide the launch state and few other utilities like the EventLoopProxy
    pub fn provide_vdom_contexts(&mut self) {
        if let Some(state) = self.window_env.window_config.state.clone() {
            self.vdom.insert_any_root_context(Box::new(state));
        }
        self.vdom
            .insert_any_root_context(Box::new(self.proxy.clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.focus_receiver.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        self.vdom
            .insert_any_root_context(Box::new(self.navigator_state.clone()));
    }

    /// Make the first build of the VirtualDOM.
    pub fn init_vdom(&mut self) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        self.provide_vdom_contexts();

        self.sdom.get_mut().init_dom(&mut self.vdom, scale_factor);
    }

    /// Update the DOM with the mutations from the VirtualDOM.
    pub fn apply_vdom_changes(&mut self) -> (bool, bool) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        let (repaint, relayout) = self
            .sdom
            .get_mut()
            .render_mutations(&mut self.vdom, scale_factor);

        if repaint {
            if let Some(mutations_notifier) = &self.mutations_notifier {
                mutations_notifier.notify_one();
            }
        }

        (repaint, relayout)
    }

    /// Poll the VirtualDOM for any new change
    pub fn poll_vdom(&mut self) {
        let waker = &self.vdom_waker.clone();
        let mut cx = std::task::Context::from_waker(waker);

        loop {
            {
                let fut = async {
                    select! {
                        ev = self.event_receiver.recv() => {
                            if let Some(ev) = ev {
                                let data = ev.data.any();
                                self.vdom.handle_event(&ev.name, data, ev.element_id, false);

                                self.vdom.process_events();
                            }
                        },
                        _ = self.vdom.wait_for_work() => {},
                    }
                };
                pin_mut!(fut);

                match fut.poll_unpin(&mut cx) {
                    std::task::Poll::Ready(_) => {}
                    std::task::Poll::Pending => break,
                }
            }

            let (must_repaint, must_relayout) = self.apply_vdom_changes();

            if must_relayout {
                self.window_env.window.request_redraw();
            } else if must_repaint {
                self.proxy.send_event(EventMessage::RequestRedraw).unwrap();
            }
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self) {
        let scale_factor = self.window_env.window.scale_factor();
        process_events(
            &self.sdom.get(),
            &self.layers,
            &mut self.events,
            &self.event_emitter,
            &mut self.elements_state,
            &self.viewports,
            scale_factor,
        )
    }

    /// Measure the layout
    pub fn process_layout(&mut self) {
        self.accessibility.clear_accessibility();

        {
            let dom = self.sdom.get();

            self.plugins.send(PluginEvent::StartedLayout(&dom.layout()));

            let (layers, viewports) = self
                .window_env
                .process_layout(&dom, &mut self.font_collection);
            self.layers = layers;
            self.viewports = viewports;

            self.plugins
                .send(PluginEvent::FinishedLayout(&dom.layout()));
        }

        info!(
            "Processed {} layers and {} group of paragraph elements",
            self.layers.len_layers(),
            self.layers.len_paragraph_elements()
        );
        info!("Processed {} viewports", self.viewports.size());

        if let Some(mutations_notifier) = &self.mutations_notifier {
            mutations_notifier.notify_one();
        }

        self.process_accessibility();
    }

    /// Create the Accessibility tree
    /// This will iterater the DOM ordered by layers (top to bottom)
    /// and add every element with an accessibility ID to the Accessibility Tree
    pub fn process_accessibility(&mut self) {
        let fdom = &self.sdom.get();
        let layout = fdom.layout();
        let rdom = fdom.rdom();
        let layers = &self.layers;

        process_accessibility(
            layers,
            &layout,
            rdom,
            &mut *self.accessibility.accessibility_state().lock().unwrap(),
        );
    }

    /// Send an event
    pub fn send_event(&mut self, event: FreyaEvent) {
        self.events.push(event);
        self.process_events();
    }

    /// Replace a VirtualDOM Template
    pub fn vdom_replace_template(&mut self, template: Template) {
        self.vdom.replace_template(template);
    }

    /// Render the RealDOM into the Window
    pub fn render(&mut self, hovered_node: &HoveredNode) {
        self.plugins.send(PluginEvent::BeforeRender {
            canvas: self.window_env.canvas(),
            font_collection: &self.font_collection,
            freya_dom: &self.sdom.get(),
            viewports: &self.viewports,
        });

        self.window_env.start_render(
            &self.layers,
            &self.viewports,
            &mut self.font_collection,
            hovered_node,
            &self.sdom.get(),
        );

        self.accessibility
            .render_accessibility(self.window_env.window.title().as_str());

        self.plugins.send(PluginEvent::AfterRender {
            canvas: self.window_env.canvas(),
            font_collection: &self.font_collection,
            freya_dom: &self.sdom.get(),
            viewports: &self.viewports,
        });

        self.window_env.finish_render();
    }

    /// Resize the Window
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.sdom.get().layout().reset();
        self.window_env.resize(size);
    }

    pub fn measure_text_group(&self, text_id: &Uuid) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        self.layers.measure_paragraph_elements(
            text_id,
            &self.sdom.get(),
            &self.font_collection,
            scale_factor,
        );
    }

    pub fn window_env(&mut self) -> &mut WindowEnv<State> {
        &mut self.window_env
    }

    pub fn accessibility(&mut self) -> &mut NativeAccessibility {
        &mut self.accessibility
    }

    pub fn on_window_event(&mut self, event: &WindowEvent) -> bool {
        self.accessibility
            .on_accessibility_window_event(&self.window_env.window, event)
    }

    pub fn focus_next_node(&mut self, direction: AccessibilityFocusDirection) {
        self.accessibility
            .focus_next_node(direction, &self.focus_sender)
    }

    pub fn tick(&self) {
        self.ticker_sender.send(()).ok();
    }

    pub fn set_navigation_mode(&mut self, mode: NavigationMode) {
        self.navigator_state.set(mode);
    }
}
