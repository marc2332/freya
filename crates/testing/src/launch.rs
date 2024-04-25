use dioxus_core::fc_to_builder;
use dioxus_core::Element;
use dioxus_core::VirtualDom;
use dioxus_core_macro::rsx;
use freya_common::EventMessage;
use freya_components::KeyboardNavigator;
use freya_core::prelude::*;
use freya_engine::prelude::*;
use freya_hooks::PlatformInformation;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::{broadcast, watch};
use winit::window::CursorIcon;

use crate::config::TestingConfig;
use crate::test_handler::TestingHandler;
use crate::test_utils::TestUtils;

/// Run a Component in a headless testing environment.
///
/// Default size is `500x500`.
pub fn launch_test(root: AppComponent) -> TestingHandler {
    launch_test_with_config(root, TestingConfig::default())
}

/// Run a Component in a headless testing environment
pub fn launch_test_with_config(root: AppComponent, config: TestingConfig) -> TestingHandler {
    let vdom = with_accessibility(root);
    let fdom = FreyaDOM::default();
    let sdom = SafeDOM::new(fdom);

    let (event_emitter, event_receiver) = unbounded_channel::<DomEvent>();
    let (platform_event_emitter, platform_event_receiver) = unbounded_channel::<EventMessage>();
    let (focus_sender, focus_receiver) = watch::channel(ACCESSIBILITY_ROOT_ID);
    let mut font_collection = FontCollection::new();
    font_collection.set_dynamic_font_manager(FontMgr::default());

    let mut handler = TestingHandler {
        vdom,
        events_queue: EventsQueue::new(),
        nodes_state: NodesState::default(),
        font_collection,
        event_emitter,
        event_receiver,
        utils: TestUtils { sdom },
        config,
        platform_event_emitter,
        platform_event_receiver,
        accessibility_manager: AccessibilityManager::new(ACCESSIBILITY_ROOT_ID).wrap(),
        ticker_sender: broadcast::channel(5).0,
        navigation_state: NavigatorState::new(NavigationMode::NotKeyboard),
        platform_information: Arc::new(Mutex::new(PlatformInformation::new(config.size))),
        cursor_icon: CursorIcon::default(),
        focus_sender,
        focus_receiver,
    };

    handler.init_dom();

    handler
}

fn with_accessibility(app: AppComponent) -> VirtualDom {
    #[derive(Clone)]
    struct RootProps {
        app: AppComponent,
    }

    #[allow(non_snake_case)]
    fn Root(props: RootProps) -> Element {
        #[allow(non_snake_case)]
        let App = props.app;

        rsx!(KeyboardNavigator {
            App {}
        })
    }

    VirtualDom::new_with_props(Root, RootProps { app })
}

type AppComponent = fn() -> Element;
