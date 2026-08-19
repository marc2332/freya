use std::{
    borrow::Cow,
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
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
    TypefaceFontProvider,
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
    events::{
        listen,
        sync_canvas_size,
        take_browser_state,
    },
    surface::WebSurface,
};

/// Requests made by the app that the browser has to fulfill.
#[derive(Default)]
struct Requests {
    focus_strategy: Option<AccessibilityFocusStrategy>,
    cursor: Option<CursorIcon>,
    url: Option<String>,
    redraw: bool,
}

pub struct WebApp {
    runner: Runner,
    tree: Tree,
    surface: WebSurface,

    nodes_state: NodesState<NodeId>,
    accessibility: AccessibilityTree,

    events_receiver: futures_channel::mpsc::UnboundedReceiver<EventsChunk>,
    events_sender: futures_channel::mpsc::UnboundedSender<EventsChunk>,

    requests: Rc<RefCell<Requests>>,

    font_manager: FontMgr,
    font_collection: FontCollection,
    default_fonts: Vec<Cow<'static, str>>,

    platform: Platform,
    ticker_sender: RenderingTickerSender,

    background: Color,
    cursor: CursorIcon,
    needs_render: bool,

    /// Scratch buffer for the events measured this frame.
    pending_events: Vec<PlatformEvent>,

    /// Cached copies of the matching `Platform` states.
    size: Size2D,
    scale_factor: f64,
}

impl WebApp {
    pub fn new(config: WebConfig) -> Option<Self> {
        let (width, height, pixel_ratio) = sync_canvas_size();
        let size = Size2D::new(width as f32, height as f32);

        let surface = WebSurface::new(width, height)?;

        let (font_manager, mut font_collection, registered_fonts) = create_fonts(&config.fonts);
        let default_fonts = if config.default_fonts.is_empty() {
            registered_fonts
        } else {
            config.default_fonts
        };

        let (events_sender, events_receiver) = futures_channel::mpsc::unbounded();

        let app = config.app;
        let mut runner = Runner::new(move || integration(app.clone()).into_element());

        runner.provide_root_context(ScreenReader::new);

        let (ticker_sender, ticker) = RenderingTicker::new();
        runner.provide_root_context(|| ticker);
        runner.provide_root_context(AnimationClock::new);
        runner.provide_root_context(AssetCacher::create);

        let requests = Rc::new(RefCell::new(Requests::default()));

        let platform = runner.provide_root_context({
            let requests = requests.clone();
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
                sender: Rc::new(move |user_event| match user_event {
                    UserEvent::FocusAccessibilityNode(strategy) => {
                        requests.borrow_mut().focus_strategy = Some(strategy);
                    }
                    UserEvent::SetCursorIcon(cursor) => {
                        requests.borrow_mut().cursor = Some(cursor);
                    }
                    UserEvent::OpenUrl(url) => {
                        requests.borrow_mut().url = Some(url);
                    }
                    UserEvent::RequestRedraw => {
                        requests.borrow_mut().redraw = true;
                    }
                    UserEvent::SetCustomScaleFactor(_) | UserEvent::Erased(_) => {}
                }),
            }
        });

        let clipboard: Option<Box<dyn ClipboardProvider>> = None;
        runner.provide_root_context(|| State::create(clipboard));

        let mut tree = Tree::default();
        runner.provide_root_context(|| tree.accessibility_generator.clone());
        runner.provide_root_context(|| font_collection.clone());

        let mutations = runner.sync_and_update();
        let result = tree.apply_mutations(mutations);
        if let Some(strategy) = result.auto_focus {
            tree.accessibility_diff.request_focus(strategy);
        }
        tree.measure_layout(
            size,
            &mut font_collection,
            &font_manager,
            &events_sender,
            pixel_ratio,
            &default_fonts,
        );

        listen();

        Some(Self {
            runner,
            tree,
            surface,
            nodes_state: NodesState::default(),
            accessibility: AccessibilityTree::default(),
            events_receiver,
            events_sender,
            requests,
            font_manager,
            font_collection,
            default_fonts,
            platform,
            ticker_sender,
            background: config.background,
            cursor: CursorIcon::Default,
            needs_render: true,
            pending_events: Vec::new(),
            size,
            scale_factor: pixel_ratio,
        })
    }

    pub fn frame(&mut self) {
        let mut browser = take_browser_state();

        if let Some((width, height, pixel_ratio)) = browser.resized {
            self.surface.resize(width, height);
            self.size = Size2D::new(width as f32, height as f32);
            self.platform.root_size.set_if_modified(self.size);
            self.needs_render = true;

            if (pixel_ratio - self.scale_factor).abs() > f64::EPSILON {
                self.scale_factor = pixel_ratio;
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

        for event in browser.events.drain(..) {
            self.pending_events.push(event);
            let mut events_measurer_adapter = EventsMeasurerAdapter {
                tree: &mut self.tree,
                scale_factor: self.scale_factor,
            };
            let processed_events = events_measurer_adapter.run(
                &mut self.pending_events,
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


        self.ticker_sender.send(()).ok();
    }

    /// Applies whatever the runner has pending.
    fn sync(&mut self) {
        if let Some(strategy) = self.requests.borrow_mut().focus_strategy.take() {
            self.tree.accessibility_diff.request_focus(strategy);
        }

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
        let tree = &mut self.tree;
        let result = self.runner.run_in(|| tree.apply_mutations(mutations));
        if let Some(strategy) = result.auto_focus {
            self.tree.accessibility_diff.request_focus(strategy);
        }

        self.needs_render |= result.needs_render;
    }

    fn finish(&mut self) {
        self.tree.measure_layout(
            self.size,
            &mut self.font_collection,
            &self.font_manager,
            &self.events_sender,
            self.scale_factor,
            &self.default_fonts,
        );

        let update = self
            .accessibility
            .process_updates(&mut self.tree, &self.events_sender);
        self.platform
            .focused_accessibility_id
            .set_if_modified(update.focus);
        if let Some(node_id) = self.accessibility.focused_node_id()
            && let Some(layout_node) = self.tree.layout.get(&node_id)
        {
            let focused_node = AccessibilityTree::create_node(node_id, layout_node, &self.tree);
            self.platform
                .focused_accessibility_node
                .set_if_modified(focused_node);
        }

        let requests = &mut *self.requests.borrow_mut();
        self.needs_render |= std::mem::take(&mut requests.redraw);

        if let Some(cursor) = requests.cursor.take()
            && cursor != self.cursor
        {
            self.cursor = cursor;
            run_script(&format!(
                "document.querySelector('#canvas').style.cursor = '{}';",
                cursor.name()
            ));
        }

        if let Some(url) = requests.url.take() {
            run_script(&format!("window.open('{}', '_blank');", escape_js(&url)));
        }
    }

    fn render(&mut self) {
        let render_pipeline = RenderPipeline {
            font_collection: &mut self.font_collection,
            font_manager: &self.font_manager,
            tree: &self.tree,
            canvas: self.surface.canvas(),
            scale_factor: self.scale_factor,
            background: self.background,
        };
        render_pipeline.render();

        self.surface.present();
    }
}

fn create_fonts(fonts: &[(String, Vec<u8>)]) -> (FontMgr, FontCollection, Vec<Cow<'static, str>>) {
    let system_manager = FontMgr::default();
    let mut provider = TypefaceFontProvider::new();
    let mut registered = Vec::new();

    for (name, data) in fonts {
        let Some(typeface) = system_manager.new_from_data(data, None) else {
            tracing::error!("Failed to load the font {name}.");
            continue;
        };
        provider.register_typeface(typeface, Some(name.as_str()));
        registered.push(Cow::Owned(name.clone()));
    }

    let font_manager: FontMgr = provider.into();
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(font_manager.clone(), None);
    font_collection.set_dynamic_font_manager(font_manager.clone());
    font_collection.paragraph_cache_mut().turn_on(false);

    (font_manager, font_collection, registered)
}

fn escape_js(text: &str) -> String {
    text.replace('\\', "\\\\").replace('\'', "\\'")
}

fn run_script(script: &str) {
    if let Ok(script) = CString::new(script) {
        unsafe { emscripten_run_script(script.as_ptr()) };
    }
}
