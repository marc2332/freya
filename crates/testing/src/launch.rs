use dioxus_core::{
    fc_to_builder,
    Element,
    VirtualDom,
};
use dioxus_core_macro::rsx;
use freya_common::EventMessage;
use freya_components::NativeContainer;
use freya_core::prelude::*;
use freya_engine::prelude::*;
use tokio::sync::{
    broadcast,
    mpsc::unbounded_channel,
    watch,
};
use winit::window::CursorIcon;

use crate::{
    config::TestingConfig,
    test_handler::TestingHandler,
    test_utils::TestUtils,
    SCALE_FACTOR,
};

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

    let (event_emitter, event_receiver) = unbounded_channel();
    let (platform_event_emitter, platform_event_receiver) = unbounded_channel::<EventMessage>();
    let (platform_sender, platform_receiver) = watch::channel(NativePlatformState {
        focused_id: ACCESSIBILITY_ROOT_ID,
        preferred_theme: PreferredTheme::default(),
        navigation_mode: NavigationMode::default(),
        information: PlatformInformation::new(config.size, false, false, false),
        scale_factor: SCALE_FACTOR as f32,
    });
    let mut font_collection = FontCollection::new();
    let font_mgr = FontMgr::default();
    font_collection.set_dynamic_font_manager(font_mgr.clone());
    font_collection.set_default_font_manager(font_mgr.clone(), "Fira Sans");

    let mut handler = TestingHandler {
        vdom,
        events_queue: EventsQueue::new(),
        nodes_state: NodesState::default(),
        font_collection,
        font_mgr,
        event_emitter,
        event_receiver,
        utils: TestUtils { sdom },
        config,
        platform_event_emitter,
        platform_event_receiver,
        accessibility_manager: AccessibilityManager::new(ACCESSIBILITY_ROOT_ID).wrap(),
        ticker_sender: broadcast::channel(5).0,
        cursor_icon: CursorIcon::default(),
        platform_sender,
        platform_receiver,
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

        rsx!(NativeContainer {
            App {}
        })
    }

    VirtualDom::new_with_props(Root, RootProps { app })
}

type AppComponent = fn() -> Element;
