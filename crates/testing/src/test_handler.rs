use std::sync::Arc;
use std::time::Duration;

use accesskit::NodeId as AccessibilityId;
use dioxus_core::VirtualDom;
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_engine::prelude::FontCollection;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{interval, timeout};
use torin::geometry::{Area, Size2D};

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
    pub(crate) events_queue: EventsQueue,
    pub(crate) elements_state: ElementsState,
    pub(crate) font_collection: FontCollection,
    pub(crate) viewports: Viewports,
    pub(crate) accessibility_manager: SharedAccessibilityManager,
    pub(crate) config: TestingConfig,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) navigation_state: NavigatorState,
}

impl TestingHandler {
    /// Init the DOM.
    pub(crate) fn init_dom(&mut self) {
        self.provide_vdom_contexts();
        let sdom = self.utils.sdom();
        let mut fdom = sdom.get();
        fdom.init_dom(&mut self.vdom, SCALE_FACTOR as f32);
    }

    /// Get a mutable reference to the current [`TestingConfig`].
    pub fn config(&mut self) -> &mut TestingConfig {
        &mut self.config
    }

    /// Provide some values to the app
    fn provide_vdom_contexts(&mut self) {
        self.vdom
            .insert_any_root_context(Box::new(self.platform_event_emitter.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        self.vdom
            .insert_any_root_context(Box::new(self.navigation_state.clone()));
    }

    /// Wait and apply new changes
    pub async fn wait_for_update(&mut self) -> (bool, bool) {
        self.wait_for_work(self.config.size());

        self.provide_vdom_contexts();

        let mut ticker = if self.config.run_ticker {
            Some(interval(Duration::from_millis(16)))
        } else {
            None
        };

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
                        if let Some(ticker) = ticker.as_mut() {
                            ticker.tick().await;
                            self.ticker_sender.send(()).unwrap();
                            timeout(self.config.vdom_timeout(), self.vdom.wait_for_work())
                                .await
                                .ok();
                        }
                    }
                    EventMessage::FocusAccessibilityNode(node_id) => {
                        self.accessibility_manager.lock().unwrap().focused_id = node_id;
                    }
                    _ => {}
                }
            }

            if let Ok(ev) = vdom_ev {
                self.vdom
                    .handle_event(&ev.name, ev.data.any(), ev.element_id, true);
                self.vdom.process_events();
            }
        }

        timeout(self.config.vdom_timeout(), self.vdom.wait_for_work())
            .await
            .ok();

        let (must_repaint, must_relayout) = self
            .utils
            .sdom()
            .get_mut()
            .render_mutations(&mut self.vdom, SCALE_FACTOR as f32);

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
            &mut self.elements_state,
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

    pub fn focus_id(&self) -> AccessibilityId {
        self.accessibility_manager.lock().unwrap().focused_id
    }
}
