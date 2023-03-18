use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    task::Waker,
};

use accesskit::{NodeClassSet, NodeId};
use accesskit_winit::Adapter;
use dioxus_core::{Template, VirtualDom};
use dioxus_native_core::SendAnyMap;
use freya_common::EventMessage;
use freya_core::{
    dom::DioxusSafeDOM,
    events::{DomEvent, EventsProcessor, FreyaEvent},
    process_events, EventEmitter, EventReceiver, EventsQueue, FocusReceiver, FocusSender,
    ViewportsCollection,
};
use freya_layout::Layers;
use futures::FutureExt;
use futures::{
    pin_mut,
    task::{self, ArcWake},
};
use glutin::{dpi::PhysicalSize, event::WindowEvent, event_loop::EventLoopProxy};
use tokio::{
    select,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        watch,
    },
};

use crate::{
    create_surface,
    window::{AccessibilityState, FocusDirection},
    HoveredNode, WindowEnv,
};

pub fn winit_waker(proxy: &EventLoopProxy<EventMessage>) -> std::task::Waker {
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

/// Manager for an Application
pub struct App<State: 'static + Clone> {
    rdom: DioxusSafeDOM,
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
    adapter: Adapter,
}

impl<State: 'static + Clone> App<State> {
    pub fn new(
        rdom: DioxusSafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventMessage>,
        mutations_sender: Option<UnboundedSender<()>>,
        window_env: WindowEnv<State>,
    ) -> Self {
        let accessibility_state = AccessibilityState {
            nodes: Vec::new(),
            node_classes: NodeClassSet::new(),
            focus: None,
        }
        .wrap();
        let adapter = {
            let accessibility_state = accessibility_state.clone();
            Adapter::new(
                window_env.windowed_context.window(),
                move || {
                    let mut accessibility_state = accessibility_state.lock().unwrap();
                    accessibility_state.process()
                },
                proxy.clone(),
            )
        };
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
            adapter,
            accessibility_state,
            focus_sender,
            focus_receiver,
        }
    }

    /// Provide the launch state and few other utilities like the [EventLoopProxy]
    pub fn provide_vdom_contexts(&self) {
        if let Some(state) = self.window_env.window_config.state.clone() {
            self.vdom.base_scope().provide_context(state);
        }
        self.vdom.base_scope().provide_context(self.proxy.clone());
        self.vdom
            .base_scope()
            .provide_context(self.focus_receiver.clone());
    }

    /// Make an first build of the [VirtualDOM]
    pub fn init_vdom(&mut self) {
        self.provide_vdom_contexts();

        let mutations = self.vdom.rebuild();
        let (to_update, diff) = self.rdom.dom_mut().apply_mutations(mutations);

        if !diff.is_empty() {
            self.mutations_sender.as_ref().map(|s| s.send(()));
        }

        self.rdom
            .dom_mut()
            .update_state(to_update, SendAnyMap::new());
    }

    /// Update the [RealDOM] with changes from the [VirtualDOM]
    pub fn apply_vdom_changes(&mut self) -> bool {
        let mutations = self.vdom.render_immediate();
        let (to_update, diff) = self.rdom.dom_mut().apply_mutations(mutations);

        if !diff.is_empty() {
            self.mutations_sender.as_ref().map(|s| s.send(()));
        }

        self.rdom
            .dom_mut()
            .update_state(to_update, SendAnyMap::new());

        !diff.is_empty()
    }

    /// Poll the [VirtualDOM] for any new change
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

            if self.apply_vdom_changes() {
                self.request_redraw();
            }
        }
    }

    /// Process the events queue
    pub fn process_events(&mut self) {
        process_events(
            &self.rdom.dom(),
            &self.layers,
            &mut self.events,
            &self.event_emitter,
            &mut self.events_processor,
            &self.viewports_collection,
        )
    }

    /// Measure the layout
    pub fn process_layout(&mut self) {
        let (layers, viewports) = self.window_env.process_layout(&self.rdom.dom());
        self.layers = layers;
        self.viewports_collection = viewports;
        self.process_accessibility();
    }

    /// Create the Accessibility tree
    pub fn process_accessibility(&mut self) {
        // TODO: move logic to core
        for layer in self.layers.layers.values() {
            for node in layer.values() {
                if let Some(accessibility_id) =
                    node.get_node(&self.rdom.dom()).state.accessibility.focus_id
                {
                    let children = node.get_accessibility_children(&self.rdom.dom());
                    self.accessibility_state.lock().unwrap().add_element(
                        node,
                        accessibility_id,
                        children,
                    );
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

    /// Replace a [VirtualDOM] Template
    pub fn vdom_replace_template(&mut self, template: Template<'static>) {
        self.vdom.replace_template(template);
    }

    /// Render the [RealDOM] into the [Window]
    pub fn render(&mut self, hovered_node: &HoveredNode) {
        self.window_env.render(
            &self.layers,
            &self.viewports_collection,
            hovered_node,
            &self.rdom.dom(),
        );
    }

    /// Resize the [Window]
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let mut context = self.window_env.gr_context.clone();
        self.window_env.surface = create_surface(
            &self.window_env.windowed_context,
            &self.window_env.fb_info,
            &mut context,
        );
        self.window_env.windowed_context.resize(size);
        self.window_env.windowed_context.window().request_redraw();
    }

    /// Focus a new accessibility node
    pub fn set_accessibility_focus(&mut self, id: NodeId) {
        self.accessibility_state
            .lock()
            .unwrap()
            .set_focus(&self.adapter, id, &self.focus_sender);
    }

    /// Validate a winit event for accessibility
    pub fn on_accessibility_window_event(&mut self, event: &WindowEvent) -> bool {
        self.adapter
            .on_event(self.window_env.windowed_context.window(), event)
    }

    /// Remove the accessibility nodes
    pub fn clear_accessibility(&mut self) {
        self.accessibility_state.lock().unwrap().clear();
    }

    /// Process the accessibility nodes
    pub fn render_accessibility(&mut self) {
        let tree = self.accessibility_state.lock().unwrap().process();
        self.adapter.update(tree);
    }

    /// Focus the next accessibility node
    pub fn focus_next_node(&mut self, direction: FocusDirection) {
        self.accessibility_state
            .lock()
            .unwrap()
            .set_focus_on_next_node(&self.adapter, direction, &self.focus_sender);
    }
}
