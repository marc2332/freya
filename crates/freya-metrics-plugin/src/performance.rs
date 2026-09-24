use std::{
    fmt,
    time::{
        Duration,
        Instant,
    },
};

use freya_engine::prelude::{
    Canvas,
    Color,
    FontCollection,
    FontStyle,
    ParagraphBuilder,
    ParagraphStyle,
    Slant,
    TextStyle,
    Weight,
    Width,
};

pub(crate) const FRAME_TIME_SAMPLES: usize = 100;

pub(crate) struct PerformanceMetrics {
    pub(crate) fps: usize,
    pub(crate) rendering_ms: f64,
    pub(crate) presenting_ms: f64,
    pub(crate) layout_ms: f64,
    pub(crate) tree_updates_ms: f64,
    pub(crate) accessibility_updates_ms: f64,
    pub(crate) tasks_ms: f64,
    pub(crate) events_ms: f64,
    pub(crate) overlay_ms: f64,
    pub(crate) frame_ms: f64,
}

impl fmt::Debug for PerformanceMetrics {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PerformanceMetrics")
            .field("fps", &self.fps)
            .field("rendering_ms", &self.rendering_ms)
            .field("presenting_ms", &self.presenting_ms)
            .field("layout_ms", &self.layout_ms)
            .field("tree_updates_ms", &self.tree_updates_ms)
            .field("accessibility_updates_ms", &self.accessibility_updates_ms)
            .field("tasks_ms", &self.tasks_ms)
            .field("events_ms", &self.events_ms)
            .field("overlay_ms", &self.overlay_ms)
            .field("frame_ms", &self.frame_ms)
            .finish()
    }
}

#[derive(Default)]
pub(crate) struct WindowMetrics {
    pub(crate) graphics_driver: &'static str,
    pub(crate) gpu_name: Option<String>,
    pub(crate) frames: Vec<Instant>,
    pub(crate) started_redraw: Option<Instant>,
    pub(crate) frame_times: Vec<f32>,
    pub(crate) graph_scale_max: f32,
    pub(crate) graph_scale_checked_at: Option<Instant>,
    pub(crate) started_render: Option<Instant>,
    pub(crate) started_layout: Option<Instant>,
    pub(crate) finished_layout: Option<Duration>,
    pub(crate) started_tree_updates: Option<Instant>,
    pub(crate) finished_tree_updates: Option<Duration>,
    pub(crate) started_tasks_poll: Option<Instant>,
    pub(crate) tasks_poll_time: Duration,
    pub(crate) started_events: Option<Instant>,
    pub(crate) events_time: Duration,
    pub(crate) started_accessibility_updates: Option<Instant>,
    pub(crate) finished_accessibility_updates: Option<Duration>,
    pub(crate) started_presenting: Option<Instant>,
    pub(crate) finished_presenting: Option<Duration>,
    pub(crate) overlay_time: Duration,
}

impl WindowMetrics {
    pub(crate) fn record_frame_time(&mut self) {
        let Some(started_redraw) = self.started_redraw.take() else {
            return;
        };
        let frame_time = started_redraw.elapsed().as_secs_f32() * 1000.0;
        self.frame_times.push(frame_time);
        if self.frame_times.len() > FRAME_TIME_SAMPLES {
            self.frame_times.remove(0);
        }
    }

    pub(crate) fn performance_metrics(&self) -> PerformanceMetrics {
        let rendering = self
            .started_render
            .map(|started_render| started_render.elapsed())
            .unwrap_or_default();
        let presenting = self.finished_presenting.unwrap_or_default();
        let layout = self.finished_layout.unwrap_or_default();
        let tree_updates = self.finished_tree_updates.unwrap_or_default();
        let accessibility_updates = self.finished_accessibility_updates.unwrap_or_default();
        let frame = rendering
            + presenting
            + layout
            + tree_updates
            + self.tasks_poll_time
            + self.events_time
            + accessibility_updates
            + self.overlay_time;

        PerformanceMetrics {
            fps: self.frames.len(),
            rendering_ms: rendering.as_secs_f64() * 1000.0,
            presenting_ms: presenting.as_secs_f64() * 1000.0,
            layout_ms: layout.as_secs_f64() * 1000.0,
            tree_updates_ms: tree_updates.as_secs_f64() * 1000.0,
            accessibility_updates_ms: accessibility_updates.as_secs_f64() * 1000.0,
            tasks_ms: self.tasks_poll_time.as_secs_f64() * 1000.0,
            events_ms: self.events_time.as_secs_f64() * 1000.0,
            overlay_ms: self.overlay_time.as_secs_f64() * 1000.0,
            frame_ms: frame.as_secs_f64() * 1000.0,
        }
    }

    pub(crate) fn graph_scale_max(&mut self) -> f32 {
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

pub(crate) fn format_bytes(bytes: usize) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GiB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.1} MiB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
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

pub(crate) fn draw_axis_label(
    canvas: &Canvas,
    font_collection: &FontCollection,
    text: &str,
    x: f32,
    y: f32,
) {
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

pub(crate) fn add_text(paragraph_builder: &mut ParagraphBuilder, text: String, font_size: f32) {
    let mut text_style = TextStyle::default();
    text_style.set_color(Color::from_rgb(255, 204, 92));
    let font_style = FontStyle::new(Weight::BOLD, Width::NORMAL, Slant::Upright);
    text_style.set_font_style(font_style);
    text_style.set_font_size(font_size);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
}
