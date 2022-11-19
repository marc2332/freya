use std::sync::{Arc, Mutex};

use anymap::AnyMap;
use dioxus_core::{Component, ElementId, VirtualDom};
use dioxus_native_core::real_dom::{Node, NodeType, RealDom};
use dioxus_native_core::traversable::Traversable;
use freya_common::{LayoutMemorizer, NodeArea};
use freya_node_state::NodeState;
use freya_processor::events::{EventsProcessor, FreyaEvent};
use freya_processor::{process_work, SafeEventEmitter, SafeFreyaEvents};
use skia_safe::textlayout::FontCollection;

pub struct TestNode {
    #[allow(dead_code)]
    id: ElementId,
    utils: TestUtils,
    state: NodeState,
    node_type: NodeType,
}

impl TestNode {
    /// Get a child of the Node by the given index
    pub fn child(&self, child_index: usize) -> Option<Self> {
        if let NodeType::Element { children, .. } = &self.node_type {
            let child_id = children.get(child_index)?;
            let child: TestNode = self.utils.get_node_by_id(*child_id);
            Some(child)
        } else {
            None
        }
    }

    /// Get the node's text
    pub fn text(&self) -> Option<&str> {
        if let NodeType::Text { text } = &self.node_type {
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
                .get_node_layout(&self.id)?
                .area,
        )
    }
}

/// Collection of utils to test a freya Component
#[derive(Clone)]
pub struct TestUtils {
    rdom: Arc<Mutex<RealDom<NodeState>>>,
    dom: Arc<Mutex<VirtualDom>>,
    layout_memorizer: Arc<Mutex<LayoutMemorizer>>,
    freya_events: SafeFreyaEvents,
    event_emitter: SafeEventEmitter,
    events_processor: Arc<Mutex<EventsProcessor>>,
    font_collection: FontCollection,
}

impl TestUtils {
    /// Wait for internal changes
    // Haven't found a way around this yet
    #[allow(clippy::await_holding_lock)]
    pub async fn wait_for_update(&mut self, sizes: (f32, f32)) {
        self.wait_for_work(sizes).await;

        let mut dom = self.dom.lock().unwrap();
        dom.wait_for_work().await;

        let mutations = dom.work_with_deadline(|| false);

        let to_update = self.rdom.lock().unwrap().apply_mutations(mutations);

        let mut ctx = AnyMap::new();

        ctx.insert(self.layout_memorizer.clone());

        if !to_update.is_empty() {
            self.rdom.lock().unwrap().update_state(&dom, to_update, ctx);
        }
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
        let root: &Node<NodeState> = rdom.get(ElementId(root_id)).unwrap();
        TestNode {
            id: ElementId(root_id),
            utils: self.clone(),
            state: root.state.clone(),
            node_type: root.node_type.clone(),
        }
    }

    /// Get a Node by the given ID
    pub fn get_node_by_id(&self, id: ElementId) -> TestNode {
        let rdom = self.rdom.lock().unwrap();
        let child: &Node<NodeState> = rdom.get(id).unwrap();
        TestNode {
            id: child.id,
            utils: self.clone(),
            state: child.state.clone(),
            node_type: child.node_type.clone(),
        }
    }
}

/// Run a component in a headless testing environment
pub fn launch_test(root: Component<()>) -> TestUtils {
    let mut dom = VirtualDom::new(root);
    let rdom = Arc::new(Mutex::new(RealDom::<NodeState>::new()));

    let event_emitter: SafeEventEmitter = Arc::default();
    let layout_memorizer = Arc::new(Mutex::new(LayoutMemorizer::new()));
    let freya_events = Arc::new(Mutex::new(Vec::new()));
    let events_processor = Arc::new(Mutex::new(EventsProcessor::default()));
    let font_collection = FontCollection::new();

    let muts = dom.rebuild();
    let to_update = rdom.lock().unwrap().apply_mutations(vec![muts]);
    let mut ctx = AnyMap::new();

    ctx.insert(layout_memorizer.clone());

    rdom.lock().unwrap().update_state(&dom, to_update, ctx);

    event_emitter
        .lock()
        .unwrap()
        .replace(dom.get_scheduler_channel());

    TestUtils {
        rdom,
        dom: Arc::new(Mutex::new(dom)),
        layout_memorizer,
        event_emitter,
        freya_events,
        events_processor,
        font_collection,
    }
}
