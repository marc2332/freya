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
            PluginEvent::BeforeRender { .. } => self.started_render = Some(Instant::now()),
            PluginEvent::AfterRender {
                canvas,
                font_collection,
                freya_dom,
                viewports,
            } => {
                let started_render = self.started_render.take().unwrap();
                let finished_layout = self.finished_layout.unwrap();
                let rdom = freya_dom.rdom();
                let layout = freya_dom.layout();

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
                add_text(
                    &mut paragraph_builder,
                    format!("{} \n", self.frames.len()),
                    30.0,
                );

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

                // Viewports
                add_text(
                    &mut paragraph_builder,
                    format!("{} Nodes viewports \n", viewports.size()),
                    14.0,
                );

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(f32::MAX);
                paragraph.paint(canvas, (5.0, 0.0));
            }
            _ => {}
        }
    }
}

fn add_text(paragraph_builder: &mut ParagraphBuilder, text: String, font_size: f32) {
    let mut text_style = TextStyle::default();
    text_style.set_color(Color::from_rgb(63, 255, 0));
    text_style.add_shadow(TextShadow::new(
        Color::from_rgb(60, 60, 60),
        (0.0, 1.0),
        1.0,
    ));
    text_style.set_font_size(font_size);
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
}
