use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use dioxus_core::VirtualDom;
use freya_common::{
    Compositor,
    EventMessage,
    TextGroupMeasurement,
};
use freya_core::prelude::*;
use freya_engine::prelude::{
    raster_n32_premul,
    Color,
    EncodedImageFormat,
    FontCollection,
    FontMgr,
};
use freya_native_core::dioxus::NodeImmutableDioxusExt;
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
use torin::geometry::{
    Area,
    Size2D,
};
use winit::window::CursorIcon;

use crate::{
    config::TestingConfig,
    test_node::TestNode,
    test_utils::TestUtils,
    SCALE_FACTOR,
};

/// Manages the lifecycle of your tests.
pub struct TestingHandler {
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
    pub(crate) accessibility_manager: SharedAccessibilityManager,
    pub(crate) config: TestingConfig,
    pub(crate) ticker_sender: broadcast::Sender<()>,
    pub(crate) cursor_icon: CursorIcon,
}

impl TestingHandler {
    /// Init the DOM.
    pub(crate) fn init_dom(&mut self) {
        self.provide_vdom_contexts();
        let sdom = self.utils.sdom();
        let mut fdom = sdom.get();
        fdom.init_dom(&mut self.vdom, SCALE_FACTOR as f32);
    }

    /// Get a mutable reference to the current [`TestingConfig`].
    pub fn config(&mut self) -> &mut TestingConfig {
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
    }

    /// Wait and apply new changes
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
                        let tree = self
                            .accessibility_manager
                            .lock()
                            .unwrap()
                            .set_focus_with_update(node_id);

                        if let Some(tree) = tree {
                            self.platform_sender.send_modify(|state| {
                                state.focused_id = tree.focus;
                            });
                        }
                    }
                    EventMessage::FocusNextAccessibilityNode => {
                        let tree = self
                            .accessibility_manager
                            .lock()
                            .unwrap()
                            .set_focus_on_next_node(AccessibilityFocusDirection::Forward);
                        self.platform_sender.send_modify(|state| {
                            state.focused_id = tree.focus;
                        });
                    }
                    EventMessage::FocusPrevAccessibilityNode => {
                        let tree = self
                            .accessibility_manager
                            .lock()
                            .unwrap()
                            .set_focus_on_next_node(AccessibilityFocusDirection::Backward);
                        self.platform_sender.send_modify(|state| {
                            state.focused_id = tree.focus;
                        });
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
                        self.vdom
                            .handle_event(name, data, element_id, event.bubbles);
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
    pub fn wait_for_work(&mut self, size: Size2D) {
        // Clear cached results
        self.utils.sdom().get_mut().layout().reset();

        // Measure layout
        process_layout(
            &self.utils.sdom().get(),
            Area {
                origin: (0.0, 0.0).into(),
                size,
            },
            &mut self.font_collection,
            SCALE_FACTOR,
            &default_fonts(),
        );

        let fdom = &self.utils.sdom().get_mut();

        process_accessibility(
            &fdom.layout(),
            fdom.rdom(),
            &mut self.accessibility_manager.lock().unwrap(),
        );

        process_events(
            fdom,
            &mut self.events_queue,
            &self.event_emitter,
            &mut self.nodes_state,
            SCALE_FACTOR,
        );
    }

    fn measure_text_group(&self, text_measurement: TextGroupMeasurement) {
        let sdom = self.utils.sdom();
        sdom.get()
            .measure_paragraphs(text_measurement, SCALE_FACTOR);
    }

    /// Push an event to the events queue
    pub fn push_event(&mut self, event: PlatformEvent) {
        self.events_queue.push(event);
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
        self.accessibility_manager.lock().unwrap().focused_id
    }

    /// Resize the simulated canvas.
    pub fn resize(&mut self, size: Size2D) {
        self.config.size = size;
        self.platform_sender.send_modify(|state| {
            state.information.viewport_size = size;
        })
    }

    /// Get the current [CursorIcon].
    pub fn cursor_icon(&self) -> CursorIcon {
        self.cursor_icon
    }

    /// Get the [SafeDOM]
    pub fn sdom(&self) -> &SafeDOM {
        self.utils.sdom()
    }

    /// Render the app into a canvas and save it into a file.
    pub fn save_snapshot(&mut self, snapshot_path: impl Into<PathBuf>) {
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

        let mut skia_renderer = SkiaRenderer {
            canvas_area: Area::from_size((width as f32, height as f32).into()),
            font_collection: &mut self.font_collection,
            font_manager: &self.font_mgr,
            matrices: Vec::default(),
            opacities: Vec::default(),
            default_fonts: &["Fira Sans".to_string()],
            scale_factor: SCALE_FACTOR as f32,
        };

        // Render to the canvas
        process_render(
            &fdom,
            &mut surface,
            &mut dirty_surface,
            &mut compositor,
            |fdom, node_id, layout_node, layout, canvas| {
                if let Some(dioxus_node) = fdom.rdom().get(*node_id) {
                    skia_renderer.render(
                        fdom.rdom(),
                        layout_node,
                        &dioxus_node,
                        false,
                        layout,
                        canvas,
                    );
                }
            },
        );

        // Capture snapshot
        let image = surface.image_snapshot();
        let mut context = surface.direct_context();
        let snapshot_data = image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .expect("Failed to encode the snapshot.");

        // Save snapshot
        let mut snapshot_file =
            File::create(snapshot_path.into()).expect("Failed to create the snapshot file.");
        let snapshot_bytes = snapshot_data.as_bytes();
        snapshot_file
            .write_all(snapshot_bytes)
            .expect("Failed to save the snapshot file.");
    }
}
