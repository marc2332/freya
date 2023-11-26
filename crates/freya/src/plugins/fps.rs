use std::time::Instant;

use freya_core::plugins::{FreyaPlugin, PluginEvent};
use freya_engine::prelude::{Color, ParagraphBuilder, ParagraphStyle, TextStyle};

#[derive(Default)]
pub struct FpsPlugin {
    frames: Vec<Instant>,
}

impl FreyaPlugin for FpsPlugin {
    fn on_event(&mut self, event: &PluginEvent) {
        match event {
            PluginEvent::CanvasRendered(canvas, font_collection) => {
                let now = Instant::now();

                while self.frames.len() > 0
                    && now.duration_since(self.frames[0]).as_millis() >= 1000
                {
                    self.frames.remove(0);
                }

                self.frames.push(now);

                let mut text_style = TextStyle::default();
                text_style.set_font_size(30.0);
                text_style.set_color(Color::GREEN);

                let mut paragraph_style = ParagraphStyle::default();
                paragraph_style.set_text_style(&text_style);

                let mut paragraph_builder =
                    ParagraphBuilder::new(&paragraph_style, *font_collection);
                paragraph_builder.add_text(format!("{}", self.frames.len()));

                let mut paragraph = paragraph_builder.build();

                paragraph.layout(f32::MAX);

                paragraph.paint(canvas, (0.0, 0.0));
            }
            _ => {}
        }
    }
}
