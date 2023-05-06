use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::Waker,
};

use accesskit::NodeId;
use accesskit_winit::Adapter;
use dioxus_core::{Template, VirtualDom};
use dioxus_native_core::real_dom::NodeImmutable;
use freya_common::EventMessage;
use freya_core::{
    events::{DomEvent, EventsProcessor, FreyaEvent},
    process_events, EventEmitter, EventReceiver, EventsQueue, FocusReceiver, FocusSender,
    ViewportsCollection,
};
use freya_dom::SafeDOM;
use freya_layout::Layers;
use freya_node_state::AccessibilitySettings;
use futures::FutureExt;
use futures::{
    pin_mut,
    task::{self, ArcWake},
};
use tokio::{
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        watch,
    },
};
use uuid::Uuid;
use winit::{dpi::PhysicalSize, event::WindowEvent, event_loop::EventLoopProxy, window::Window};

use crate::{
    accessibility::{AccessibilityFocusDirection, AccessibilityState, SharedAccessibilityState},
    HoveredNode, WindowEnv,
};

fn winit_waker(proxy: &EventLoopProxy<EventMessage>) -> std::task::Waker {
    struct DomHandle(EventLoopProxy<EventMessage>);

    unsafe impl Send for DomHandle {}
    unsafe impl Sync for DomHandle {}

    impl ArcWake for DomHandle {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self.0.send_event(EventMessage::PollVDOM);
        }
    }

    task::waker(Arc::new(DomHandle(proxy.clone())))
}

fn create_accessibility_adapter(
    window: &Window,
    window_title: String,
    accessibility_state: SharedAccessibilityState,
    proxy: &EventLoopProxy<EventMessage>,
) -> Adapter {
    Adapter::new(
        window,
        move || {
            let mut accessibility_state = accessibility_state.lock().unwrap();
            accessibility_state.process(&window_title)
        },
        proxy.clone(),
    )
}

/// Manages the Application lifecycle
pub struct App<State: 'static + Clone> {
    rdom: SafeDOM,
    vdom: VirtualDom,

    events: EventsQueue,

    vdom_waker: Waker,
    proxy: EventLoopProxy<EventMessage>,
    mutations_sender: Option<UnboundedSender<()>>,

    event_emitter: EventEmitter,
    event_receiver: EventReceiver,

    window_env: WindowEnv<State>,

    layers: Layers,
    events_processor: EventsProcessor,
    viewports_collection: ViewportsCollection,

    focus_sender: FocusSender,
    focus_receiver: FocusReceiver,

    accessibility_state: Arc<Mutex<AccessibilityState>>,
    accessibility_adapter: Adapter,
}

impl<State: 'static + Clone> App<State> {
    pub fn new(
        rdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventMessage>,
        mutations_sender: Option<UnboundedSender<()>>,
        window_env: WindowEnv<State>,
    ) -> Self {
        let accessibility_state = AccessibilityState::new().wrap();
        let accessibility_adapter = create_accessibility_adapter(
            &window_env.window,
            window_env.window_config.title.to_string(),
            accessibility_state.clone(),
            proxy,
        );

        let (event_emitter, event_receiver) = unbounded_channel::<DomEvent>();
        let (focus_sender, focus_receiver) = watch::channel(None);
        Self {
            rdom,
            vdom,
            events: Vec::new(),
            vdom_waker: winit_waker(proxy),
            proxy: proxy.clone(),
            mutations_sender,
            event_emitter,
            event_receiver,
            window_env,
            layers: Layers::default(),
            events_processor: EventsProcessor::default(),
            viewports_collection: HashMap::default(),
            accessibility_adapter,
            accessibility_state,
            focus_sender,
            focus_receiver,
        }
    }

    /// Provide the launch state and few other utilities like the EventLoopProxy
    pub fn provide_vdom_contexts(&self) {
        if let Some(state) = self.window_env.window_config.state.clone() {
            self.vdom.base_scope().provide_context(state);
        }
        self.vdom.base_scope().provide_context(self.proxy.clone());
        self.vdom
            .base_scope()
            .provide_context(self.focus_receiver.clone());
    }

    /// Make the first build of the VirtualDOM.
    pub fn init_vdom(&mut self) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        self.provide_vdom_contexts();

        let mutations = self.vdom.rebuild();

        self.rdom.get_mut().init_dom(mutations, scale_factor);

        self.mutations_sender.as_ref().map(|s| s.send(()));
    }

    /// Update the DOM with the mutations from the VirtualDOM.
    pub fn apply_vdom_changes(&mut self) -> (bool, bool) {
        let scale_factor = self.window_env.window.scale_factor() as f32;
        let mutations = self.vdom.render_immediate();

        let (repaint, relayout) = self.rdom.get_mut().apply_mutations(mutations, scale_factor);

        if repaint || relayout {
            self.mutations_sender.as_ref().map(|s| s.send(()));
        }

        (repaint, relayout)
    }

    /// Poll the VirtualDOM for any new change
    pub fn poll_vdom(&mut self) {
        let waker = &self.vdom_waker.clone();
        let mut cx = std::task::Context::from_waker(waker);

        loop {
            self.provide_vdom_contexts();

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
                self.request_redraw();
            } else if must_repaint {
                self.request_rerender();
            }
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self) {
        let scale_factor = self.window_env.window.scale_factor();
        process_events(
            &self.rdom.get(),
            &self.layers,
            &mut self.events,
            &self.event_emitter,
            &mut self.events_processor,
            &self.viewports_collection,
            scale_factor,
        )
    }

    /// Measure the layout
    pub fn process_layout(&mut self) {
        let (layers, viewports) = self.window_env.process_layout(&self.rdom.get());
        self.layers = layers;
        self.viewports_collection = viewports;
        self.process_accessibility();
    }

    /// Create the Accessibility tree
    pub fn process_accessibility(&mut self) {
        let dom = &self.rdom.get();

        for layer in self.layers.layers.values() {
            for render_node in layer.values() {
                let dioxus_node = render_node.get_node(dom);
                if let Some(dioxus_node) = dioxus_node {
                    let node_accessibility = &*dioxus_node.get::<AccessibilitySettings>().unwrap();
                    if let Some(accessibility_id) = node_accessibility.focus_id {
                        self.accessibility_state.lock().unwrap().add_element(
                            render_node,
                            accessibility_id,
                            node_accessibility,
                            dom,
                        );
                    }
                }
            }
        }
    }

    /// Push an event to the events queue
    pub fn push_event(&mut self, event: FreyaEvent) {
        self.events.push(event);
    }

    /// Request a redraw
    pub fn request_redraw(&self) {
        self.window_env.request_redraw();
    }

    /// Request a rerender
    pub fn request_rerender(&self) {
        self.proxy
            .send_event(EventMessage::RequestRerender)
            .unwrap();
    }

    /// Replace a VirtualDOM Template
    pub fn vdom_replace_template(&mut self, template: Template<'static>) {
        self.vdom.replace_template(template);
    }

    /// Render the RealDOM into the Window
    pub fn render(&mut self, hovered_node: &HoveredNode) {
        self.window_env.render(
            &self.layers,
            &self.viewports_collection,
            hovered_node,
            &self.rdom.get(),
        );
    }

    /// Resize the Window
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.window_env.resize(size);
    }

    /// Focus a new accessibility node
    pub fn set_accessibility_focus(&mut self, id: NodeId) {
        self.accessibility_state
            .lock()
            .unwrap()
            .set_focus(&self.accessibility_adapter, id);
    }

    /// Validate a winit event for accessibility
    pub fn on_accessibility_window_event(&mut self, event: &WindowEvent) -> bool {
        self.accessibility_adapter
            .on_event(&self.window_env.window, event)
    }

    /// Remove the accessibility nodes
    pub fn clear_accessibility(&mut self) {
        self.accessibility_state.lock().unwrap().clear();
    }

    /// Process the accessibility nodes
    pub fn render_accessibility(&mut self) {
        let tree = self
            .accessibility_state
            .lock()
            .unwrap()
            .process(self.window_env.window_config.title);
        self.accessibility_adapter.update(tree);
    }

    /// Focus the next accessibility node
    pub fn focus_next_node(&mut self, direction: AccessibilityFocusDirection) {
        self.accessibility_state
            .lock()
            .unwrap()
            .set_focus_on_next_node(&self.accessibility_adapter, direction, &self.focus_sender);
    }

    pub fn measure_text_group(&self, text_id: &Uuid) {
        self.layers.measure_paragraph_elements(
            text_id,
            &self.rdom.get(),
            &self.window_env.font_collection,
        );
    }

    pub fn window_env(&mut self) -> &mut WindowEnv<State> {
        &mut self.window_env
    }
}
