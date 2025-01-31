use std::{
    future::Future,
    path::PathBuf,
    sync::{
        Arc,
        Mutex,
    },
};

use accesskit::{
    NodeBuilder,
    Role,
};
use dioxus_core::{
    fc_to_builder,
    Element,
    IntoDynNode,
    VirtualDom,
};
use dioxus_core_macro::{
    component,
    rsx,
    Props,
};
use dioxus_signals::{
    GlobalSignal,
    Readable,
};
use freya_components::NativeContainer;
use freya_core::prelude::{
    EventMessage,
    *,
};
use freya_elements as dioxus_elements;
use freya_engine::prelude::*;
use tokio::{
    runtime::Runtime,
    sync::{
        broadcast,
        mpsc::unbounded_channel,
        watch,
    },
};
use torin::prelude::Size2D;
use winit::window::CursorIcon;

use crate::{
    config::TestingConfig,
    test_handler::TestingHandler,
    test_utils::TestUtils,
    SCALE_FACTOR,
};

/// Run a Component in a headless testing environment.
///
/// ```rust
/// # use freya_testing::prelude::*;
/// # use freya::prelude::*;
/// # let rt = tokio::runtime::Builder::new_current_thread()
/// # .enable_all()
/// # .build()
/// # .unwrap();
/// # let _guard = rt.enter();
/// fn app() -> Element {
///     rsx!(
///         rect {
///             label {
///                 "Hello, World!"
///             }
///         }
///     )
/// }
///
/// # rt.block_on(async move {
/// let mut utils = launch_test(app);
///
/// let root = utils.root();
/// let rect = root.get(0);
/// let label = rect.get(0);
/// let text = label.get(0);
///
/// assert_eq!(text.text(), Some("Hello, World!"));
/// # });
/// ```
pub fn launch_test(root: AppComponent) -> TestingHandler<()> {
    launch_test_with_config(root, TestingConfig::default())
}

/// Run a Component in a headless testing environment
pub fn launch_test_with_config<T: 'static + Clone>(
    root: AppComponent,
    config: TestingConfig<T>,
) -> TestingHandler<T> {
    let vdom = with_accessibility(root);
    let fdom = FreyaDOM::default();
    let sdom = SafeDOM::new(fdom);

    let (event_emitter, event_receiver) = unbounded_channel();
    let (platform_event_emitter, platform_event_receiver) = unbounded_channel::<EventMessage>();
    let (platform_sender, platform_receiver) = watch::channel(NativePlatformState {
        focused_accessibility_id: ACCESSIBILITY_ROOT_ID,
        focused_accessibility_node: NodeBuilder::new(Role::Window).build(),
        preferred_theme: PreferredTheme::default(),
        navigation_mode: NavigationMode::default(),
        information: PlatformInformation::new(config.size, false, false, false),
        scale_factor: SCALE_FACTOR,
    });
    let mut font_collection = FontCollection::new();
    let font_mgr = FontMgr::default();
    font_collection.set_dynamic_font_manager(font_mgr.clone());
    font_collection.set_default_font_manager(font_mgr.clone(), None);

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
        accessibility_tree: Arc::new(Mutex::new(AccessibilityTree::new(ACCESSIBILITY_ROOT_ID))),
        ticker_sender: broadcast::channel(5).0,
        cursor_icon: CursorIcon::default(),
        platform_sender,
        platform_receiver,
    };

    handler.init_dom();
    handler.resize(handler.config.size);

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

#[component]
pub fn Preview(children: Element) -> Element {
    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "fill",
            height: "fill",
            spacing: "8",
            {children}
        }
    )
}

pub fn launch_doc(root: AppComponent, size: Size2D, path: impl Into<PathBuf>) {
    let path: PathBuf = path.into();
    launch_doc_with_utils(root, size, move |mut utils| async move {
        utils.wait_for_update().await;
        utils.save_snapshot(&path);
    });
}

pub fn launch_doc_with_utils<F: Future<Output = ()>>(
    root: AppComponent,
    size: Size2D,
    cb: impl FnOnce(TestingHandler<()>) -> F,
) {
    let mut utils = launch_test(root);
    utils.resize(size);
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        cb(utils).await;
    });
}
