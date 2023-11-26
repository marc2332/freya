use std::time::{Duration, Instant};

use freya_core::plugins::{FreyaPlugin, PluginEvent};
use freya_engine::prelude::{Color, ParagraphBuilder, ParagraphStyle, TextShadow, TextStyle};

#[derive(Default)]
pub struct PerformanceOverlayPlugin {
    frames: Vec<Instant>,
    started_render: Option<Instant>,
    started_layout: Option<Instant>,
    finished_layout: Option<Duration>,
}

impl FreyaPlugin for PerformanceOverlayPlugin {
    fn on_event(&mut self, event: &PluginEvent) {
        match event {
            PluginEvent::StartedLayout(_) => self.started_layout = Some(Instant::now()),
            PluginEvent::FinishedLayout(_) => {
                self.finished_layout = Some(self.started_layout.unwrap().elapsed())
            }
            PluginEvent::StartedRender(_canvas, _font_collection) => {
                self.started_render = Some(Instant::now())
            }
            PluginEvent::FinishedRender(canvas, font_collection) => {
                let started_render = self.started_render.take().unwrap();
                let finished_layout = self.finished_layout.unwrap();

                let now = Instant::now();

                self.frames
                    .retain(|frame| now.duration_since(*frame).as_millis() < 1000);

                self.frames.push(now);

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
                let mut text_style = TextStyle::default();
                text_style.set_color(Color::from_rgb(63, 255, 0));
                text_style.add_shadow(TextShadow::new(
                    Color::from_rgb(60, 60, 60),
                    (0.0, 1.0),
                    1.0,
                ));
                text_style.set_font_size(30.0);
                paragraph_builder.push_style(&text_style);
                paragraph_builder.add_text(format!("{} \n", self.frames.len()));

                // Rendering time
                let mut text_style = TextStyle::default();
                text_style.set_color(Color::from_rgb(63, 255, 0));
                text_style.add_shadow(TextShadow::new(
                    Color::from_rgb(60, 60, 60),
                    (0.0, 1.0),
                    1.0,
                ));
                text_style.set_font_size(18.0);
                paragraph_builder.push_style(&text_style);
                paragraph_builder.add_text(format!(
                    "Rendering: {}ms \n",
                    started_render.elapsed().as_millis()
                ));

                // Layout time
                let mut text_style = TextStyle::default();
                text_style.set_color(Color::from_rgb(63, 255, 0));
                text_style.add_shadow(TextShadow::new(
                    Color::from_rgb(60, 60, 60),
                    (0.0, 1.0),
                    1.0,
                ));
                text_style.set_font_size(18.0);
                paragraph_builder.push_style(&text_style);
                paragraph_builder.add_text(format!("Layout: {}ms \n", finished_layout.as_millis()));

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(f32::MAX);
                paragraph.paint(canvas, (5.0, 0.0));
            }
            _ => {}
        }
    }
}
