use std::sync::{Arc, Mutex};

use dioxus_core::{Component, VirtualDom};
use dioxus_native_core::node::{Node, NodeType};
use dioxus_native_core::real_dom::RealDom;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::{NodeId, SendAnyMap};
use freya_common::{LayoutMemorizer, NodeArea};
use freya_layers::DOMNode;
use freya_node_state::{CustomAttributeValues, NodeState};
use freya_processor::events::{EventsProcessor, FreyaEvent};
use freya_processor::{process_work, DomEvent, EventEmitter, EventReceiver, SafeFreyaEvents};
use skia_safe::textlayout::FontCollection;
use tokio::sync::mpsc::unbounded_channel;

pub struct TestNode {
    node_id: NodeId,
    utils: TestUtils,
    state: NodeState,
    node_info: DOMNode,
}

impl TestNode {
    /// Get a child of the Node by the given index
    pub fn child(&self, child_index: usize) -> Option<Self> {
        if let Some(children) = &self.node_info.children {
            let child_id = children.get(child_index)?;
            let child: TestNode = self.utils.get_node_by_id(*child_id);
            Some(child)
        } else {
            None
        }
    }

    /// Get the node's text
    pub fn text(&self) -> Option<&str> {
        if let NodeType::Text { text } = &self.node_info.node.node_data.node_type {
            Some(text)
        } else {
            None
        }
    }

    /// Get the node's state
    pub fn state(&self) -> &NodeState {
        &self.state
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
    freya_events: SafeFreyaEvents,
    events_processor: Arc<Mutex<EventsProcessor>>,
    font_collection: FontCollection,
    event_emitter: EventEmitter,
    event_receiver: Arc<Mutex<EventReceiver>>,
}

impl TestUtils {
    /// Wait for internal changes
    // Haven't found a way around this yet
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

    pub fn send_event(&mut self, event: FreyaEvent) {
        self.freya_events.lock().unwrap().push(event);
    }

    pub fn root(&mut self) -> TestNode {
        let rdom = self.rdom.lock().unwrap();
        let root_id = rdom.root_id();
        let root: &Node<NodeState, CustomAttributeValues> = rdom.get(root_id).unwrap();
        let children = rdom.tree.children_ids(root_id).map(|v| v.to_vec());
        TestNode {
            node_id: root_id,
            utils: self.clone(),
            state: root.state.clone(),
            node_info: DOMNode {
                node: root.clone(),
                height: 0,
                parent_id: None,
                children,
            },
        }
    }

    /// Get a Node by the given ID
    pub fn get_node_by_id(&self, node_id: NodeId) -> TestNode {
        let rdom = self.rdom.lock().unwrap();
        let child: &Node<NodeState, CustomAttributeValues> = rdom.get(node_id).unwrap();
        let height = rdom.tree.height(node_id).unwrap();
        let parent_id = rdom.tree.parent_id(node_id);
        let children = rdom.tree.children_ids(node_id).map(|v| v.to_vec());
        TestNode {
            node_id,
            utils: self.clone(),
            state: child.state.clone(),
            node_info: DOMNode {
                node: child.clone(),
                height,
                parent_id,
                children,
            },
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
    let font_collection = FontCollection::new();

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
