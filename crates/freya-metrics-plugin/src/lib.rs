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
    Color,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    Rect,
    TextAlign,
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

mod performance;
#[cfg(debug_assertions)]
mod stats;

use performance::{
    FRAME_TIME_SAMPLES,
    WindowMetrics,
    add_text,
    draw_axis_label,
    format_bytes,
};
#[cfg(debug_assertions)]
use stats::draw_stats_overlay;

/// Metrics plugin that displays performance and stats overlays on top of the
/// rendered frame. Both overlays are hidden by default.
pub struct MetricsPlugin {
    performance_enabled: bool,
    performance_toggle_shortcut: (Key, Modifiers),
    #[cfg(debug_assertions)]
    stats_enabled: bool,
    #[cfg(debug_assertions)]
    stats_toggle_shortcut: (Key, Modifiers),
    metrics: HashMap<WindowId, WindowMetrics>,
}

impl Default for MetricsPlugin {
    fn default() -> Self {
        Self {
            performance_enabled: false,
            performance_toggle_shortcut: (
                Key::Character("p".into()),
                Modifiers::ctrl_or_meta() | Modifiers::SHIFT,
            ),
            #[cfg(debug_assertions)]
            stats_enabled: false,
            #[cfg(debug_assertions)]
            stats_toggle_shortcut: (
                Key::Character("l".into()),
                Modifiers::ctrl_or_meta() | Modifiers::SHIFT,
            ),
            metrics: HashMap::new(),
        }
    }
}

impl MetricsPlugin {
    /// Set the keyboard shortcut that toggles the overlay visibility.
    pub fn with_performance_toggle_shortcut(mut self, key: Key, modifiers: Modifiers) -> Self {
        self.performance_toggle_shortcut = (key, modifiers);
        self
    }

    /// Set whether the performance overlay is visible by default.
    pub fn with_visible_performance(mut self, visible: bool) -> Self {
        self.performance_enabled = visible;
        self
    }

    #[cfg(debug_assertions)]
    /// Set whether the stats overlay is visible by default.
    pub fn with_visible_stats(mut self, visible: bool) -> Self {
        self.stats_enabled = visible;
        self
    }

    #[cfg(debug_assertions)]
    /// Set the keyboard shortcut that toggles the stats overlay.
    pub fn with_stats_toggle_shortcut(mut self, key: Key, modifiers: Modifiers) -> Self {
        self.stats_toggle_shortcut = (key, modifiers);
        self
    }

    fn get_metrics(&mut self, id: WindowId) -> &mut WindowMetrics {
        self.metrics.entry(id).or_default()
    }

    #[cfg(debug_assertions)]
    fn overlay_visible(&self) -> bool {
        self.performance_enabled || self.stats_enabled
    }

    #[cfg(not(debug_assertions))]
    fn overlay_visible(&self) -> bool {
        self.performance_enabled
    }
}

impl FreyaPlugin for MetricsPlugin {
    fn plugin_id(&self) -> &'static str {
        "freya-metrics-overlay"
    }

    fn on_event(&mut self, event: &mut PluginEvent, handle: PluginHandle) {
        match event {
            PluginEvent::KeyboardInput {
                key,
                modifiers,
                is_pressed,
                ..
            } => {
                let matches_shortcut = |shortcut: &(Key, Modifiers)| {
                    let key_matches = match (&*key, &shortcut.0) {
                        (Key::Character(a), Key::Character(b)) => a.eq_ignore_ascii_case(b),
                        (a, b) => a == b,
                    };
                    *is_pressed && *modifiers == shortcut.1 && key_matches
                };

                let performance_toggled = matches_shortcut(&self.performance_toggle_shortcut);
                if performance_toggled {
                    self.performance_enabled = !self.performance_enabled;
                }
                #[cfg(debug_assertions)]
                let debug_toggled = matches_shortcut(&self.stats_toggle_shortcut);
                #[cfg(debug_assertions)]
                if debug_toggled {
                    self.stats_enabled = !self.stats_enabled;
                }
                if performance_toggled {
                    for window_id in self.metrics.keys() {
                        handle.send_event_loop_event(NativeEvent::Window(NativeWindowEvent {
                            window_id: *window_id,
                            action: NativeWindowEventAction::User(UserEvent::RequestRedraw),
                        }));
                    }
                }
                #[cfg(debug_assertions)]
                if debug_toggled {
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

                // Reset frame accumulators.
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
                if self.overlay_visible() {
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
                if self.overlay_visible() {
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
                tree: _,
                animation_clock,
                metrics,
            } => {
                let performance_metrics = self.get_metrics(window.id()).performance_metrics();
                tracing::info!(
                    target: "freya::metrics::performance",
                    metrics = ?performance_metrics,
                );
                tracing::info!(
                    target: "freya::metrics::stats",
                    metrics = ?metrics,
                );

                #[cfg(debug_assertions)]
                if !self.performance_enabled && !self.stats_enabled {
                    return;
                }
                #[cfg(not(debug_assertions))]
                if !self.performance_enabled {
                    return;
                }
                canvas.save();
                let scale_factor = window.scale_factor() as f32;
                canvas.scale((scale_factor, scale_factor));

                if self.performance_enabled {
                    let performance_state = self.get_metrics(window.id());
                    let started_render = performance_state.started_render.take().unwrap();

                    let finished_render = started_render.elapsed();
                    let finished_presenting =
                        performance_state.finished_presenting.unwrap_or_default();
                    let finished_layout = performance_state.finished_layout.unwrap_or_default();
                    let finished_tree_updates =
                        performance_state.finished_tree_updates.unwrap_or_default();
                    let tasks_poll_time = performance_state.tasks_poll_time;
                    let events_time = performance_state.events_time;
                    let finished_accessibility_updates = performance_state
                        .finished_accessibility_updates
                        .unwrap_or_default();
                    let overlay_time = performance_state.overlay_time;
                    let overlay_started = Instant::now();

                    let mut fps_paragraph_builder =
                        ParagraphBuilder::new(&ParagraphStyle::default(), *font_collection);
                    add_text(
                        &mut fps_paragraph_builder,
                        format!("{} FPS", performance_state.frames.len()),
                        24.0,
                    );
                    let mut fps_paragraph = fps_paragraph_builder.build();
                    fps_paragraph.layout(235.0);

                    let mut rows = vec![
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
                        ("Scale Factor", format!("{}x", window.scale_factor())),
                        (
                            "Animation clock speed",
                            format!("{}x", animation_clock.speed()),
                        ),
                        ("Renderer", performance_state.graphics_driver.to_string()),
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
                    if let Some((usage, limit)) = metrics.resource_cache {
                        rows.push((
                            "GrContext",
                            format!(
                                "{}/{}",
                                format_bytes(usage).replace(' ', ""),
                                format_bytes(limit).replace(' ', "")
                            ),
                        ));
                    }

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

                    let gpu_paragraph = performance_state.gpu_name.as_ref().map(|gpu_name| {
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

                    let scale_max = performance_state.graph_scale_max();

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
                        &format!("last {} frames", performance_state.frame_times.len()),
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
                    for (index, window) in performance_state.frame_times.windows(2).enumerate() {
                        canvas.draw_line(
                            point(index, window[0]),
                            point(index + 1, window[1]),
                            &line_paint,
                        );
                    }

                    performance_state.overlay_time = overlay_started.elapsed();
                }

                #[cfg(debug_assertions)]
                if self.stats_enabled {
                    draw_stats_overlay(
                        canvas,
                        font_collection,
                        window.scale_factor() as f32,
                        window.inner_size().width,
                        *metrics,
                    );
                }

                canvas.restore();
            }
            _ => {}
        }
    }
}
