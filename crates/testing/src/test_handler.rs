use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use accesskit::TreeUpdate;
use dioxus_core::{
    Event,
    VirtualDom,
};
use freya_core::prelude::{
    EventMessage,
    TextGroupMeasurement,
    *,
};
use freya_engine::prelude::{
    raster_n32_premul,
    Color,
    Data,
    EncodedImageFormat,
    FontCollection,
    FontMgr,
};
use freya_native_core::{
    dioxus::NodeImmutableDioxusExt,
    prelude::NodeImmutable,
    NodeId,
};
use freya_node_state::AccessibilityNodeState;
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
    torin::Torin,
};
use winit::{
    event::MouseButton,
    window::CursorIcon,
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
    pub(crate) platform_event_emitter: UnboundedSender<EventMessage>,
    pub(crate) platform_event_receiver: UnboundedReceiver<EventMessage>,
    pub(crate) events_queue: EventsQueue,
    pub(crate) nodes_state: NodesState,
    pub(crate) platform_sender: NativePlatformSender,
    pub(crate) platform_receiver: NativePlatformReceiver,
    pub(crate) font_collection: FontCollection,
    pub(crate) font_mgr: FontMgr,
    pub(crate) accessibility_tree: SharedAccessibilityTree,
    pub(crate) config: TestingConfig<T>,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) cursor_icon: CursorIcon,
}

impl<T: 'static + Clone> TestingHandler<T> {
    /// Init the DOM.
    pub(crate) fn init_dom(&mut self) {
        self.provide_vdom_contexts();
        let sdom = self.utils.sdom();
        let mut fdom = sdom.get_mut();
        fdom.init_dom(&mut self.vdom, SCALE_FACTOR as f32);
    }

    /// Get a mutable reference to the current [`TestingConfig`].
    pub fn config(&mut self) -> &mut TestingConfig<T> {
        &mut self.config
    }

    /// Provide some values to the app
    fn provide_vdom_contexts(&mut self) {
        self.vdom
            .insert_any_root_context(Box::new(self.platform_event_emitter.clone()));
        self.vdom
            .insert_any_root_context(Box::new(self.platform_receiver.clone()));
        self.vdom
            .insert_any_root_context(Box::new(Arc::new(self.ticker_sender.subscribe())));
        let accessibility_generator = {
            let sdom = self.sdom();
            let fdom = sdom.get();
            fdom.accessibility_generator().clone()
        };
        self.vdom
            .insert_any_root_context(Box::new(accessibility_generator));

        if let Some(state) = self.config.state.clone() {
            self.vdom.insert_any_root_context(Box::new(state));
        }
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
                        let fdom = self.utils.sdom.get();
                        let rdom = fdom.rdom();
                        let layout = fdom.layout();
                        let res = self
                            .accessibility_tree
                            .lock()
                            .unwrap()
                            .set_focus_with_update(node_id);
                        if let Some((tree, node_id)) = res {
                            self.sync_accessibility(rdom, &layout, node_id, tree)
                        }
                    }
                    EventMessage::FocusNextAccessibilityNode => {
                        let fdom = self.utils.sdom.get();
                        let rdom = fdom.rdom();
                        let layout = fdom.layout();
                        let (tree, node_id) = self
                            .accessibility_tree
                            .lock()
                            .unwrap()
                            .set_focus_on_next_node(AccessibilityFocusStrategy::Forward, rdom);
                        self.sync_accessibility(rdom, &layout, node_id, tree)
                    }
                    EventMessage::FocusPrevAccessibilityNode => {
                        let fdom = self.utils.sdom.get();
                        let rdom = fdom.rdom();
                        let layout = fdom.layout();
                        let (tree, node_id) = self
                            .accessibility_tree
                            .lock()
                            .unwrap()
                            .set_focus_on_next_node(AccessibilityFocusStrategy::Backward, rdom);
                        self.sync_accessibility(rdom, &layout, node_id, tree)
                    }
                    EventMessage::SetCursorIcon(icon) => {
                        self.cursor_icon = icon;
                    }
                    EventMessage::RemeasureTextGroup(text_measurement) => {
                        self.measure_text_group(text_measurement);
                    }
                    _ => {}
                }
            }

            if let Ok(events) = vdom_events {
                let fdom = self.utils.sdom().get();
                let rdom = fdom.rdom();
                for event in events {
                    if let Some(element_id) =
                        rdom.get(event.node_id).and_then(|node| node.mounted_id())
                    {
                        let name = event.name.into();
                        let data = event.data.any();
                        let event = Event::new(data, event.bubbles);
                        self.vdom.runtime().handle_event(name, event, element_id);
                        self.vdom.process_events();
                    }
                }
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
        process_layout(
            &self.utils.sdom().get(),
            Area {
                origin: (0.0, 0.0).into(),
                size,
            },
            &mut self.font_collection,
            SCALE_FACTOR as f32,
            &default_fonts(),
        );

        let fdom = &self.utils.sdom().get_mut();
        {
            let rdom = fdom.rdom();
            let layout = fdom.layout();
            let mut dirty_accessibility_tree = fdom.accessibility_dirty_nodes();
            self.accessibility_tree.lock().unwrap().process_updates(
                rdom,
                &layout,
                &mut dirty_accessibility_tree,
            );
        }

        process_events(
            fdom,
            &mut self.events_queue,
            &self.event_emitter,
            &mut self.nodes_state,
            SCALE_FACTOR,
            self.accessibility_tree.lock().unwrap().focused_node_id(),
        );
    }

    fn measure_text_group(&self, text_measurement: TextGroupMeasurement) {
        let sdom = self.utils.sdom();
        sdom.get()
            .measure_paragraphs(text_measurement, SCALE_FACTOR);
    }

    /// Push an event to the events queue
    ///
    /// ```rust, no_run
    /// # use freya_testing::prelude::*;
    /// # use freya::prelude::*;
    /// # let mut utils = launch_test(|| rsx!( rect { } ));
    /// utils.push_event(TestEvent::Mouse {
    ///     name: EventName::MouseDown,
    ///     cursor: (490., 20.).into(),
    ///     button: Some(MouseButton::Left),
    /// });
    /// ```
    ///
    /// For mouse movements and clicks you can use shorcuts like [Self::move_cursor] and [Self::click_cursor].
    pub fn push_event(&mut self, event: impl Into<PlatformEvent>) {
        self.events_queue.push(event.into());
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
        self.accessibility_tree.lock().unwrap().focused_id
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

    /// Get the current [CursorIcon].
    pub fn cursor_icon(&self) -> CursorIcon {
        self.cursor_icon
    }

    /// Get the [SafeDOM]
    pub fn sdom(&self) -> &SafeDOM {
        self.utils.sdom()
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
            selected_node: None,
            font_collection: &mut self.font_collection,
            font_manager: &self.font_mgr,
            default_fonts: &["Fira Sans".to_string()],
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
        self.push_event(PlatformEvent {
            name: EventName::MouseMove,
            data: PlatformEventData::Mouse {
                cursor: cursor.into(),
                button: Some(MouseButton::Left),
            },
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
        self.push_event(PlatformEvent {
            name: EventName::MouseDown,
            data: PlatformEventData::Mouse {
                cursor: cursor.clone().into(),
                button: Some(MouseButton::Left),
            },
        });
        self.wait_for_update().await;
        self.push_event(PlatformEvent {
            name: EventName::MouseUp,
            data: PlatformEventData::Mouse {
                cursor: cursor.into(),
                button: Some(MouseButton::Left),
            },
        });
        self.wait_for_update().await;
    }

    /// Keep the components on sync with the latest accessibility tree update.
    pub(crate) fn sync_accessibility(
        &self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        node_id: NodeId,
        tree: TreeUpdate,
    ) {
        self.platform_sender.send_modify(|state| {
            state.focused_accessibility_id = tree.focus;
            let node_ref = rdom.get(node_id).unwrap();
            let node_accessibility = node_ref.get::<AccessibilityNodeState>().unwrap();
            let layout_node = layout.get(node_id).unwrap();
            state.focused_accessibility_node =
                AccessibilityTree::create_node(&node_ref, layout_node, &node_accessibility)
        });
    }
}
