use std::{
    cell::RefCell,
    ffi::CString,
    rc::Rc,
};

use freya_clipboard::copypasta::ClipboardProvider;
use freya_components::{
    cache::AssetCacher,
    integration::integration,
};
use freya_core::{
    integration::*,
    prelude::{
        Color,
        CursorIcon,
    },
};
use ragnarok::{
    EventsExecutorRunner,
    EventsMeasurerRunner,
    NodesState,
};
use torin::prelude::Size2D;

use crate::{
    config::WebConfig,
    emscripten::emscripten_run_script,
    events::BrowserState,
    fonts::Fonts,
    surface::WebSurface,
};

pub struct WebApp {
    runner: Runner,
    tree: Tree,
    surface: WebSurface,

    nodes_state: NodesState<NodeId>,
    accessibility: AccessibilityTree,

    events_receiver: futures_channel::mpsc::UnboundedReceiver<EventsChunk>,
    events_sender: futures_channel::mpsc::UnboundedSender<EventsChunk>,

    user_events: Rc<RefCell<Vec<UserEvent>>>,

    fonts: Fonts,

    platform: Platform,
    ticker_sender: RenderingTickerSender,

    background: Color,
    cursor: CursorIcon,
    needs_render: bool,
}

impl WebApp {
    pub fn new(config: WebConfig) -> Option<Self> {
        let (size, pixel_ratio) = BrowserState::sync_canvas_size();

        let surface = WebSurface::new(size.to_i32())?;

        let mut fonts = Fonts::new(config.fonts, config.default_fonts);

        let (events_sender, events_receiver) = futures_channel::mpsc::unbounded();

        let app = config.app;
        let mut runner = Runner::new(move || integration(app.clone()).into_element());

        runner.provide_root_context(ScreenReader::new);
        runner.provide_root_context(GlobalContexts::default);

        let (ticker_sender, ticker) = RenderingTicker::new();
        runner.provide_root_context(|| ticker);
        runner.provide_root_context(AnimationClock::new);
        runner.provide_root_context(AssetCacher::create);

        let user_events = Rc::new(RefCell::new(Vec::new()));

        let platform = runner.provide_root_context({
            let user_events = user_events.clone();
            move || Platform {
                focused_accessibility_id: State::create(ACCESSIBILITY_ROOT_ID),
                focused_accessibility_node: State::create(accesskit::Node::new(
                    accesskit::Role::Window,
                )),
                root_size: State::create(size),
                scale_factor: State::create(pixel_ratio),
                custom_scale_factor: State::create(1.),
                navigation_mode: State::create(NavigationMode::NotKeyboard),
                preferred_theme: State::create(PreferredTheme::Light),
                is_app_focused: State::create(true),
                accent_color: State::create(AccentColor::default()),
                sender: Rc::new(move |user_event| user_events.borrow_mut().push(user_event)),
            }
        });

        let clipboard: Option<Box<dyn ClipboardProvider>> = None;
        runner.provide_root_context(|| State::create(clipboard));

        let mut tree = Tree::default();
        runner.provide_root_context(|| tree.accessibility_generator.clone());
        runner.provide_root_context(|| fonts.collection.clone());

        let mut nodes_state = NodesState::default();

        let mutations = runner.sync_and_update();
        let result = tree.apply_mutations(mutations, pixel_ratio as f32);
        if let Some(strategy) = result.auto_focus {
            tree.accessibility_diff.request_focus(strategy);
        }
        tree.measure_layout(
            size,
            &mut fonts.collection,
            &fonts.manager,
            &events_sender,
            &mut nodes_state,
            pixel_ratio,
            &fonts.default_families,
        );

        BrowserState::listen();

        Some(Self {
            runner,
            tree,
            surface,
            nodes_state,
            accessibility: AccessibilityTree::default(),
            events_receiver,
            events_sender,
            user_events,
            fonts,
            platform,
            ticker_sender,
            background: config.background,
            cursor: CursorIcon::Default,
            needs_render: true,
        })
    }

    fn size(&self) -> Size2D {
        *self.platform.root_size.peek()
    }

    fn scale_factor(&self) -> f64 {
        *self.platform.scale_factor.peek()
    }

    /// Advances the app by one browser frame.
    pub fn frame(&mut self) {
        let browser = BrowserState::take();

        if let Some((size, pixel_ratio)) = browser.resized {
            self.surface.resize(size.to_i32());
            self.platform.root_size.set_if_modified(size);
            self.needs_render = true;

            if (pixel_ratio - self.scale_factor()).abs() > f64::EPSILON {
                self.platform.scale_factor.set_if_modified(pixel_ratio);
                self.tree.layout.reset();
                self.tree.text_cache.reset();
            } else {
                self.tree.layout.clear_dirty();
                self.tree.layout.invalidate(NodeId::ROOT);
            }
        }

        if let Some(focused) = browser.focused {
            self.platform.is_app_focused.set_if_modified(focused);
        }

        let scale_factor = self.scale_factor();
        for event in browser.events {
            let mut events_measurer_adapter = EventsMeasurerAdapter {
                tree: &mut self.tree,
                scale_factor,
            };
            let processed_events = events_measurer_adapter.run(
                &mut vec![event],
                &mut self.nodes_state,
                self.accessibility.focused_node_id(),
            );
            let events_executor_adapter = EventsExecutorAdapter {
                runner: &mut self.runner,
            };
            events_executor_adapter.run(&mut self.nodes_state, processed_events);
        }

        self.runner.handle_events_immediately();
        self.sync();

        self.finish();

        if self.needs_render {
            self.needs_render = false;
            self.render();
        }

        self.ticker_sender.notify();
    }

    /// Runs the pending events and applies the resulting mutations to the tree.
    fn sync(&mut self) {
        while let Ok(events_chunk) = self.events_receiver.try_recv() {
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
        let scale_factor = self.scale_factor() as f32;
        let tree = &mut self.tree;
        let result = self
            .runner
            .run_in(|| tree.apply_mutations(mutations, scale_factor));
        if let Some(strategy) = result.auto_focus {
            self.tree.accessibility_diff.request_focus(strategy);
        }

        self.needs_render |= result.needs_render;
    }

    /// Fulfills the requests made by the app and measures the layout.
    fn finish(&mut self) {
        for user_event in self.user_events.borrow_mut().drain(..) {
            match user_event {
                UserEvent::FocusAccessibilityNode(strategy) => {
                    self.tree.accessibility_diff.request_focus(strategy);
                }
                UserEvent::OpenUrl(url) => {
                    let url = url.replace('\\', "\\\\").replace('\'', "\\'");
                    Self::run_script(&format!("window.open('{url}', '_blank');"));
                }
                UserEvent::RequestRedraw => self.needs_render = true,
                UserEvent::LoadFont {
                    font_name,
                    font_data,
                } => {
                    self.fonts.load(&font_name, &font_data);
                    self.tree.layout.reset();
                    self.tree.text_cache.reset();
                    self.needs_render = true;
                }
                UserEvent::SetCustomScaleFactor(_) | UserEvent::Erased(_) => {}
            }
        }

        let size = self.size();
        let scale_factor = self.scale_factor();
        self.tree.measure_layout(
            size,
            &mut self.fonts.collection,
            &self.fonts.manager,
            &self.events_sender,
            &mut self.nodes_state,
            scale_factor,
            &self.fonts.default_families,
        );

        let update = self
            .accessibility
            .process_updates(&mut self.tree, &self.events_sender, "");
        self.platform
            .focused_accessibility_id
            .set_if_modified(update.focus);
        if let Some(node_id) = self.accessibility.focused_node_id()
            && let Some(layout_node) = self.tree.layout.get(&node_id)
        {
            let focused_node = AccessibilityTree::create_node(node_id, layout_node, &self.tree, "");
            self.platform
                .focused_accessibility_node
                .set_if_modified(focused_node);
        }

        let cursor = self.tree.cursor_icon(&self.nodes_state);
        if cursor != self.cursor {
            self.cursor = cursor;
            Self::run_script(&format!(
                "document.querySelector('#canvas').style.cursor = '{}';",
                cursor.name()
            ));
        }
    }

    fn render(&mut self) {
        let scale_factor = self.scale_factor();
        let render_pipeline = RenderPipeline {
            font_collection: &mut self.fonts.collection,
            font_manager: &self.fonts.manager,
            tree: &self.tree,
            canvas: self.surface.canvas(),
            scale_factor,
            background: self.background,
        };
        render_pipeline.render();

        self.surface.present();
    }

    fn run_script(script: &str) {
        if let Ok(script) = CString::new(script) {
            unsafe { emscripten_run_script(script.as_ptr()) };
        }
    }
}
