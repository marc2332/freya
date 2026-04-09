use std::hash::{
    Hash,
    Hasher,
};

use alacritty_terminal::term::cell::{
    Cell,
    Flags,
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
    SkRect,
    TextBlob,
    TextStyle,
};
use rustc_hash::FxHasher;
use torin::prelude::Area;

use crate::colors::map_ansi_color;

/// Cached per-row drawing primitives keyed by a hash of the row contents.
pub(crate) enum CachedRow {
    /// Same-color glyph runs ready to be redrawn directly.
    TextBlobs(Vec<(TextBlob, Color)>),
    /// Skia paragraph used when font fallback was required (emoji, wide chars).
    Paragraph(Paragraph),
}

/// Selection range expressed in viewport row indices, normalized so the
/// `start_*` values precede the `end_*` values in document order.
#[derive(Clone, Copy)]
pub(crate) struct SelectionBounds {
    pub start_row: i64,
    pub start_col: usize,
    pub end_row: i64,
    pub end_col: usize,
}

/// Single renderer that draws the terminal background, a row's cell
/// backgrounds, glyph runs, selection overlay, cursor, and scrollbar.
pub(crate) struct Renderer<'a> {
    pub canvas: &'a Canvas,
    pub paint: &'a mut Paint,
    pub font: &'a Font,
    pub font_collection: &'a mut FontCollection,
    pub row_cache: &'a mut FifoCache<u64, CachedRow>,
    pub area: Area,
    pub char_width: f32,
    pub line_height: f32,
    pub baseline_offset: f32,
    pub foreground: Color,
    pub background: Color,
    pub selection_color: Color,
    pub font_family: &'a str,
    pub font_size: f32,
}

impl Renderer<'_> {
    pub fn render_background(&mut self) {
        self.paint.set_color(self.background);
        self.canvas.draw_rect(
            SkRect::new(
                self.area.min_x(),
                self.area.min_y(),
                self.area.max_x(),
                self.area.max_y(),
            ),
            self.paint,
        );
    }

    /// Render one row: cell backgrounds, glyphs, then any selection overlay.
    pub fn render_row(
        &mut self,
        row_idx: usize,
        row: &[Cell],
        y: f32,
        selection: Option<&SelectionBounds>,
    ) {
        self.render_cell_backgrounds(row, y);
        self.render_text_row(row, y);
        if let Some(bounds) = selection {
            self.render_selection(row_idx, row.len(), y, bounds);
        }
    }

    pub fn render_cursor(&mut self, row: &[Cell], y: f32, cursor_col: usize) {
        let left = self.area.min_x() + (cursor_col as f32) * self.char_width;
        let right = left + self.char_width.max(1.0);
        let bottom = y + self.line_height.max(1.0);

        self.paint.set_color(self.foreground);
        self.canvas.draw_rect(
            SkRect::new(left, y.round(), right, bottom.round()),
            self.paint,
        );

        let content = row
            .get(cursor_col)
            .map(|cell| if cell.c == '\0' { ' ' } else { cell.c })
            .unwrap_or(' ')
            .to_string();

        self.paint.set_color(self.background);
        if let Some(blob) = TextBlob::from_pos_text_h(&content, &[0.0], 0.0, self.font) {
            self.canvas
                .draw_text_blob(&blob, (left, y + self.baseline_offset), self.paint);
        }
    }

    pub fn render_scrollbar(
        &mut self,
        scroll_offset: usize,
        total_scrollback: usize,
        rows_count: usize,
    ) {
        let viewport_height = self.area.height();
        let total_rows = rows_count + total_scrollback;
        let total_content_height = total_rows as f32 * self.line_height;

        let scrollbar_height = (viewport_height * viewport_height / total_content_height).max(20.0);
        let track_height = viewport_height - scrollbar_height;
        let scroll_ratio = scroll_offset as f32 / total_scrollback as f32;
        let thumb_y = self.area.min_y() + track_height * (1.0 - scroll_ratio);

        let scrollbar_x = self.area.max_x() - 4.0;
        let corner_radius = 2.0;

        self.paint.set_anti_alias(true);
        self.paint.set_color(Color::from_argb(50, 0, 0, 0));
        self.canvas.draw_round_rect(
            SkRect::new(
                scrollbar_x,
                self.area.min_y(),
                self.area.max_x(),
                self.area.max_y(),
            ),
            corner_radius,
            corner_radius,
            self.paint,
        );

        self.paint.set_color(Color::from_argb(60, 255, 255, 255));
        self.canvas.draw_round_rect(
            SkRect::new(
                scrollbar_x,
                thumb_y,
                self.area.max_x(),
                thumb_y + scrollbar_height,
            ),
            corner_radius,
            corner_radius,
            self.paint,
        );
    }

    fn render_cell_backgrounds(&mut self, row: &[Cell], y: f32) {
        let mut run_start: Option<(usize, Color)> = None;
        let mut col = 0;
        while col < row.len() {
            let cell = &row[col];
            if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
                col += 1;
                continue;
            }
            let cell_bg = if cell.flags.contains(Flags::INVERSE) {
                map_ansi_color(cell.fg, self.foreground, self.background)
            } else {
                map_ansi_color(cell.bg, self.foreground, self.background)
            };
            let end_col = if cell.flags.contains(Flags::WIDE_CHAR) {
                col + 2
            } else {
                col + 1
            };

            if cell_bg != self.background {
                match &run_start {
                    Some((_, color)) if *color == cell_bg => {}
                    Some((start, color)) => {
                        self.fill_cells(*start, col, *color, y);
                        run_start = Some((col, cell_bg));
                    }
                    None => {
                        run_start = Some((col, cell_bg));
                    }
                }
            } else if let Some((start, color)) = run_start.take() {
                self.fill_cells(start, col, color, y);
            }
            col = end_col;
        }
        if let Some((start, color)) = run_start {
            self.fill_cells(start, col, color, y);
        }
    }

    fn fill_cells(&mut self, start: usize, end: usize, color: Color, y: f32) {
        let left = self.area.min_x() + (start as f32) * self.char_width;
        let right = self.area.min_x() + (end as f32) * self.char_width;
        self.paint.set_color(color);
        self.canvas.draw_rect(
            SkRect::new(left, y.round(), right, (y + self.line_height).round()),
            self.paint,
        );
    }

    fn render_selection(
        &mut self,
        row_idx: usize,
        row_len: usize,
        y: f32,
        bounds: &SelectionBounds,
    ) {
        let row_i = row_idx as i64;
        if row_i < bounds.start_row || row_i > bounds.end_row {
            return;
        }
        let sel_start = if row_i == bounds.start_row {
            bounds.start_col
        } else {
            0
        };
        let sel_end = if row_i == bounds.end_row {
            bounds.end_col.min(row_len)
        } else {
            row_len
        };
        if sel_start >= sel_end {
            return;
        }
        let left = self.area.min_x() + (sel_start as f32) * self.char_width;
        let right = self.area.min_x() + (sel_end as f32) * self.char_width;
        self.paint.set_color(self.selection_color);
        self.canvas.draw_rect(
            SkRect::new(left, y.round(), right, (y + self.line_height).round()),
            self.paint,
        );
    }

    /// Draw a row's glyphs, hashing the row contents to hit `row_cache` on
    /// repeat frames. Picks the fast `TextBlob` path or the `Paragraph` path
    /// when font fallback (emoji, wide chars) is needed.
    fn render_text_row(&mut self, row: &[Cell], y: f32) {
        let mut hasher = FxHasher::default();
        let mut needs_fallback = false;
        for cell in row.iter() {
            if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
                continue;
            }
            let text = cell_text(cell);
            let cell_fg = self.cell_foreground(cell);
            text.hash(&mut hasher);
            cell_fg.hash(&mut hasher);
            if !needs_fallback {
                needs_fallback = cell.flags.contains(Flags::WIDE_CHAR)
                    || (!text.is_ascii() && self.font.text_to_glyphs_vec(&text).contains(&0));
            }
        }
        let cache_key = hasher.finish();
        let text_y = y + self.baseline_offset;
        let area_min_x = self.area.min_x();

        if let Some(cached) = self.row_cache.get(&cache_key) {
            match cached {
                CachedRow::TextBlobs(blobs) => {
                    for (blob, color) in blobs {
                        self.paint.set_color(*color);
                        self.canvas
                            .draw_text_blob(blob, (area_min_x, text_y), self.paint);
                    }
                }
                CachedRow::Paragraph(paragraph) => {
                    paragraph.paint(self.canvas, (area_min_x, y));
                }
            }
        } else if needs_fallback {
            self.render_paragraph(row, y, cache_key);
        } else {
            self.render_textblob(row, text_y, cache_key);
        }
    }

    fn cell_foreground(&self, cell: &Cell) -> Color {
        let raw = if cell.flags.contains(Flags::INVERSE) {
            cell.bg
        } else {
            cell.fg
        };
        map_ansi_color(raw, self.foreground, self.background)
    }

    /// Fast path: same-color glyphs batched into one `TextBlob`, each glyph
    /// pinned to its grid x-offset to preserve monospace alignment.
    fn render_textblob(&mut self, row: &[Cell], text_y: f32, cache_key: u64) {
        let mut current_color: Option<Color> = None;
        let mut glyphs = String::new();
        let mut glyph_positions: Vec<f32> = Vec::new();
        let mut blobs: Vec<(TextBlob, Color)> = Vec::new();

        for (col_idx, cell) in row.iter().enumerate() {
            if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
                continue;
            }
            let cell_fg = self.cell_foreground(cell);
            let text = cell_text(cell);
            let x = (col_idx as f32) * self.char_width;

            if current_color != Some(cell_fg) {
                if let Some(prev_color) = current_color {
                    self.flush_blob(&glyphs, &glyph_positions, text_y, &mut blobs, prev_color);
                    glyphs.clear();
                    glyph_positions.clear();
                }
                current_color = Some(cell_fg);
            }
            for _ in text.chars() {
                glyph_positions.push(x);
            }
            glyphs.push_str(&text);
        }

        if let Some(color) = current_color
            && !glyphs.is_empty()
        {
            self.flush_blob(&glyphs, &glyph_positions, text_y, &mut blobs, color);
        }

        self.row_cache
            .insert(cache_key, CachedRow::TextBlobs(blobs));
    }

    fn flush_blob(
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
                .draw_text_blob(&blob, (self.area.min_x(), text_y), self.paint);
            blobs.push((blob, color));
        }
    }

    /// Slow path: Paragraph with font fallback for emoji/wide chars.
    fn render_paragraph(&mut self, row: &[Cell], row_y: f32, cache_key: u64) {
        let mut text_style = TextStyle::new();
        text_style.set_font_size(self.font_size);
        text_style.set_font_families(&[self.font_family]);
        text_style.set_color(self.foreground);

        let mut builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), self.font_collection.clone());

        for cell in row.iter() {
            if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
                continue;
            }
            let mut cell_style = text_style.clone();
            cell_style.set_color(self.cell_foreground(cell));
            builder.push_style(&cell_style);
            builder.add_text(cell_text(cell).as_str());
        }

        let mut paragraph = builder.build();
        paragraph.layout(f32::MAX);
        paragraph.paint(self.canvas, (self.area.min_x(), row_y));

        self.row_cache
            .insert(cache_key, CachedRow::Paragraph(paragraph));
    }
}

/// Printable text for a cell, treating empty cells as a space and appending
/// any combining (zero-width) characters.
fn cell_text(cell: &Cell) -> String {
    let mut s = String::new();
    s.push(if cell.c == '\0' { ' ' } else { cell.c });
    if let Some(extra) = cell.zerowidth() {
        for c in extra {
            s.push(*c);
        }
    }
    s
}
