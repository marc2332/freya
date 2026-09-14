use std::{
    collections::HashMap,
    time::{
        Duration,
        Instant,
    },
};

use freya_core::prelude::{
    ModifiersExt,
    UserEvent,
};
use freya_engine::prelude::{
    Canvas,
    Color,
    FontCollection,
    FontStyle,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    Rect,
    Slant,
    TextAlign,
    TextStyle,
    Weight,
    Width,
};
use freya_winit::{
    plugins::{
        FreyaPlugin,
        Key,
        Modifiers,
        PluginEvent,
        PluginHandle,
    },
    reexports::winit::window::WindowId,
    renderer::{
        NativeEvent,
        NativeWindowEvent,
        NativeWindowEventAction,
    },
};

const FRAME_TIME_SAMPLES: usize = 100;

/// Performance overlay plugin that displays FPS, timing metrics, and other
/// diagnostics on top of the rendered frame. Hidden by default, toggle with
/// Ctrl+Shift+P (Cmd+Shift+P on macOS).
pub struct PerformanceOverlayPlugin {
    enabled: bool,
    toggle_shortcut: (Key, Modifiers),
    metrics: HashMap<WindowId, WindowMetrics>,
}

impl Default for PerformanceOverlayPlugin {
    fn default() -> Self {
        Self {
            enabled: false,
            toggle_shortcut: (
                Key::Character("p".into()),
                Modifiers::ctrl_or_meta() | Modifiers::SHIFT,
            ),
            metrics: HashMap::new(),
        }
    }
}

#[derive(Default)]
struct WindowMetrics {
    graphics_driver: &'static str,
    gpu_name: Option<String>,

    frames: Vec<Instant>,

    started_redraw: Option<Instant>,
    frame_times: Vec<f32>,
    graph_scale_max: f32,
    graph_scale_checked_at: Option<Instant>,

    started_render: Option<Instant>,

    started_layout: Option<Instant>,
    finished_layout: Option<Duration>,

    started_tree_updates: Option<Instant>,
    finished_tree_updates: Option<Duration>,

    started_tasks_poll: Option<Instant>,
    tasks_poll_time: Duration,

    started_events: Option<Instant>,
    events_time: Duration,

    started_accessibility_updates: Option<Instant>,
    finished_accessibility_updates: Option<Duration>,

    started_presenting: Option<Instant>,
    finished_presenting: Option<Duration>,

    overlay_time: Duration,
}

impl WindowMetrics {
    fn record_frame_time(&mut self) {
        let Some(started_redraw) = self.started_redraw.take() else {
            return;
        };
        let frame_time = started_redraw.elapsed().as_secs_f32() * 1000.0;
        self.frame_times.push(frame_time);
        if self.frame_times.len() > FRAME_TIME_SAMPLES {
            self.frame_times.remove(0);
        }
    }

    fn graph_scale_max(&mut self) -> f32 {
        let max_frame_time = self.frame_times.iter().copied().fold(0.0, f32::max);
        let required = nice_scale_max(max_frame_time);

        let due_for_recheck = match self.graph_scale_checked_at {
            Some(checked_at) => checked_at.elapsed() >= Duration::from_secs(1),
            None => true,
        };

        if max_frame_time > self.graph_scale_max || due_for_recheck {
            self.graph_scale_max = required;
            self.graph_scale_checked_at = Some(Instant::now());
        }

        self.graph_scale_max
    }
}

impl PerformanceOverlayPlugin {
    /// Set the keyboard shortcut that toggles the overlay visibility.
    pub fn with_toggle_shortcut(mut self, key: Key, modifiers: Modifiers) -> Self {
        self.toggle_shortcut = (key, modifiers);
        self
    }

    /// Set whether the overlay is visible by default.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.enabled = visible;
        self
    }

    fn get_metrics(&mut self, id: WindowId) -> &mut WindowMetrics {
        self.metrics.entry(id).or_default()
    }
}

impl FreyaPlugin for PerformanceOverlayPlugin {
    fn plugin_id(&self) -> &'static str {
        "freya-performance-overlay"
    }

    fn on_event(&mut self, event: &mut PluginEvent, handle: PluginHandle) {
        match event {
            PluginEvent::KeyboardInput {
                key,
                modifiers,
                is_pressed,
                ..
            } => {
                let (shortcut_key, shortcut_modifiers) = &self.toggle_shortcut;
                let key_matches = match (key, shortcut_key) {
                    (Key::Character(a), Key::Character(b)) => a.eq_ignore_ascii_case(b),
                    (a, b) => a == b,
                };
                if *is_pressed && *modifiers == *shortcut_modifiers && key_matches {
                    self.enabled = !self.enabled;
                    for window_id in self.metrics.keys() {
                        handle.send_event_loop_event(NativeEvent::Window(NativeWindowEvent {
                            window_id: *window_id,
                            action: NativeWindowEventAction::User(UserEvent::RequestRedraw),
                        }));
                    }
                }
            }
            PluginEvent::WindowCreated {
                window,
                graphics_driver,
                gpu_name,
                ..
            }
            | PluginEvent::GraphicsDriverChanged {
                window,
                graphics_driver,
                gpu_name,
            } => {
                let metrics = self.get_metrics(window.id());
                metrics.graphics_driver = graphics_driver;
                metrics.gpu_name = gpu_name.map(str::to_string);
            }
            PluginEvent::WindowClosed { window, .. } => {
                self.metrics.remove(&window.id());
            }
            PluginEvent::AfterRedraw { window, .. } => {
                let metrics = self.get_metrics(window.id());
                let now = Instant::now();

                metrics.record_frame_time();

                metrics
                    .frames
                    .retain(|frame| now.duration_since(*frame).as_millis() < 1000);

                metrics.frames.push(now);

                // Accumulated across the frame, so they need a reset
                metrics.tasks_poll_time = Duration::ZERO;
                metrics.events_time = Duration::ZERO;
                metrics.finished_layout = None;
            }
            PluginEvent::BeforePresenting { window, .. } => {
                self.get_metrics(window.id()).started_presenting = Some(Instant::now())
            }
            PluginEvent::AfterPresenting { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_presenting = Some(metrics.started_presenting.unwrap().elapsed())
            }
            PluginEvent::StartedMeasuringLayout { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.started_redraw.get_or_insert(Instant::now());
                metrics.started_layout = Some(Instant::now());
            }
            PluginEvent::FinishedMeasuringLayout { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_layout = Some(metrics.started_layout.unwrap().elapsed())
            }
            PluginEvent::StartedUpdatingTree { window, .. } => {
                self.get_metrics(window.id()).started_tree_updates = Some(Instant::now())
            }
            PluginEvent::FinishedUpdatingTree { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_tree_updates =
                    Some(metrics.started_tree_updates.unwrap().elapsed())
            }
            PluginEvent::StartedPollingTasks { window, .. } => {
                self.get_metrics(window.id()).started_tasks_poll = Some(Instant::now())
            }
            PluginEvent::FinishedPollingTasks { window, .. } => {
                let metrics = self.get_metrics(window.id());
                if let Some(started) = metrics.started_tasks_poll.take() {
                    metrics.tasks_poll_time += started.elapsed();
                }
                if self.enabled {
                    handle.send_event_loop_event(NativeEvent::Window(NativeWindowEvent {
                        window_id: window.id(),
                        action: NativeWindowEventAction::User(UserEvent::RequestRedraw),
                    }));
                }
            }
            PluginEvent::StartedMeasuringEvents { window, .. } => {
                self.get_metrics(window.id()).started_events = Some(Instant::now())
            }
            PluginEvent::FinishedMeasuringEvents { window, .. } => {
                let metrics = self.get_metrics(window.id());
                if let Some(started) = metrics.started_events.take() {
                    metrics.events_time += started.elapsed();
                }
                if self.enabled {
                    handle.send_event_loop_event(NativeEvent::Window(NativeWindowEvent {
                        window_id: window.id(),
                        action: NativeWindowEventAction::User(UserEvent::RequestRedraw),
                    }));
                }
            }
            PluginEvent::BeforeAccessibility { window, .. } => {
                self.get_metrics(window.id()).started_accessibility_updates = Some(Instant::now())
            }
            PluginEvent::AfterAccessibility { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_accessibility_updates =
                    Some(metrics.started_accessibility_updates.unwrap().elapsed())
            }
            PluginEvent::BeforeRender { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.started_redraw.get_or_insert(Instant::now());
                metrics.started_render = Some(Instant::now());
            }
            PluginEvent::AfterRender {
                window,
                canvas,
                font_collection,
                tree,
                animation_clock,
            } => {
                if !self.enabled {
                    return;
                }
                let metrics = self.get_metrics(window.id());
                let scale_factor = window.scale_factor() as f32;
                let started_render = metrics.started_render.take().unwrap();

                canvas.save();
                canvas.scale((scale_factor, scale_factor));

                let finished_render = started_render.elapsed();
                let finished_presenting = metrics.finished_presenting.unwrap_or_default();
                let finished_layout = metrics.finished_layout.unwrap_or_default();
                let finished_tree_updates = metrics.finished_tree_updates.unwrap_or_default();
                let tasks_poll_time = metrics.tasks_poll_time;
                let events_time = metrics.events_time;
                let finished_accessibility_updates =
                    metrics.finished_accessibility_updates.unwrap_or_default();
                let overlay_time = metrics.overlay_time;
                let overlay_started = Instant::now();

                let mut fps_paragraph_builder =
                    ParagraphBuilder::new(&ParagraphStyle::default(), *font_collection);
                add_text(
                    &mut fps_paragraph_builder,
                    format!("{} FPS", metrics.frames.len()),
                    24.0,
                );
                let mut fps_paragraph = fps_paragraph_builder.build();
                fps_paragraph.layout(235.0);

                let rows = [
                    (
                        "Rendering",
                        format!("{:.3}ms", finished_render.as_secs_f64() * 1000.0),
                    ),
                    (
                        "Presenting",
                        format!("{:.3}ms", finished_presenting.as_secs_f64() * 1000.0),
                    ),
                    (
                        "Layout",
                        format!("{:.3}ms", finished_layout.as_secs_f64() * 1000.0),
                    ),
                    (
                        "Tree Updates",
                        format!("{:.3}ms", finished_tree_updates.as_secs_f64() * 1000.0),
                    ),
                    (
                        "a11y Updates",
                        format!(
                            "{:.3}ms",
                            finished_accessibility_updates.as_secs_f64() * 1000.0
                        ),
                    ),
                    (
                        "Tasks",
                        format!("{:.3}ms", tasks_poll_time.as_secs_f64() * 1000.0),
                    ),
                    (
                        "Events",
                        format!("{:.3}ms", events_time.as_secs_f64() * 1000.0),
                    ),
                    (
                        "Overlay",
                        format!("{:.3}ms", overlay_time.as_secs_f64() * 1000.0),
                    ),
                    (
                        "Frame",
                        format!(
                            "{:.3}ms",
                            (finished_render
                                + finished_presenting
                                + finished_layout
                                + finished_tree_updates
                                + tasks_poll_time
                                + events_time
                                + finished_accessibility_updates
                                + overlay_time)
                                .as_secs_f64()
                                * 1000.0
                        ),
                    ),
                    ("Tree Nodes", tree.size().to_string()),
                    ("Layout Nodes", tree.layout.size().to_string()),
                    ("Scale Factor", format!("{}x", window.scale_factor())),
                    (
                        "Animation clock speed",
                        format!("{}x", animation_clock.speed()),
                    ),
                    ("Renderer", metrics.graphics_driver.to_string()),
                    ("Freya", env!("CARGO_PKG_VERSION").to_string()),
                    (
                        "Build",
                        (if cfg!(debug_assertions) {
                            "Debug"
                        } else {
                            "Release"
                        })
                        .to_string(),
                    ),
                ];

                let mut keys_paragraph_builder =
                    ParagraphBuilder::new(&ParagraphStyle::default(), *font_collection);
                let mut values_style = ParagraphStyle::default();
                values_style.set_text_align(TextAlign::Right);
                let mut values_paragraph_builder =
                    ParagraphBuilder::new(&values_style, *font_collection);
                for (key, value) in &rows {
                    add_text(&mut keys_paragraph_builder, format!("{key}\n"), 14.0);
                    add_text(&mut values_paragraph_builder, format!("{value}\n"), 14.0);
                }
                let mut keys_paragraph = keys_paragraph_builder.build();
                keys_paragraph.layout(235.0);
                let mut values_paragraph = values_paragraph_builder.build();
                values_paragraph.layout(235.0);

                let gpu_paragraph = metrics.gpu_name.as_ref().map(|gpu_name| {
                    let mut builder =
                        ParagraphBuilder::new(&ParagraphStyle::default(), *font_collection);
                    add_text(&mut builder, format!("GPU: {gpu_name}"), 14.0);
                    let mut paragraph = builder.build();
                    paragraph.layout(235.0);
                    paragraph
                });

                let graph_left = 40.0;
                let graph_top = fps_paragraph.height() + 10.0;
                let graph_bottom = graph_top + 60.0;

                let rows_top = graph_bottom + 22.0;
                let gpu_top = rows_top + keys_paragraph.height() + 4.0;
                let content_bottom = gpu_paragraph
                    .as_ref()
                    .map(|paragraph| gpu_top + paragraph.height())
                    .unwrap_or(rows_top + keys_paragraph.height());

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(Color::from_argb(235, 24, 24, 24));
                canvas.draw_rect(Rect::new(5., 5., 245.0, content_bottom + 10.0), &paint);

                fps_paragraph.paint(canvas, (5.0, 0.0));
                keys_paragraph.paint(canvas, (5.0, rows_top));
                values_paragraph.paint(canvas, (5.0, rows_top));
                if let Some(paragraph) = &gpu_paragraph {
                    paragraph.paint(canvas, (5.0, gpu_top));
                }

                let scale_max = metrics.graph_scale_max();

                let mut axis_paint = Paint::default();
                axis_paint.set_anti_alias(true);
                axis_paint.set_style(PaintStyle::Stroke);
                axis_paint.set_stroke_width(1.0);
                axis_paint.set_color(Color::from_rgb(130, 130, 130));

                canvas.draw_line(
                    (graph_left, graph_top),
                    (graph_left, graph_bottom),
                    &axis_paint,
                );
                canvas.draw_line(
                    (graph_left, graph_bottom),
                    (graph_left + 195.0, graph_bottom),
                    &axis_paint,
                );

                let decimals = if scale_max < 10.0 { 1 } else { 0 };
                for (value, y) in [
                    (scale_max, graph_top),
                    (scale_max / 2.0, graph_top + 30.0),
                    (0.0, graph_bottom),
                ] {
                    canvas.draw_line((graph_left - 3.0, y), (graph_left, y), &axis_paint);
                    draw_axis_label(
                        canvas,
                        font_collection,
                        &format!("{value:.decimals$}ms"),
                        7.0,
                        y - 6.0,
                    );
                }

                draw_axis_label(
                    canvas,
                    font_collection,
                    &format!("last {} frames", metrics.frame_times.len()),
                    graph_left + 62.5,
                    graph_bottom + 4.0,
                );

                let mut line_paint = Paint::default();
                line_paint.set_anti_alias(true);
                line_paint.set_style(PaintStyle::Stroke);
                line_paint.set_stroke_width(1.5);
                line_paint.set_color(Color::from_rgb(255, 204, 92));

                let step = 195.0 / (FRAME_TIME_SAMPLES - 1) as f32;
                let point = |index: usize, frame_time: f32| {
                    let x = graph_left + index as f32 * step;
                    let y = graph_bottom - (frame_time / scale_max).min(1.0) * 60.0;
                    (x, y)
                };
                for (index, window) in metrics.frame_times.windows(2).enumerate() {
                    canvas.draw_line(
                        point(index, window[0]),
                        point(index + 1, window[1]),
                        &line_paint,
                    );
                }

                metrics.overlay_time = overlay_started.elapsed();

                canvas.restore();
            }
            _ => {}
        }
    }
}

/// Rounds up to a human-friendly axis ceiling (1/2/5 times a power of ten).
fn nice_scale_max(value: f32) -> f32 {
    if value <= 0.0 {
        return 1.0;
    }
    let magnitude = 10f32.powf(value.log10().floor());
    let fraction = value / magnitude;
    let nice_fraction = if fraction <= 1.0 {
        1.0
    } else if fraction <= 2.0 {
        2.0
    } else if fraction <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * magnitude
}

fn draw_axis_label(canvas: &Canvas, font_collection: &FontCollection, text: &str, x: f32, y: f32) {
    let mut paragraph_builder = ParagraphBuilder::new(&ParagraphStyle::default(), font_collection);
    let mut text_style = TextStyle::default();
    text_style.set_color(Color::from_rgb(170, 170, 170));
    text_style.set_font_size(10.0);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(90.0);
    paragraph.paint(canvas, (x, y));
}

fn add_text(paragraph_builder: &mut ParagraphBuilder, text: String, font_size: f32) {
    let mut text_style = TextStyle::default();
    text_style.set_color(Color::from_rgb(255, 204, 92));
    let font_style = FontStyle::new(Weight::BOLD, Width::NORMAL, Slant::Upright);
    text_style.set_font_style(font_style);
    text_style.set_font_size(font_size);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
}
