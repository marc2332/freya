use std::{
    collections::HashMap,
    time::{
        Duration,
        Instant,
    },
};

use freya_core::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};
use freya_engine::prelude::{
    Color,
    FontStyle,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    Rect,
    Slant,
    TextShadow,
    TextStyle,
    Weight,
    Width,
};
use freya_winit::reexports::winit::window::WindowId;

#[derive(Default)]
pub struct PerformanceOverlayPlugin {
    metrics: HashMap<WindowId, WindowMetrics>,
}

#[derive(Default)]
struct WindowMetrics {
    frames: Vec<Instant>,
    fps_historic: Vec<usize>,
    max_fps: usize,

    started_render: Option<Instant>,

    started_layout: Option<Instant>,
    finished_layout: Option<Duration>,

    started_dom_updates: Option<Instant>,
    finished_dom_updates: Option<Duration>,

    started_events: Option<Instant>,
    finished_events: Option<Duration>,
}

impl PerformanceOverlayPlugin {
    fn get_metrics(&mut self, id: WindowId) -> &mut WindowMetrics {
        self.metrics.entry(id).or_default()
    }
}

impl FreyaPlugin for PerformanceOverlayPlugin {
    fn on_event(&mut self, event: &PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::StartedMeasuringLayout { window, .. } => {
                self.get_metrics(window.id()).started_layout = Some(Instant::now())
            }
            PluginEvent::FinishedMeasuringLayout { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_layout = Some(metrics.started_layout.unwrap().elapsed())
            }
            PluginEvent::StartedMeasuringEvents { window, .. } => {
                self.get_metrics(window.id()).started_events = Some(Instant::now())
            }
            PluginEvent::FinishedMeasuringEvents { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_events = Some(metrics.started_events.unwrap().elapsed())
            }
            PluginEvent::StartedUpdatingDOM { window, .. } => {
                self.get_metrics(window.id()).started_dom_updates = Some(Instant::now())
            }
            PluginEvent::FinishedUpdatingDOM { window, .. } => {
                let metrics = self.get_metrics(window.id());
                metrics.finished_dom_updates = Some(metrics.started_dom_updates.unwrap().elapsed())
            }
            PluginEvent::BeforeRender { window, .. } => {
                self.get_metrics(window.id()).started_render = Some(Instant::now())
            }
            PluginEvent::AfterRender {
                window,
                canvas,
                font_collection,
                fdom,
            } => {
                let metrics = self.get_metrics(window.id());
                let started_render = metrics.started_render.take().unwrap();
                let finished_layout = metrics.finished_layout.unwrap();
                let finished_events = metrics.finished_events.unwrap_or_default();
                let finished_dom_updates = metrics.finished_dom_updates.unwrap();

                let rdom = fdom.rdom();
                let layout = fdom.layout();
                let animation_clock = fdom.animation_clock();

                let now = Instant::now();

                metrics
                    .frames
                    .retain(|frame| now.duration_since(*frame).as_millis() < 1000);

                metrics.frames.push(now);

                // Render the texts
                let mut paragraph_builder =
                    ParagraphBuilder::new(&ParagraphStyle::default(), *font_collection);
                let mut text_style = TextStyle::default();
                text_style.set_color(Color::from_rgb(63, 255, 0));
                text_style.add_shadow(TextShadow::new(
                    Color::from_rgb(60, 60, 60),
                    (0.0, 1.0),
                    1.0,
                ));
                paragraph_builder.push_style(&text_style);

                // FPS
                add_text(
                    &mut paragraph_builder,
                    format!("{} FPS\n", metrics.frames.len()),
                    30.0,
                );

                metrics.fps_historic.push(metrics.frames.len());
                if metrics.fps_historic.len() > 70 {
                    metrics.fps_historic.remove(0);
                }

                // Rendering time
                add_text(
                    &mut paragraph_builder,
                    format!("Rendering: {}ms \n", started_render.elapsed().as_millis()),
                    18.0,
                );

                // Layout time
                add_text(
                    &mut paragraph_builder,
                    format!("Layout: {}ms \n", finished_layout.as_millis()),
                    18.0,
                );

                // Events time
                add_text(
                    &mut paragraph_builder,
                    format!("Events: {}ms \n", finished_events.as_millis()),
                    18.0,
                );

                // DOM updates time
                add_text(
                    &mut paragraph_builder,
                    format!("DOM Updates: {}ms \n", finished_dom_updates.as_millis()),
                    18.0,
                );

                // DOM size
                add_text(
                    &mut paragraph_builder,
                    format!("{} DOM Nodes \n", rdom.tree_ref().len()),
                    14.0,
                );

                // Layout size
                add_text(
                    &mut paragraph_builder,
                    format!("{} Layout Nodes \n", layout.size()),
                    14.0,
                );

                // Animation clock speed
                add_text(
                    &mut paragraph_builder,
                    format!("Animation clock speed: {}x \n", animation_clock.speed()),
                    14.0,
                );

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(f32::MAX);
                paragraph.paint(canvas, (5.0, 0.0));

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(Color::from_argb(120, 255, 255, 255));

                metrics.max_fps = metrics.max_fps.max(
                    metrics
                        .fps_historic
                        .iter()
                        .max()
                        .copied()
                        .unwrap_or_default(),
                );
                let start_x = 5.0;
                let start_y = 250.0 + metrics.max_fps.max(60) as f32;

                canvas.draw_rect(Rect::new(5., 200., 200., start_y), &paint);

                for (i, fps) in metrics.fps_historic.iter().enumerate() {
                    let mut paint = Paint::default();
                    paint.set_anti_alias(true);
                    paint.set_style(PaintStyle::Fill);
                    paint.set_color(Color::from_rgb(63, 255, 0));
                    paint.set_stroke_width(3.0);

                    let x = start_x + (i * 2) as f32;
                    let y = start_y - *fps as f32 + 2.0;
                    canvas.draw_circle((x, y), 2.0, &paint);
                }
            }
            _ => {}
        }
    }
}

fn add_text(paragraph_builder: &mut ParagraphBuilder, text: String, font_size: f32) {
    let mut text_style = TextStyle::default();
    text_style.set_color(Color::from_rgb(25, 225, 35));
    let font_style = FontStyle::new(Weight::BOLD, Width::EXPANDED, Slant::Upright);
    text_style.set_font_style(font_style);
    text_style.add_shadow(TextShadow::new(
        Color::from_rgb(65, 65, 65),
        (0.0, 1.0),
        1.0,
    ));
    text_style.set_font_size(font_size);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
}
