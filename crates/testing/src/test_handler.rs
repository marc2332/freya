use std::sync::{Arc, Mutex};
use std::time::Duration;

use dioxus_core::VirtualDom;
use freya_common::EventMessage;
use freya_core::prelude::*;
use freya_engine::prelude::FontCollection;
use freya_hooks::PlatformInformation;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{interval, timeout};
use torin::geometry::{Area, Size2D};
use winit::window::CursorIcon;

use crate::config::TestingConfig;
use crate::test_node::TestNode;
use crate::test_utils::TestUtils;
use crate::SCALE_FACTOR;

/// Manages the lifecycle of your tests.
pub struct TestingHandler {
    pub(crate) vdom: VirtualDom,
    pub(crate) utils: TestUtils,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) platform_event_emitter: UnboundedSender<EventMessage>,
    pub(crate) platform_event_receiver: UnboundedReceiver<EventMessage>,
    pub(crate) events_queue: EventsQueue,
    pub(crate) nodes_state: NodesState,
    pub(crate) focus_sender: FocusSender,
    pub(crate) focus_receiver: FocusReceiver,
    pub(crate) font_collection: FontCollection,
    pub(crate) accessibility_manager: SharedAccessibilityManager,
    pub(crate) config: TestingConfig,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) navigation_state: NavigatorState,
    pub(crate) platform_information: Arc<Mutex<PlatformInformation>>,
    pub(crate) cursor_icon: CursorIcon,
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
            .insert_any_root_context(Box::new(self.focus_receiver.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        self.vdom
            .insert_any_root_context(Box::new(self.navigation_state.clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.platform_information.clone()));
    }

    /// Wait and apply new changes
    pub async fn wait_for_update(&mut self) -> (bool, bool) {
        self.wait_for_work(self.config.size());

        let mut ticker = if self.config.event_loop_ticker {
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
                        let tree = self
                            .accessibility_manager
                            .lock()
                            .unwrap()
                            .set_focus_with_update(node_id);

                        if let Some(tree) = tree {
                            self.focus_sender
                                .send(tree.focus)
                                .expect("Failed to focus the Node.");
                        }
                    }
                    EventMessage::FocusNextAccessibilityNode => {
                        let tree = self
                            .accessibility_manager
                            .lock()
                            .unwrap()
                            .set_focus_on_next_node(AccessibilityFocusDirection::Forward);
                        self.focus_sender
                            .send(tree.focus)
                            .expect("Failed to focus the Node.");
                    }
                    EventMessage::FocusPrevAccessibilityNode => {
                        let tree = self
                            .accessibility_manager
                            .lock()
                            .unwrap()
                            .set_focus_on_next_node(AccessibilityFocusDirection::Backward);
                        self.focus_sender
                            .send(tree.focus)
                            .expect("Failed to focus the Node.");
                    }
                    EventMessage::SetCursorIcon(icon) => {
                        self.cursor_icon = icon;
                    }
                    _ => {}
                }
            }

            if let Ok(ev) = vdom_ev {
                self.vdom
                    .handle_event(ev.name.into(), ev.data.any(), ev.element_id, ev.bubbles);
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
        process_layout(
            &self.utils.sdom().get(),
            Area {
                origin: (0.0, 0.0).into(),
                size,
            },
            &mut self.font_collection,
            SCALE_FACTOR as f32,
            &["Fira Sans".to_string()],
        );

        let dom = &self.utils.sdom().get_mut();

        process_accessibility(
            &dom.layout(),
            dom.rdom(),
            &mut self.accessibility_manager.lock().unwrap(),
        );

        process_events(
            dom,
            &mut self.events_queue,
            &self.event_emitter,
            &mut self.nodes_state,
            SCALE_FACTOR,
        );
    }

    /// Push an event to the events queue
    pub fn push_event(&mut self, event: PlatformEvent) {
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

        self.utils
            .get_node_by_id(root_id)
            // Get get the first element because of `KeyboardNavigator`
            .get(0)
    }

    /// Get the current [AccessibilityId].
    pub fn focus_id(&self) -> AccessibilityId {
        self.accessibility_manager.lock().unwrap().focused_id
    }

    /// Resize the simulated canvas.
    pub fn resize(&mut self, size: Size2D) {
        self.config.size = size;
        self.platform_information.lock().unwrap().window_size = size;
    }

    /// Get the current [CursorIcon].
    pub fn cursor_icon(&self) -> CursorIcon {
        self.cursor_icon
    }

    /// Get the [SafeDOM]
    pub fn sdom(&self) -> &SafeDOM {
        self.utils.sdom()
    }
}
