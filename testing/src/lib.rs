use std::sync::{Arc, Mutex};

use dioxus_core::{Component, VirtualDom};
use dioxus_native_core::node::NodeType;
use dioxus_native_core::real_dom::RealDom;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::NodeId;
use euclid::Size2D;
use freya_common::NodeArea;
use freya_core::events::EventsProcessor;
use freya_core::{events::DomEvent, EventEmitter, EventReceiver};
use freya_core::{process_events, process_layout, ViewportsCollection};
use freya_dom::{DioxusNode, FreyaDOM, SafeDOM};
use freya_layout::Layers;
use freya_node_state::NodeState;
use rustc_hash::FxHashMap;
use skia_safe::textlayout::FontCollection;
use skia_safe::FontMgr;
use tokio::sync::mpsc::unbounded_channel;

pub use freya_core::events::FreyaEvent;
pub use freya_elements::events::mouse::MouseButton;
use tokio::time::timeout;

pub use config::*;

mod config;

#[derive(Clone)]
pub struct TestUtils {
    dom: SafeDOM,
    layers: Arc<Mutex<Layers>>,
}

impl TestUtils {
    /// Get a Node by the given ID
    pub fn get_node_by_id(&self, node_id: NodeId) -> TestNode {
        let dom = self.dom.get();
        let rdom = dom.dom();
        let child: &DioxusNode = rdom.get(node_id).unwrap();
        let height = rdom.tree.height(node_id).unwrap();
        let parent_id = rdom.tree.parent_id(node_id);
        let children_ids = rdom.tree.children_ids(node_id).map(|v| v.to_vec());
        TestNode {
            node_id,
            utils: self.clone(),
            node: child.clone(),
            height,
            parent_id,
            children_ids,
        }
    }
}

/// Represents a `Node` in the DOM.
#[derive(Clone)]
pub struct TestNode {
    node_id: NodeId,
    utils: TestUtils,
    height: u16,
    children_ids: Option<Vec<NodeId>>,
    node: DioxusNode,
    parent_id: Option<NodeId>,
}

impl TestNode {
    /// Get a child of the Node by the given index
    pub fn child(&self, child_index: usize) -> Option<Self> {
        let children_ids = self.children_ids.as_ref()?;
        let child_id = children_ids.get(child_index)?;
        let child: TestNode = self.utils.get_node_by_id(*child_id);
        Some(child)
    }

    /// Get the Node text
    pub fn text(&self) -> Option<&str> {
        if let NodeType::Text { text } = &self.node.node_data.node_type {
            Some(text.as_str())
        } else {
            None
        }
    }

    /// Get the Node state
    pub fn state(&self) -> &NodeState {
        &self.node.state
    }

    /// Get the Node layout
    pub fn layout(&self) -> Option<NodeArea> {
        let layers = &self.utils.layers.lock().unwrap().layers;
        for layer in layers.values() {
            for (id, node) in layer {
                if id == &self.node_id {
                    return Some(node.node_area);
                }
            }
        }
        None
    }

    /// Get a mutable reference to the test utils.
    pub fn utils(&mut self) -> &mut TestUtils {
        &mut self.utils
    }

    /// Get the NodeId from the parent
    pub fn parent_id(&self) -> Option<NodeId> {
        self.parent_id
    }

    /// Get the Node height in the DOM
    pub fn dom_height(&self) -> u16 {
        self.height
    }
}

/// Manages the lifecycle of your tests.
pub struct TestingHandler {
    vdom: VirtualDom,
    utils: TestUtils,

    event_emitter: EventEmitter,
    event_receiver: EventReceiver,

    events_queue: Vec<FreyaEvent>,
    events_processor: EventsProcessor,
    font_collection: FontCollection,
    viewports: ViewportsCollection,

    config: TestingConfig,
}

impl TestingHandler {
    /// Replace the current [`TestingConfig`].
    pub fn set_config(&mut self, config: TestingConfig) {
        self.config = config;
    }

    /// Wait and apply new changes
    pub async fn wait_for_update(&mut self) -> bool {
        self.wait_for_work(self.config.size());

        let vdom = &mut self.vdom;

        loop {
            let ev = self.event_receiver.try_recv();

            if let Ok(ev) = ev {
                vdom.handle_event(&ev.name, ev.data.any(), ev.element_id, false);
                vdom.process_events();
            } else {
                break;
            }
        }

        timeout(self.config.vdom_timeout(), vdom.wait_for_work())
            .await
            .ok();

        let mutations = self.vdom.render_immediate();

        let (must_repaint, _) = self.utils.dom.get_mut().apply_mutations(mutations);
        self.wait_for_work(self.config.size());
        must_repaint
    }

    /// Wait for layout and events to be processed
    pub fn wait_for_work(&mut self, size: Size2D<f32, f32>) {
        let (layers, viewports) = process_layout(
            &self.utils.dom.get(),
            NodeArea {
                width: size.width,
                height: size.height,
                x: 0.0,
                y: 0.0,
            },
            &mut self.font_collection,
        );

        *self.utils.layers.lock().unwrap() = layers;
        self.viewports = viewports;

        process_events(
            &self.utils.dom.get(),
            &self.utils.layers.lock().unwrap(),
            &mut self.events_queue,
            &self.event_emitter,
            &mut self.events_processor,
            &self.viewports,
        );
    }

    /// Push an event to the events queue
    pub fn push_event(&mut self, event: FreyaEvent) {
        self.events_queue.push(event);
    }

    /// Get the root node
    pub fn root(&mut self) -> TestNode {
        let dom = self.utils.dom.get();
        let rdom = dom.dom();
        let root_id = rdom.root_id();
        let root: &DioxusNode = rdom.get(root_id).unwrap();
        let children_ids = rdom.tree.children_ids(root_id).map(|v| v.to_vec());
        TestNode {
            node_id: root_id,
            utils: self.utils.clone(),
            node: root.clone(),
            height: 0,
            parent_id: None,
            children_ids,
        }
    }
}

/// Run a Component in a headless testing environment
pub fn launch_test(root: Component<()>) -> TestingHandler {
    launch_test_with_config(root, TestingConfig::default())
}

/// Run a Component in a headless testing environment
pub fn launch_test_with_config(root: Component<()>, config: TestingConfig) -> TestingHandler {
    let mut vdom = VirtualDom::new(root);
    let mutations = vdom.rebuild();

    let dom = SafeDOM::new(FreyaDOM::new(RealDom::new()));
    dom.get_mut().init_dom(mutations);

    let (event_emitter, event_receiver) = unbounded_channel::<DomEvent>();
    let layers = Arc::new(Mutex::new(Layers::default()));
    let freya_events = Vec::new();
    let events_processor = EventsProcessor::default();
    let mut font_collection = FontCollection::new();
    font_collection.set_dynamic_font_manager(FontMgr::default());

    TestingHandler {
        vdom,
        events_queue: freya_events,
        events_processor,
        font_collection,
        event_emitter,
        event_receiver,
        viewports: FxHashMap::default(),
        utils: TestUtils { dom, layers },
        config,
    }
}
