#![cfg(debug_assertions)]

use freya_core::metrics::Metrics;
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
    SkTextShadow as TextShadow,
    Slant,
    TextAlign,
    TextStyle,
    Weight,
    Width,
};

pub(crate) fn draw_stats_overlay(
    canvas: &Canvas,
    font_collection: &FontCollection,
    scale_factor: f32,
    window_width_pixels: u32,
    metrics: Metrics,
) {
    let rows = [
        ("Tree Nodes", metrics.tree_nodes),
        ("Layout Nodes", metrics.layout_nodes),
        ("Components", metrics.scopes),
        ("Scope values", metrics.scope_values),
        ("Async tasks", metrics.alive_tasks),
        ("Cached assets", metrics.cached_assets),
        ("Contexts", metrics.contexts),
        ("Reactive contexts", metrics.reactive_contexts),
        ("Text cache", metrics.text_cache_size),
        ("Text cache users", metrics.text_cache_users),
    ];
    let mut keys_builder =
        ParagraphBuilder::new(&ParagraphStyle::default(), font_collection.clone());
    let mut values_style = ParagraphStyle::default();
    values_style.set_text_align(TextAlign::Right);
    let mut values_builder = ParagraphBuilder::new(&values_style, font_collection.clone());
    for (index, (key, value)) in rows.iter().enumerate() {
        let line_break = if index + 1 == rows.len() { "" } else { "\n" };
        add_debug_text(&mut keys_builder, format!("{key}{line_break}"), 14.0);
        add_debug_text(&mut values_builder, format!("{value}{line_break}"), 14.0);
    }
    let mut keys = keys_builder.build();
    let mut values = values_builder.build();
    keys.layout(235.0);
    values.layout(235.0);

    let window_width = window_width_pixels as f32 / scale_factor;
    let left = (window_width - 255.0).max(5.0);
    let top = 5.0;
    let height = keys.height() + 4.0;

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_color(Color::from_argb(235, 24, 24, 24));
    canvas.draw_rect(Rect::new(left, top, left + 250.0, top + height), &paint);

    keys.paint(canvas, (left + 8.0, top + 2.0));
    values.paint(canvas, (left + 8.0, top + 2.0));
}

#[cfg(debug_assertions)]
fn add_debug_text(paragraph_builder: &mut ParagraphBuilder, text: String, font_size: f32) {
    let mut text_style = TextStyle::default();
    text_style.set_color(Color::from_rgb(115, 190, 255));
    text_style.set_font_style(FontStyle::new(Weight::BOLD, Width::NORMAL, Slant::Upright));
    text_style.set_font_size(font_size);
    text_style.add_shadow(TextShadow::new(
        Color::from_argb(120, 115, 190, 255),
        (0.0, 0.0),
        1.5,
    ));
    paragraph_builder.push_style(&text_style);
    paragraph_builder.add_text(text);
}
