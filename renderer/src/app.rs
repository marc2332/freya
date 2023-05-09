use std::{collections::HashMap, sync::Arc, task::Waker};

use dioxus_core::{Template, VirtualDom};
use freya_common::EventMessage;
use freya_core::{
    events::{DomEvent, EventsProcessor, FreyaEvent},
    process_events, EventEmitter, EventReceiver, EventsQueue, ViewportsCollection,
};
use freya_dom::SafeDOM;
use freya_layout::Layers;
use futures::FutureExt;
use futures::{
    pin_mut,
    task::{self, ArcWake},
};
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};
use uuid::Uuid;
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy};

use crate::{HoveredNode, WindowEnv};

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
}

impl<State: 'static + Clone> App<State> {
    pub fn new(
        rdom: SafeDOM,
        vdom: VirtualDom,
        proxy: &EventLoopProxy<EventMessage>,
        mutations_sender: Option<UnboundedSender<()>>,
        window_env: WindowEnv<State>,
    ) -> Self {
        let (event_emitter, event_receiver) = unbounded_channel::<DomEvent>();
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
        }
    }

    /// Provide the launch state and few other utilities like the EventLoopProxy
    pub fn provide_vdom_contexts(&self) {
        if let Some(state) = self.window_env.window_config.state.clone() {
            self.vdom.base_scope().provide_context(state);
        }
        self.vdom.base_scope().provide_context(self.proxy.clone());
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

        let is_empty = mutations.dirty_scopes.is_empty()
            && mutations.edits.is_empty()
            && mutations.templates.is_empty();

        let (repaint, relayout) = if !is_empty {
            self.rdom.get_mut().apply_mutations(mutations, scale_factor)
        } else {
            (false, false)
        };

        if repaint {
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
                self.window_env.window.request_redraw();
            } else if must_repaint {
                self.proxy
                    .send_event(EventMessage::RequestRerender)
                    .unwrap();
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
        let dom = self.rdom.get();
        let (layers, viewports) = self.window_env.process_layout(&dom);
        self.layers = layers;
        self.viewports_collection = viewports;
    }

    /// Push an event to the events queue
    pub fn push_event(&mut self, event: FreyaEvent) {
        self.events.push(event);
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
        self.rdom.get().layout().reset();
        self.window_env.resize(size);
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
