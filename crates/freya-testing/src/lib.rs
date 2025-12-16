use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::Write,
    path::PathBuf,
    rc::Rc,
    time::{
        Duration,
        Instant,
    },
};

use freya_clipboard::copypasta::{
    ClipboardContext,
    ClipboardProvider,
};
use freya_components::{
    cache::AssetCacher,
    integration::integration,
};
use freya_core::integration::*;
pub use freya_core::{
    events::platform::*,
    prelude::*,
};
use freya_engine::prelude::{
    EncodedImageFormat,
    FontCollection,
    FontMgr,
    SkData,
    TypefaceFontProvider,
    raster_n32_premul,
};
use ragnarok::{
    CursorPoint,
    EventsExecutorRunner,
    EventsMeasurerRunner,
    NodesState,
};
use torin::prelude::{
    LayoutNode,
    Size2D,
};

pub mod prelude {
    pub use crate::*;
}

pub fn launch_doc(app: impl Into<FpRender>, size: Size2D, path: impl Into<PathBuf>) {
    launch_doc_hook(app, size, path, |_| {})
}

pub fn launch_doc_hook(
    app: impl Into<FpRender>,
    size: Size2D,
    path: impl Into<PathBuf>,
    hook: impl FnOnce(&mut TestingRunner),
) {
    let (mut test, _) = TestingRunner::new(app, size, |_| {});
    hook(&mut test);
    test.render_to_file(path);
}

pub fn launch_test(app: impl Into<FpRender>) -> TestingRunner {
    TestingRunner::new(app, Size2D::new(500., 500.), |_| {}).0
}

pub struct TestingRunner {
    nodes_state: NodesState<NodeId>,
    runner: Runner,
    tree: Rc<RefCell<Tree>>,
    size: Size2D,

    accessibility: AccessibilityTree,

    events_receiver: futures_channel::mpsc::UnboundedReceiver<EventsChunk>,
    events_sender: futures_channel::mpsc::UnboundedSender<EventsChunk>,

    font_manager: FontMgr,
    font_collection: FontCollection,

    platform: Platform,

    ticker_sender: RenderingTickerSender,

    default_fonts: Vec<Cow<'static, str>>,
}

impl TestingRunner {
    pub fn new<T>(
        app: impl Into<FpRender>,
        size: Size2D,
        hook: impl FnOnce(&mut Runner) -> T,
    ) -> (Self, T) {
        let (events_sender, events_receiver) = futures_channel::mpsc::unbounded();
        let app = app.into();
        let mut runner = Runner::new(move || integration(app.clone()).into_element());

        runner.provide_root_context(ScreenReader::new);

        let (mut ticker_sender, ticker) = RenderingTicker::new();
        ticker_sender.set_overflow(true);
        runner.provide_root_context(|| ticker);

        let animation_clock = AnimationClock::new();
        runner.provide_root_context(|| animation_clock.clone());

        runner.provide_root_context(AssetCacher::create);

        let tree = Tree::default();
        let tree = Rc::new(RefCell::new(tree));

        let platform = runner.provide_root_context({
            let tree = tree.clone();
            || Platform {
                focused_accessibility_id: State::create(ACCESSIBILITY_ROOT_ID),
                focused_accessibility_node: State::create(accesskit::Node::new(
                    accesskit::Role::Window,
                )),
                root_size: State::create(size),
                navigation_mode: State::create(NavigationMode::NotKeyboard),
                preferred_theme: State::create(PreferredTheme::Light),
                sender: Rc::new(move |user_event| {
                    match user_event {
                        UserEvent::RequestRedraw => {
                            // Nothing
                        }
                        UserEvent::FocusAccessibilityNode(strategy) => {
                            tree.borrow_mut().accessibility_diff.request_focus(strategy);
                        }
                        UserEvent::SetCursorIcon(_) => {
                            // Nothing
                        }
                        UserEvent::Erased(_) => {
                            // Nothing
                        }
                    }
                }),
            }
        });

        runner.provide_root_context(|| {
            let clipboard: Option<Box<dyn ClipboardProvider>> = ClipboardContext::new()
                .ok()
                .map(|c| Box::new(c) as Box<dyn ClipboardProvider>);

            State::create(clipboard)
        });

        runner.provide_root_context(|| tree.borrow().accessibility_generator.clone());

        let hook_result = hook(&mut runner);

        let mut font_collection = FontCollection::new();
        let def_mgr = FontMgr::default();
        let provider = TypefaceFontProvider::new();
        let font_manager: FontMgr = provider.into();
        font_collection.set_default_font_manager(def_mgr, None);
        font_collection.set_dynamic_font_manager(font_manager.clone());
        font_collection.paragraph_cache_mut().turn_on(false);

        let nodes_state = NodesState::default();
        let accessibility = AccessibilityTree::default();

        let mut runner = Self {
            runner,
            tree,
            size,

            accessibility,
            platform,

            nodes_state,
            events_receiver,
            events_sender,

            font_manager,
            font_collection,

            ticker_sender,

            default_fonts: default_fonts(),
        };

        runner.sync_and_update();

        (runner, hook_result)
    }

    pub fn set_fonts(&mut self, fonts: HashMap<&str, &[u8]>) {
        let mut provider = TypefaceFontProvider::new();
        for (font_name, font_data) in fonts {
            let ft_type = self
                .font_collection
                .fallback_manager()
                .unwrap()
                .new_from_data(font_data, None)
                .unwrap_or_else(|| panic!("Failed to load font {font_name}."));
            provider.register_typeface(ft_type, Some(font_name));
        }
        let font_manager: FontMgr = provider.into();
        self.font_manager = font_manager.clone();
        self.font_collection.set_dynamic_font_manager(font_manager);
    }

    pub fn set_default_fonts(&mut self, fonts: &[Cow<'static, str>]) {
        self.default_fonts.clear();
        self.default_fonts.extend_from_slice(fonts);
        self.tree.borrow_mut().layout.reset();
        self.tree.borrow_mut().text_cache.reset();
        self.tree.borrow_mut().measure_layout(
            self.size,
            &self.font_collection,
            &self.font_manager,
            &self.events_sender,
            1.0,
            &self.default_fonts,
        );
        self.tree.borrow_mut().accessibility_diff.clear();
        self.accessibility.focused_id = ACCESSIBILITY_ROOT_ID;
        self.accessibility.init(&mut self.tree.borrow_mut());
        self.sync_and_update();
    }

    pub async fn handle_events(&mut self) {
        self.runner.handle_events().await
    }

    pub fn handle_events_immediately(&mut self) {
        self.runner.handle_events_immediately()
    }

    pub fn sync_and_update(&mut self) {
        while let Ok(Some(events_chunk)) = self.events_receiver.try_next() {
            match events_chunk {
                EventsChunk::Processed(processed_events) => {
                    let events_executor_adapter = EventsExecutorAdapter {
                        runner: &mut self.runner,
                    };
                    events_executor_adapter.run(&mut self.nodes_state, processed_events);
                }
                EventsChunk::Batch(events) => {
                    for event in events {
                        self.runner.handle_event(
                            event.node_id,
                            event.name,
                            event.data,
                            event.bubbles,
                        );
                    }
                }
            }
        }

        let mutations = self.runner.sync_and_update();
        self.tree.borrow_mut().apply_mutations(mutations);
        self.tree.borrow_mut().measure_layout(
            self.size,
            &self.font_collection,
            &self.font_manager,
            &self.events_sender,
            1.0,
            &self.default_fonts,
        );

        let accessibility_update = self
            .accessibility
            .process_updates(&mut self.tree.borrow_mut(), &self.events_sender);

        self.platform
            .focused_accessibility_id
            .set_if_modified(accessibility_update.focus);
        let node_id = self.accessibility.focused_node_id().unwrap();
        let tree = self.tree.borrow();
        let layout_node = tree.layout.get(&node_id).unwrap();
        self.platform
            .focused_accessibility_node
            .set_if_modified(AccessibilityTree::create_node(node_id, layout_node, &tree));
    }

    /// Poll async tasks and events every `step` time for a total time of `duration`.
    /// This is useful for animations for instance.
    pub fn poll(&mut self, step: Duration, duration: Duration) {
        let started = Instant::now();
        while started.elapsed() < duration {
            self.handle_events_immediately();
            self.sync_and_update();
            std::thread::sleep(step);
            self.ticker_sender.broadcast_blocking(()).unwrap();
        }
    }

    pub fn send_event(&mut self, platform_event: PlatformEvent) {
        let mut events_measurer_adapter = EventsMeasurerAdapter {
            tree: &mut self.tree.borrow_mut(),
            scale_factor: 1.0,
        };
        let processed_events = events_measurer_adapter.run(
            &mut vec![platform_event],
            &mut self.nodes_state,
            self.accessibility.focused_node_id(),
        );
        self.events_sender
            .unbounded_send(EventsChunk::Processed(processed_events))
            .unwrap();
    }

    pub fn move_cursor(&mut self, cursor: impl Into<CursorPoint>) {
        self.send_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseMove,
            cursor: cursor.into(),
            button: Some(MouseButton::Left),
        })
    }

    pub fn write_text(&mut self, text: impl ToString) {
        let text = text.to_string();
        self.send_event(PlatformEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Character(text),
            code: Code::Unidentified,
            modifiers: Modifiers::default(),
        });
        self.sync_and_update();
    }

    pub fn press_key(&mut self, key: Key) {
        self.send_event(PlatformEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key,
            code: Code::Unidentified,
            modifiers: Modifiers::default(),
        });
        self.sync_and_update();
    }

    pub fn press_cursor(&mut self, cursor: impl Into<CursorPoint>) {
        let cursor = cursor.into();
        self.send_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor,
            button: Some(MouseButton::Left),
        });
        self.sync_and_update();
    }

    pub fn release_cursor(&mut self, cursor: impl Into<CursorPoint>) {
        let cursor = cursor.into();
        self.send_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseUp,
            cursor,
            button: Some(MouseButton::Left),
        });
        self.sync_and_update();
    }

    pub fn click_cursor(&mut self, cursor: impl Into<CursorPoint>) {
        let cursor = cursor.into();
        self.send_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor,
            button: Some(MouseButton::Left),
        });
        self.sync_and_update();
        self.send_event(PlatformEvent::Mouse {
            name: MouseEventName::MouseUp,
            cursor,
            button: Some(MouseButton::Left),
        });
        self.sync_and_update();
    }

    pub fn scroll(&mut self, cursor: impl Into<CursorPoint>, scroll: impl Into<CursorPoint>) {
        let cursor = cursor.into();
        let scroll = scroll.into();
        self.send_event(PlatformEvent::Wheel {
            name: WheelEventName::Wheel,
            scroll,
            cursor,
            source: WheelSource::Device,
        });
        self.sync_and_update();
    }

    pub fn render(&mut self) -> SkData {
        let mut surface = raster_n32_premul((self.size.width as i32, self.size.height as i32))
            .expect("Failed to create the surface.");

        let render_pipeline = RenderPipeline {
            font_collection: &mut self.font_collection,
            font_manager: &self.font_manager,
            tree: &self.tree.borrow(),
            canvas: surface.canvas(),
            scale_factor: 1.0,
            background: Color::WHITE,
        };
        render_pipeline.render();

        let image = surface.image_snapshot();
        let mut context = surface.direct_context();
        image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .expect("Failed to encode the snapshot.")
    }

    pub fn render_to_file(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();

        let image = self.render();

        let mut snapshot_file = File::create(path).expect("Failed to create the snapshot file.");

        snapshot_file
            .write_all(&image)
            .expect("Failed to save the snapshot file.");
    }

    pub fn find<T>(
        &self,
        matcher: impl Fn(TestingNode, &dyn ElementExt) -> Option<T>,
    ) -> Option<T> {
        let mut matched = None;
        {
            let tree = self.tree.borrow();
            tree.traverse_depth(|id| {
                if matched.is_some() {
                    return;
                }
                let element = tree.elements.get(&id).unwrap();
                let node = TestingNode {
                    tree: self.tree.clone(),
                    id,
                };
                matched = matcher(node, element.as_ref());
            });
        }

        matched
    }

    pub fn find_many<T>(
        &self,
        matcher: impl Fn(TestingNode, &dyn ElementExt) -> Option<T>,
    ) -> Vec<T> {
        let mut matched = Vec::new();
        {
            let tree = self.tree.borrow();
            tree.traverse_depth(|id| {
                let element = tree.elements.get(&id).unwrap();
                let node = TestingNode {
                    tree: self.tree.clone(),
                    id,
                };
                if let Some(result) = matcher(node, element.as_ref()) {
                    matched.push(result);
                }
            });
        }

        matched
    }
}

pub struct TestingNode {
    tree: Rc<RefCell<Tree>>,
    id: NodeId,
}

impl TestingNode {
    pub fn layout(&self) -> LayoutNode {
        self.tree.borrow().layout.get(&self.id).cloned().unwrap()
    }

    pub fn children(&self) -> Vec<Self> {
        let children = self
            .tree
            .borrow()
            .children
            .get(&self.id)
            .cloned()
            .unwrap_or_default();

        children
            .into_iter()
            .map(|child_id| Self {
                id: child_id,
                tree: self.tree.clone(),
            })
            .collect()
    }

    pub fn is_visible(&self) -> bool {
        let layout = self.layout();
        let effect_state = self
            .tree
            .borrow()
            .effect_state
            .get(&self.id)
            .cloned()
            .unwrap();

        effect_state.is_visible(&self.tree.borrow().layout, &layout.area)
    }

    pub fn element(&self) -> Rc<dyn ElementExt> {
        self.tree
            .borrow()
            .elements
            .get(&self.id)
            .cloned()
            .expect("Element does not exist.")
    }
}
