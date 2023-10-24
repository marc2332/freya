use std::sync::Arc;
use std::time::Duration;

use accesskit::NodeId as AccessibilityId;
use dioxus_core::VirtualDom;
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_engine::prelude::FontCollection;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use torin::geometry::{Area, Size2D};

pub use freya_core::events::FreyaEvent;
pub use freya_elements::events::mouse::MouseButton;
use tokio::time::{sleep, timeout};

use crate::test_node::TestNode;
use crate::test_utils::TestUtils;
use crate::{TestingConfig, SCALE_FACTOR};

/// Manages the lifecycle of your tests.
pub struct TestingHandler {
    pub(crate) vdom: VirtualDom,
    pub(crate) utils: TestUtils,

    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,

    pub(crate) platform_event_emitter: UnboundedSender<EventMessage>,
    pub(crate) platform_event_receiver: UnboundedReceiver<EventMessage>,

    pub(crate) events_queue: Vec<FreyaEvent>,
    pub(crate) events_processor: EventsProcessor,
    pub(crate) font_collection: FontCollection,
    pub(crate) viewports: ViewportsCollection,
    pub(crate) accessibility_state: SharedAccessibilityState,

    pub(crate) config: TestingConfig,

    pub(crate) ticker_sender: broadcast::Sender<()>,
}

impl TestingHandler {
    /// Init the DOM.
    pub(crate) fn init_dom(&mut self) {
        self.provide_vdom_contexts();
        let sdom = self.utils.sdom();
        let mut fdom = sdom.get();
        let mutations = self.vdom.rebuild();
        fdom.init_dom(mutations, SCALE_FACTOR as f32);
    }

    /// Get a mutable reference to the current [`TestingConfig`].
    pub fn config(&mut self) -> &mut TestingConfig {
        &mut self.config
    }

    /// Provide some values to the app
    fn provide_vdom_contexts(&self) {
        self.vdom
            .base_scope()
            .provide_context(self.platform_event_emitter.clone());
        self.vdom
            .base_scope()
            .provide_context(Arc::new(self.ticker_sender.subscribe()));
    }

    /// Wait and apply new changes
    pub async fn wait_for_update(&mut self) -> (bool, bool) {
        self.wait_for_work(self.config.size());

        self.provide_vdom_contexts();

        let vdom = &mut self.vdom;

        // Handle platform and VDOM events
        loop {
            let platform_ev = self.platform_event_receiver.try_recv();
            let vdom_ev = self.event_receiver.try_recv();

            if vdom_ev.is_err() && platform_ev.is_err() {
                break;
            }

            if let Ok(ev) = platform_ev {
                match ev {
                    EventMessage::RequestRerender => {
                        if self.config.run_ticker {
                            sleep(Duration::from_millis(16)).await;
                            self.ticker_sender.send(()).unwrap();
                        }
                    }
                    EventMessage::FocusAccessibilityNode(node_id) => {
                        self.accessibility_state
                            .lock()
                            .unwrap()
                            .set_focus(Some(node_id));
                    }
                    _ => {}
                }
            }

            if let Ok(ev) = vdom_ev {
                vdom.handle_event(&ev.name, ev.data.any(), ev.element_id, false);
                vdom.process_events();
            }
        }

        timeout(self.config.vdom_timeout(), vdom.wait_for_work())
            .await
            .ok();

        let mutations = self.vdom.render_immediate();

        let (must_repaint, must_relayout) = self
            .utils
            .sdom()
            .get_mut()
            .apply_mutations(mutations, SCALE_FACTOR as f32);

        self.wait_for_work(self.config.size());

        self.ticker_sender.send(()).unwrap();

        (must_repaint, must_relayout)
    }

    /// Wait for layout and events to be processed
    pub fn wait_for_work(&mut self, size: Size2D) {
        // Clear cached results
        self.utils.sdom().get_mut().layout().reset();

        // Measure layout
        let (layers, viewports) = process_layout(
            &self.utils.sdom().get(),
            Area {
                origin: (0.0, 0.0).into(),
                size,
            },
            &mut self.font_collection,
            SCALE_FACTOR as f32,
        );

        *self.utils.layers().lock().unwrap() = layers;
        self.viewports = viewports;

        process_events(
            &self.utils.sdom().get(),
            &self.utils.layers().lock().unwrap(),
            &mut self.events_queue,
            &self.event_emitter,
            &mut self.events_processor,
            &self.viewports,
            SCALE_FACTOR,
        );
    }

    /// Push an event to the events queue
    pub fn push_event(&mut self, event: FreyaEvent) {
        self.events_queue.push(event);
    }

    /// Get the root node
    pub fn root(&mut self) -> TestNode {
        let root_id = {
            let sdom = self.utils.sdom();
            let fdom = sdom.get();
            let rdom = fdom.rdom();
            rdom.root_id()
        };

        self.utils.get_node_by_id(root_id)
    }

    pub fn focus_id(&self) -> Option<AccessibilityId> {
        self.accessibility_state.lock().unwrap().focus_id()
    }
}
