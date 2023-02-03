use std::sync::{Arc, Mutex};

use dioxus_core::{Component, VirtualDom};
use dioxus_native_core::node::NodeType;
use dioxus_native_core::real_dom::RealDom;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::{NodeId, SendAnyMap};
use freya_common::{LayoutMemorizer, NodeArea};
use freya_core::events::EventsProcessor;
use freya_core::{events::DomEvent, process_work, EventEmitter, EventReceiver, SharedFreyaEvents};
use freya_layout::DioxusNode;
use freya_node_state::{CustomAttributeValues, NodeState};
use skia_safe::textlayout::FontCollection;
use skia_safe::FontMgr;
use tokio::sync::mpsc::unbounded_channel;

pub use freya_core::events::FreyaEvent;
pub use freya_elements::MouseButton;

/// Represents a `Node` in the DOM.
#[allow(dead_code)]
pub struct TestNode {
    node_id: NodeId,
    utils: TestUtils,
    height: u16,
    children: Option<Vec<NodeId>>,
    node: DioxusNode,
    parent_id: Option<NodeId>,
}

impl TestNode {
    /// Get a child of the Node by the given index
    pub fn child(&self, child_index: usize) -> Option<Self> {
        if let Some(children) = &self.children {
            let child_id = children.get(child_index)?;
            let child: TestNode = self.utils.get_node_by_id(*child_id);
            Some(child)
        } else {
            None
        }
    }

    /// Get the node's text
    pub fn text(&self) -> Option<&str> {
        if let NodeType::Text { text } = &self.node.node_data.node_type {
            Some(text)
        } else {
            None
        }
    }

    /// Get the node's state
    pub fn state(&self) -> &NodeState {
        &self.node.state
    }

    /// Get the node's layout
    pub fn layout(&self) -> Option<NodeArea> {
        Some(
            self.utils
                .layout_memorizer
                .lock()
                .unwrap()
                .get_node_layout(&self.node_id)?
                .area,
        )
    }
}

/// Collection of utils to test a Freya Component
#[derive(Clone)]
pub struct TestUtils {
    rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    dom: Arc<Mutex<VirtualDom>>,
    layout_memorizer: Arc<Mutex<LayoutMemorizer>>,
    freya_events: SharedFreyaEvents,
    events_processor: Arc<Mutex<EventsProcessor>>,
    font_collection: FontCollection,
    event_emitter: EventEmitter,
    event_receiver: Arc<Mutex<EventReceiver>>,
}

impl TestUtils {
    /// Wait for internal changes
    // TODO Remove this warning
    #[allow(clippy::await_holding_lock)]
    pub async fn wait_for_update(&mut self, sizes: (f32, f32)) {
        self.wait_for_work(sizes).await;

        let ev = self.event_receiver.lock().unwrap().try_recv();

        let mut dom = self.dom.lock().unwrap();

        if let Ok(ev) = ev {
            dom.handle_event(&ev.name, ev.data.any(), ev.element_id, false);
            dom.process_events();
        }

        dom.wait_for_work().await;

        let mutations = dom.render_immediate();

        let (to_update, _) = self.rdom.lock().unwrap().apply_mutations(mutations);

        let mut ctx = SendAnyMap::new();
        ctx.insert(self.layout_memorizer.clone());
        self.rdom.lock().unwrap().update_state(to_update, ctx);
    }

    /// Wait to process the internal Freya changes, like layout or events
    pub async fn wait_for_work(&mut self, sizes: (f32, f32)) {
        process_work::<()>(
            &self.rdom,
            NodeArea {
                width: sizes.0,
                height: sizes.1,
                x: 0.0,
                y: 0.0,
            },
            self.freya_events.clone(),
            &self.event_emitter,
            &mut self.font_collection,
            &mut self.events_processor.lock().unwrap(),
            &self.layout_memorizer,
            &mut (),
            |_, _, _, _, _| {},
        );
    }

    /// Remove any memoization of the DOM layout
    pub fn cleanup_layout(&mut self) {
        self.layout_memorizer.lock().unwrap().dirty_nodes.clear();
        self.layout_memorizer.lock().unwrap().nodes.clear();
    }

    /// Emit an event
    pub fn send_event(&mut self, event: FreyaEvent) {
        self.freya_events.lock().unwrap().push(event);
    }

    pub fn root(&mut self) -> TestNode {
        let rdom = self.rdom.lock().unwrap();
        let root_id = rdom.root_id();
        let root: &DioxusNode = rdom.get(root_id).unwrap();
        let children = rdom.tree.children_ids(root_id).map(|v| v.to_vec());
        TestNode {
            node_id: root_id,
            utils: self.clone(),
            node: root.clone(),
            height: 0,
            parent_id: None,
            children,
        }
    }

    /// Get a Node by the given ID
    pub fn get_node_by_id(&self, node_id: NodeId) -> TestNode {
        let rdom = self.rdom.lock().unwrap();
        let child: &DioxusNode = rdom.get(node_id).unwrap();
        let height = rdom.tree.height(node_id).unwrap();
        let parent_id = rdom.tree.parent_id(node_id);
        let children = rdom.tree.children_ids(node_id).map(|v| v.to_vec());
        TestNode {
            node_id,
            utils: self.clone(),
            node: child.clone(),
            height,
            parent_id,
            children,
        }
    }
}

/// Run a component in a headless testing environment
pub fn launch_test(root: Component<()>) -> TestUtils {
    let mut dom = VirtualDom::new(root);
    let rdom = Arc::new(Mutex::new(
        RealDom::<NodeState, CustomAttributeValues>::new(),
    ));

    let (event_emitter, event_receiver) = unbounded_channel::<DomEvent>();
    let layout_memorizer = Arc::new(Mutex::new(LayoutMemorizer::new()));
    let freya_events = Arc::new(Mutex::new(Vec::new()));
    let events_processor = Arc::new(Mutex::new(EventsProcessor::default()));
    let mut font_collection = FontCollection::new();
    font_collection.set_dynamic_font_manager(FontMgr::default());

    let muts = dom.rebuild();
    let (to_update, _) = rdom.lock().unwrap().apply_mutations(muts);

    let mut ctx = SendAnyMap::new();
    ctx.insert(layout_memorizer.clone());
    rdom.lock().unwrap().update_state(to_update, ctx);

    TestUtils {
        rdom,
        dom: Arc::new(Mutex::new(dom)),
        layout_memorizer,
        freya_events,
        events_processor,
        font_collection,
        event_emitter,
        event_receiver: Arc::new(Mutex::new(event_receiver)),
    }
}
