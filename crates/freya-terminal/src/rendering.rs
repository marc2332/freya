use std::hash::{
    Hash,
    Hasher,
};

use freya_core::{
    fifo_cache::FifoCache,
    prelude::Color,
};
use freya_engine::prelude::{
    Canvas,
    Font,
    FontCollection,
    Paint,
    Paragraph,
    ParagraphBuilder,
    ParagraphStyle,
    TextBlob,
    TextStyle,
};
use rustc_hash::FxHasher;

use crate::colors::map_vt100_color;

pub(crate) enum CachedRow {
    TextBlobs(Vec<(TextBlob, Color)>),
    Paragraph(Paragraph),
}

/// Renders terminal text using TextBlob (fast) or Paragraph (font fallback).
pub(crate) struct TextRenderer<'a> {
    pub canvas: &'a Canvas,
    pub font: &'a Font,
    pub font_collection: &'a mut FontCollection,
    pub paint: &'a mut Paint,
    pub row_cache: &'a mut FifoCache<u64, CachedRow>,
    pub area_min_x: f32,
    pub char_width: f32,
    pub line_height: f32,
    pub baseline_offset: f32,
    pub foreground: Color,
    pub background: Color,
    pub font_family: &'a str,
    pub font_size: f32,
}

impl TextRenderer<'_> {
    fn cell_text(cell: &vt100::Cell) -> &str {
        if cell.has_contents() {
            cell.contents()
        } else {
            " "
        }
    }

    fn cell_foreground(&self, cell: &vt100::Cell) -> Color {
        if cell.inverse() {
            map_vt100_color(cell.bgcolor(), self.background)
        } else {
            map_vt100_color(cell.fgcolor(), self.foreground)
        }
    }

    fn render_blob(
        &mut self,
        glyphs: &str,
        glyph_positions: &[f32],
        text_y: f32,
        blobs: &mut Vec<(TextBlob, Color)>,
        color: Color,
    ) {
        if let Some(blob) = TextBlob::from_pos_text_h(glyphs, glyph_positions, 0.0, self.font) {
            self.paint.set_color(color);
            self.canvas
                .draw_text_blob(&blob, (self.area_min_x, text_y), self.paint);
            blobs.push((blob, color));
        }
    }

    pub fn render_text(
        &mut self,
        rows: &[Vec<vt100::Cell>],
        area_min_y: f32,
        area_max_y: f32,
        mut pre_row: impl FnMut(&[vt100::Cell], f32, &Canvas, &mut Paint),
        mut post_row: impl FnMut(usize, &[vt100::Cell], f32, &Canvas, &mut Paint),
    ) {
        let mut y = area_min_y;

        for (row_idx, row) in rows.iter().enumerate() {
            if y + self.line_height > area_max_y {
                break;
            }

            pre_row(row, y, self.canvas, self.paint);

            let mut hasher = FxHasher::default();
            let mut needs_fallback = false;
            for cell in row.iter() {
                if cell.is_wide_continuation() {
                    continue;
                }
                let contents = cell.contents();
                let cell_fg = self.cell_foreground(cell);
                contents.hash(&mut hasher);
                cell_fg.hash(&mut hasher);
                if !needs_fallback {
                    needs_fallback = cell.is_wide()
                        || (!contents.is_ascii()
                            && self.font.text_to_glyphs_vec(contents).contains(&0));
                }
            }
            let cache_key = hasher.finish();
            let text_y = y + self.baseline_offset;

            if let Some(cached) = self.row_cache.get(&cache_key) {
                match cached {
                    CachedRow::TextBlobs(blobs) => {
                        for (blob, color) in blobs {
                            self.paint.set_color(*color);
                            self.canvas
                                .draw_text_blob(blob, (self.area_min_x, text_y), self.paint);
                        }
                    }
                    CachedRow::Paragraph(paragraph) => {
                        paragraph.paint(self.canvas, (self.area_min_x, y));
                    }
                }
            } else if needs_fallback {
                self.render_paragraph(row, y, cache_key);
            } else {
                self.render_textblob(row, text_y, cache_key);
            }

            post_row(row_idx, row, y, self.canvas, self.paint);

            y += self.line_height;
        }
    }

    /// Fast path: TextBlob with explicit grid positions per glyph.
    fn render_textblob(&mut self, row: &[vt100::Cell], text_y: f32, cache_key: u64) {
        let mut current_color: Option<Color> = None;

        // Same-color glyphs are batched into a single TextBlob.
        // Each char gets a grid x-offset to preserve monospace alignment.
        let mut glyphs = String::new();
        let mut glyph_positions: Vec<f32> = Vec::new();

        let mut blobs: Vec<(TextBlob, Color)> = Vec::new();

        for (col_idx, cell) in row.iter().enumerate() {
            if cell.is_wide_continuation() {
                continue;
            }
            let cell_fg = self.cell_foreground(cell);
            let text = Self::cell_text(cell);
            let x = (col_idx as f32) * self.char_width;

            if current_color != Some(cell_fg) {
                if let Some(prev_color) = current_color {
                    self.render_blob(&glyphs, &glyph_positions, text_y, &mut blobs, prev_color);
                    glyphs.clear();
                    glyph_positions.clear();
                }
                current_color = Some(cell_fg);
            }
            for _ in text.chars() {
                glyph_positions.push(x);
            }
            glyphs.push_str(text);
        }

        if !glyphs.is_empty() {
            self.render_blob(
                &glyphs,
                &glyph_positions,
                text_y,
                &mut blobs,
                current_color.unwrap(),
            );
        }

        self.row_cache
            .insert(cache_key, CachedRow::TextBlobs(blobs));
    }

    /// Slow path: Paragraph with font fallback for emoji/wide chars.
    fn render_paragraph(&mut self, row: &[vt100::Cell], row_y: f32, cache_key: u64) {
        let mut text_style = TextStyle::new();
        text_style.set_font_size(self.font_size);
        text_style.set_font_families(&[self.font_family]);
        text_style.set_color(self.foreground);

        let mut builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), self.font_collection.clone());

        for cell in row.iter() {
            if cell.is_wide_continuation() {
                continue;
            }
            let mut cell_style = text_style.clone();
            cell_style.set_color(self.cell_foreground(cell));
            builder.push_style(&cell_style);
            builder.add_text(Self::cell_text(cell));
        }

        let mut paragraph = builder.build();
        paragraph.layout(f32::MAX);
        paragraph.paint(self.canvas, (self.area_min_x, row_y));

        self.row_cache
            .insert(cache_key, CachedRow::Paragraph(paragraph));
    }
}
