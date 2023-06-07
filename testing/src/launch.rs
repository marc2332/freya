use dioxus_core::{Component, VirtualDom};
use freya_core::events::DomEvent;
use freya_core::events::EventsProcessor;
use freya_dom::prelude::{FreyaDOM, SafeDOM};
use freya_layout::Layers;
use rustc_hash::FxHashMap;
use skia_safe::textlayout::FontCollection;
use skia_safe::FontMgr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::unbounded_channel;

pub use freya_core::events::FreyaEvent;
pub use freya_elements::events::mouse::MouseButton;

use crate::test_handler::TestingHandler;
use crate::test_utils::TestUtils;
use crate::{TestingConfig, SCALE_FACTOR};

/// Run a Component in a headless testing environment
pub fn launch_test(root: Component<()>) -> TestingHandler {
    launch_test_with_config(root, TestingConfig::default())
}

/// Run a Component in a headless testing environment
pub fn launch_test_with_config(root: Component<()>, config: TestingConfig) -> TestingHandler {
    let mut vdom = VirtualDom::new(root);
    let mutations = vdom.rebuild();

    let mut fdom = FreyaDOM::default();

    fdom.init_dom(mutations, SCALE_FACTOR as f32);

    let sdom = SafeDOM::new(fdom);

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
        utils: TestUtils { sdom, layers },
        config,
    }
}
