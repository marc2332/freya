use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use accesskit::NodeId as AccessibilityId;
use dioxus_core::VirtualDom;
use freya_core::{
    accessibility::AccessibilityTree,
    dom::SafeDOM,
    event_loop_messages::{
        EventLoopMessage,
        EventLoopMessageAction,
    },
    events::{
        EventsExecutorAdapter,
        EventsMeasurerAdapter,
        MouseEventName,
        PlatformEvent,
    },
    layout::process_layout,
    platform::CursorIcon,
    render::{
        Compositor,
        RenderPipeline,
    },
    states::AccessibilityState,
    style::fallback_fonts,
    types::{
        EventEmitter,
        EventReceiver,
        EventsQueue,
        NativePlatformReceiver,
        NativePlatformSender,
    },
    values::Color,
};
use freya_elements::MouseButton;
use freya_engine::prelude::{
    raster_n32_premul,
    Data,
    EncodedImageFormat,
    FontCollection,
    FontMgr,
};
use freya_native_core::{
    prelude::NodeImmutable,
    NodeId,
};
use ragnarok::{
    EventsExecutorRunner,
    EventsMeasurerRunner,
    NodesState,
};
use tokio::{
    sync::{
        broadcast,
        mpsc::{
            UnboundedReceiver,
            UnboundedSender,
        },
    },
    time::{
        interval,
        timeout,
    },
};
use torin::{
    geometry::{
        Area,
        Size2D,
    },
    prelude::CursorPoint,
};

use crate::{
    config::TestingConfig,
    test_node::TestNode,
    test_utils::TestUtils,
    SCALE_FACTOR,
};

/// Manages the lifecycle of your tests.
pub struct TestingHandler<T: 'static + Clone> {
    pub(crate) vdom: VirtualDom,
    pub(crate) utils: TestUtils,
    pub(crate) event_emitter: EventEmitter,
    pub(crate) event_receiver: EventReceiver,
    pub(crate) platform_event_emitter: UnboundedSender<EventLoopMessage>,
    pub(crate) platform_event_receiver: UnboundedReceiver<EventLoopMessage>,
    pub(crate) events_queue: EventsQueue,
    pub(crate) nodes_state: NodesState<NodeId>,
    pub(crate) platform_sender: NativePlatformSender,
    pub(crate) platform_receiver: NativePlatformReceiver,
    pub(crate) font_collection: FontCollection,
    pub(crate) font_mgr: FontMgr,
    pub(crate) accessibility_tree: AccessibilityTree,
    pub(crate) config: TestingConfig<T>,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) cursor_icon: CursorIcon,
}

impl<T: 'static + Clone> TestingHandler<T> {
    /// Sync the RealDOM with the VirtualDOM.
    pub(crate) fn init_doms(&mut self) {
        if let Some(state) = self.config.state.take() {
            self.vdom.insert_any_root_context(Box::new(state));
        }
        self.vdom
            .insert_any_root_context(Box::new(self.platform_event_emitter.clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.platform_receiver.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        self.vdom.insert_any_root_context(Box::new(
            self.utils.sdom.get_mut().accessibility_generator().clone(),
        ));
        self.vdom.insert_any_root_context(Box::new(
            self.utils.sdom.get_mut().animation_clock().clone(),
        ));

        let sdom = self.utils.sdom();
        let mut fdom = sdom.get_mut();
        fdom.init_dom(&mut self.vdom, SCALE_FACTOR as f32);
    }

    /// Get a mutable reference to the current [`TestingConfig`].
    pub fn config(&mut self) -> &mut TestingConfig<T> {
        &mut self.config
    }

    /// Get the current [CursorIcon].
    pub fn cursor_icon(&self) -> CursorIcon {
        self.cursor_icon
    }

    /// Get the [SafeDOM].
    pub fn sdom(&self) -> &SafeDOM {
        self.utils.sdom()
    }

    /// Get the current [AccessibilityId].
    pub fn focus_id(&self) -> AccessibilityId {
        self.accessibility_tree.focused_id
    }

    /// Get the current [AccessibilityId] but as a [TestNode].
    pub fn focus_node(&self) -> TestNode {
        let node_id = self
            .accessibility_tree
            .map
            .get(&self.accessibility_tree.focused_id)
            .unwrap();

        self.utils.get_node_by_id(*node_id)
    }

    /// Apply the latest changes of the virtual dom.
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
            let vdom_events = self.event_receiver.try_recv();

            if vdom_events.is_err() && platform_ev.is_err() {
                break;
            }

            if let Ok(message) = platform_ev {
                match message.action {
                    EventLoopMessageAction::RequestRerender => {
                        if let Some(ticker) = ticker.as_mut() {
                            ticker.tick().await;
                            self.ticker_sender.send(()).unwrap();
                            timeout(self.config.vdom_timeout(), self.vdom.wait_for_work())
                                .await
                                .ok();
                        }
                    }
                    EventLoopMessageAction::FocusAccessibilityNode(strategy) => {
                        let fdom = self.utils.sdom.get();
                        fdom.accessibility_dirty_nodes().request_focus(strategy);
                    }
                    EventLoopMessageAction::SetCursorIcon(icon) => {
                        self.cursor_icon = icon;
                    }
                    EventLoopMessageAction::RemeasureTextGroup(text_measurement) => {
                        let fdom = self.utils.sdom.get();
                        fdom.measure_paragraphs(text_measurement, SCALE_FACTOR);
                    }
                    _ => {}
                }
            }

            if let Ok(processed_events) = vdom_events {
                let sdom = self.utils.sdom();
                let fdom = sdom.get();
                let rdom = fdom.rdom();
                let events_executor_adapter = EventsExecutorAdapter {
                    rdom,
                    vdom: &mut self.vdom,
                };
                events_executor_adapter.run(&mut self.nodes_state, processed_events);
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
    fn wait_for_work(&mut self, size: Size2D) {
        let sdom = &self.utils.sdom();
        let fdom = sdom.get_mut();
        let rdom = fdom.rdom();
        let mut layout = fdom.layout();
        let layers = fdom.layers();
        let mut images_cache = fdom.images_cache();
        let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
        let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();
        let mut compositor_dirty_area = fdom.compositor_dirty_area();

        // Process layout
        process_layout(
            rdom,
            &mut layout,
            &mut images_cache,
            &mut dirty_accessibility_tree,
            &mut compositor_dirty_nodes,
            &mut compositor_dirty_area,
            Area {
                origin: (0.0, 0.0).into(),
                size,
            },
            &mut self.font_collection,
            SCALE_FACTOR as f32,
            &fallback_fonts(),
        );

        // Process accessibility updates
        let (tree, node_id) = self.accessibility_tree.process_updates(
            rdom,
            &layout,
            &mut dirty_accessibility_tree,
            &self.event_emitter,
        );
        // Notify the components
        self.platform_sender.send_modify(|state| {
            state.focused_accessibility_id = tree.focus;
            let node_ref = rdom.get(node_id).unwrap();
            let node_accessibility = node_ref.get::<AccessibilityState>().unwrap();
            let layout_node = layout.get(node_id).unwrap();
            state.focused_accessibility_node =
                AccessibilityTree::create_node(&node_ref, layout_node, &node_accessibility)
        });

        // Process events
        let mut events_measurer_adapter = EventsMeasurerAdapter {
            rdom,
            layers: &layers,
            layout: &layout,
            vdom: &mut self.vdom,
            scale_factor: SCALE_FACTOR,
        };
        let processed_events = events_measurer_adapter.run(
            &mut self.events_queue,
            &mut self.nodes_state,
            self.accessibility_tree.focused_node_id(),
        );
        self.event_emitter.send(processed_events).unwrap();
    }

    /// Push an event to the events queue
    ///
    /// ```rust, no_run
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.push_event(TestEvent::Mouse {
    ///     name: MouseEventName::MouseDown,
    ///     cursor: (490., 20.).into(),
    ///     button: Some(MouseButton::Left),
    /// });
    /// ```
    ///
    /// For mouse **movements** and **clicks** you can use shortcuts like [TestingHandler::move_cursor] and [TestingHandler::click_cursor].
    pub fn push_event(&mut self, event: impl Into<PlatformEvent>) {
        self.events_queue.push(event.into());
    }

    /// Get the Root node.
    pub fn root(&self) -> TestNode {
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

    /// Resize the simulated canvas.
    ///
    /// ```rust, no_run
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.resize((500., 250.).into());
    /// ```
    pub fn resize(&mut self, size: Size2D) {
        self.config.size = size;
        self.platform_sender.send_modify(|state| {
            state.information.viewport_size = size;
        });
        self.utils.sdom().get_mut().layout().reset();
        self.utils
            .sdom()
            .get_mut()
            .compositor_dirty_area()
            .unite_or_insert(&Area::new((0.0, 0.0).into(), size));
    }

    /// Render the app into a canvas and create a snapshot of it.
    ///
    /// ```rust, no_run
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.save_snapshot("./snapshot.png");
    /// ```
    pub fn create_snapshot(&mut self) -> Data {
        let fdom = self.utils.sdom.get();
        let (width, height) = self.config.size.to_i32().to_tuple();

        // Create the main surface
        let mut surface =
            raster_n32_premul((width, height)).expect("Failed to create the surface.");
        surface.canvas().clear(Color::WHITE);

        // Create the dirty surface
        let mut dirty_surface = surface
            .new_surface_with_dimensions((width, height))
            .expect("Failed to create the dirty surface.");
        dirty_surface.canvas().clear(Color::WHITE);

        let mut compositor = Compositor::default();

        // Render to the canvas
        let mut render_pipeline = RenderPipeline {
            canvas_area: Area::from_size((width as f32, height as f32).into()),
            rdom: fdom.rdom(),
            compositor_dirty_area: &mut fdom.compositor_dirty_area(),
            compositor_dirty_nodes: &mut fdom.compositor_dirty_nodes(),
            compositor_cache: &mut fdom.compositor_cache(),
            layers: &mut fdom.layers(),
            layout: &mut fdom.layout(),
            background: Color::WHITE,
            surface: &mut surface,
            dirty_surface: &mut dirty_surface,
            compositor: &mut compositor,
            scale_factor: SCALE_FACTOR as f32,
            font_collection: &mut self.font_collection,
            font_manager: &self.font_mgr,
            fallback_fonts: &["Fira Sans".to_string()],
            images_cache: &mut fdom.images_cache(),
        };
        render_pipeline.run();

        // Capture snapshot
        let image = surface.image_snapshot();
        let mut context = surface.direct_context();
        image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .expect("Failed to encode the snapshot.")
    }

    /// Render the app into a canvas and save it into a file.
    ///
    /// ```rust, no_run
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.save_snapshot("./snapshot.png");
    /// ```
    pub fn save_snapshot(&mut self, snapshot_path: impl Into<PathBuf>) {
        let mut snapshot_file =
            File::create(snapshot_path.into()).expect("Failed to create the snapshot file.");
        let snapshot_data = self.create_snapshot();

        snapshot_file
            .write_all(&snapshot_data)
            .expect("Failed to save the snapshot file.");
    }

    /// Shorthand to simulate a cursor move to the given location.
    ///
    /// ```rust
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.move_cursor((5., 5.));
    /// ```
    pub async fn move_cursor(&mut self, cursor: impl Into<CursorPoint>) {
        self.push_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseMove,
            cursor: cursor.into(),
            button: Some(MouseButton::Left),
        });
        self.wait_for_update().await;
    }

    /// Shorthand to simulate a click with cursor in the given location.
    ///
    /// ```rust
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.click_cursor((5., 5.));
    /// ```
    pub async fn click_cursor(&mut self, cursor: impl Into<CursorPoint> + Clone) {
        self.push_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor: cursor.clone().into(),
            button: Some(MouseButton::Left),
        });
        self.wait_for_update().await;
        self.push_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseUp,
            cursor: cursor.into(),
            button: Some(MouseButton::Left),
        });
        self.wait_for_update().await;
    }
}
