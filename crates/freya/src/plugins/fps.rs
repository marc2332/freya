use std::time::Instant;

use freya_core::plugins::{FreyaPlugin, PluginEvent};
use freya_engine::prelude::{Color, ParagraphBuilder, ParagraphStyle, TextShadow, TextStyle};

#[derive(Default)]
pub struct FpsPlugin {
    frames: Vec<Instant>,
}

impl FreyaPlugin for FpsPlugin {
    fn on_event(&mut self, event: &PluginEvent) {
        if let PluginEvent::CanvasRendered(canvas, font_collection) = event {
            let now = Instant::now();

            self.frames
                .retain(|frame| now.duration_since(*frame).as_millis() < 1000);

            self.frames.push(now);

            let mut text_style = TextStyle::default();
            text_style.set_font_size(30.0);
            text_style.set_color(Color::from_rgb(63, 255, 0));
            text_style.add_shadow(TextShadow::new(
                Color::from_rgb(60, 60, 60),
                (0.0, 1.0),
                1.0,
            ));

            let mut paragraph_style = ParagraphStyle::default();
            paragraph_style.set_text_style(&text_style);

            let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, *font_collection);
            paragraph_builder.add_text(format!("{}", self.frames.len()));

            let mut paragraph = paragraph_builder.build();

            paragraph.layout(f32::MAX);

            paragraph.paint(canvas, (5.0, 0.0));
        }
    }
}
