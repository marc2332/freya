use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
};

use freya_core::{
    data::{
        AccessibilityData,
        LayoutData,
    },
    diff_key::DiffKey,
    element::{
        Element,
        ElementExt,
        EventHandlerType,
        LayoutContext,
        RenderContext,
    },
    events::name::EventName,
    fifo_cache::FifoCache,
    prelude::*,
    tree::DiffModifies,
};
use freya_engine::prelude::{
    Canvas,
    Font,
    FontEdging,
    FontHinting,
    FontStyle,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    SkRect,
    TextBlob,
    TextStyle,
};
use rustc_hash::FxHashMap;
use torin::prelude::{
    Area,
    Size2D,
};

use crate::{
    colors::map_vt100_color,
    handle::TerminalHandle,
    rendering::{
        CachedRow,
        TextRenderer,
    },
};

/// Cached layout measurements and font for text drawing.
struct TerminalMeasure {
    char_width: f32,
    line_height: f32,
    font: Font,
    font_family: String,
    font_size: f32,
    row_cache: RefCell<FifoCache<u64, CachedRow>>,
}

/// Renders selection, backgrounds, cursor, and scrollbar.
struct TerminalRenderer<'a> {
    canvas: &'a Canvas,
    paint: &'a mut Paint,
    area: Area,
    char_width: f32,
    line_height: f32,
    baseline_offset: f32,
    foreground: Color,
    background: Color,
    selection_color: Color,
}

impl TerminalRenderer<'_> {
    fn render_background(&mut self) {
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

    fn render_selection(
        &mut self,
        row_idx: usize,
        row_len: usize,
        y: f32,
        selection_bounds: &(i64, usize, i64, usize),
    ) {
        let (display_start, start_col, display_end, end_col) = selection_bounds;
        let row_i = row_idx as i64;

        if row_i < *display_start || row_i > *display_end {
            return;
        }

        let sel_start = if row_i == *display_start {
            *start_col
        } else {
            0
        };
        let sel_end = if row_i == *display_end {
            (*end_col).min(row_len)
        } else {
            row_len
        };

        if sel_start < sel_end {
            let left = self.area.min_x() + (sel_start as f32) * self.char_width;
            let right = self.area.min_x() + (sel_end as f32) * self.char_width;
            self.paint.set_color(self.selection_color);
            self.canvas.draw_rect(
                SkRect::new(left, y, right, y + self.line_height),
                self.paint,
            );
        }
    }

    fn render_cell_backgrounds(&mut self, row: &[vt100::Cell], y: f32) {
        let mut run_start: Option<(usize, Color)> = None;
        let mut col = 0;
        while col < row.len() {
            let cell = &row[col];
            if cell.is_wide_continuation() {
                col += 1;
                continue;
            }
            let cell_bg = if cell.inverse() {
                map_vt100_color(cell.fgcolor(), self.foreground)
            } else {
                map_vt100_color(cell.bgcolor(), self.background)
            };
            let end_col = if cell.is_wide() { col + 2 } else { col + 1 };

            if cell_bg != self.background {
                match &run_start {
                    Some((_, color)) if *color == cell_bg => {}
                    Some((start, color)) => {
                        self.render_cell_background(*start, col, *color, y);
                        run_start = Some((col, cell_bg));
                    }
                    None => {
                        run_start = Some((col, cell_bg));
                    }
                }
            } else if let Some((start, color)) = run_start.take() {
                self.render_cell_background(start, col, color, y);
            }
            col = end_col;
        }
        if let Some((start, color)) = run_start {
            self.render_cell_background(start, col, color, y);
        }
    }

    fn render_cell_background(&mut self, start: usize, end: usize, color: Color, y: f32) {
        let left = self.area.min_x() + (start as f32) * self.char_width;
        let right = self.area.min_x() + (end as f32) * self.char_width;
        self.paint.set_color(color);
        self.canvas.draw_rect(
            SkRect::new(left, y, right, y + self.line_height),
            self.paint,
        );
    }

    fn render_cursor(&mut self, row: &[vt100::Cell], y: f32, cursor_col: usize, font: &Font) {
        let left = self.area.min_x() + (cursor_col as f32) * self.char_width;
        let right = left + self.char_width.max(1.0);
        let bottom = y + self.line_height.max(1.0);

        self.paint.set_color(self.foreground);
        self.canvas
            .draw_rect(SkRect::new(left, y, right, bottom), self.paint);

        let content = row
            .get(cursor_col)
            .map(|cell| {
                if cell.has_contents() {
                    cell.contents()
                } else {
                    " "
                }
            })
            .unwrap_or(" ");

        self.paint.set_color(self.background);
        if let Some(blob) = TextBlob::from_pos_text_h(content, &[0.0], 0.0, font) {
            self.canvas
                .draw_text_blob(&blob, (left, y + self.baseline_offset), self.paint);
        }
    }

    fn render_scrollbar(
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
}

#[derive(Clone)]
pub struct Terminal {
    handle: TerminalHandle,
    layout_data: LayoutData,
    accessibility: AccessibilityData,
    font_family: String,
    font_size: f32,
    foreground: Color,
    background: Color,
    selection_color: Color,
    on_measured: Option<EventHandler<(f32, f32)>>,
    event_handlers: FxHashMap<EventName, EventHandlerType>,
}

impl PartialEq for Terminal {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
            && self.font_size == other.font_size
            && self.font_family == other.font_family
            && self.foreground == other.foreground
            && self.background == other.background
            && self.event_handlers.len() == other.event_handlers.len()
    }
}

impl Terminal {
    pub fn new(handle: TerminalHandle) -> Self {
        let mut accessibility = AccessibilityData::default();
        accessibility.builder.set_role(AccessibilityRole::Terminal);
        Self {
            handle,
            layout_data: Default::default(),
            accessibility,
            font_family: "Cascadia Code".to_string(),
            font_size: 14.,
            foreground: (220, 220, 220).into(),
            background: (10, 10, 10).into(),
            selection_color: (60, 179, 214, 0.3).into(),
            on_measured: None,
            event_handlers: FxHashMap::default(),
        }
    }

    pub fn selection_color(mut self, selection_color: impl Into<Color>) -> Self {
        self.selection_color = selection_color.into();
        self
    }

    pub fn on_measured(mut self, callback: impl Into<EventHandler<(f32, f32)>>) -> Self {
        self.on_measured = Some(callback.into());
        self
    }

    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn foreground(mut self, foreground: impl Into<Color>) -> Self {
        self.foreground = foreground.into();
        self
    }

    pub fn background(mut self, background: impl Into<Color>) -> Self {
        self.background = background.into();
        self
    }
}

impl EventHandlersExt for Terminal {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.event_handlers
    }
}

impl LayoutExt for Terminal {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout_data
    }
}

impl AccessibilityExt for Terminal {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl ElementExt for Terminal {
    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(terminal) = (other.as_ref() as &dyn Any).downcast_ref::<Terminal>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.font_size != terminal.font_size
            || self.font_family != terminal.font_family
            || self.handle != terminal.handle
            || self.event_handlers.len() != terminal.event_handlers.len()
        {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.foreground != terminal.foreground
            || self.background != terminal.background
            || self.selection_color != terminal.selection_color
        {
            diff.insert(DiffModifies::STYLE);
        }

        if self.accessibility != terminal.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout_data)
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        // Measure char width and line height using a reference glyph
        let mut builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());

        let mut style = TextStyle::new();
        style.set_font_size(self.font_size);
        style.set_font_families(&[self.font_family.as_str()]);
        builder.push_style(&style);
        builder.add_text("W");

        let mut paragraph = builder.build();
        paragraph.layout(f32::MAX);
        let mut line_height = paragraph.height();
        if line_height <= 0.0 || line_height.is_nan() {
            line_height = (self.font_size * 1.2).max(1.0);
        }
        let char_width = paragraph.max_intrinsic_width();

        let mut height = context.area_size.height;
        if height <= 0.0 {
            height = (line_height * 24.0).max(200.0);
        }

        let target_cols = if char_width > 0.0 {
            (context.area_size.width / char_width).floor() as u16
        } else {
            0
        }
        .max(1);
        let target_rows = if line_height > 0.0 {
            (height / line_height).floor() as u16
        } else {
            0
        }
        .max(1);

        self.handle.resize(target_rows, target_cols);

        if let Some(on_measured) = &self.on_measured {
            on_measured.call((char_width, line_height));
        }

        let typeface = context
            .font_collection
            .find_typefaces(&[&self.font_family], FontStyle::default())
            .into_iter()
            .next()
            .expect("Terminal font family not found");

        let mut font = Font::from_typeface(typeface, self.font_size);
        font.set_subpixel(true);
        font.set_edging(FontEdging::SubpixelAntiAlias);
        font.set_hinting(match self.font_size as u32 {
            0..=6 => FontHinting::Full,
            7..=12 => FontHinting::Normal,
            13..=24 => FontHinting::Slight,
            _ => FontHinting::None,
        });

        Some((
            Size2D::new(context.area_size.width.max(100.0), height),
            Rc::new(TerminalMeasure {
                char_width,
                line_height,
                font,
                font_family: self.font_family.clone(),
                font_size: self.font_size,
                row_cache: RefCell::new(FifoCache::new()),
            }),
        ))
    }

    fn render(&self, context: RenderContext) {
        let area = context.layout_node.visible_area();
        let measure = context
            .layout_node
            .data
            .as_ref()
            .unwrap()
            .downcast_ref::<TerminalMeasure>()
            .unwrap();

        let font = &measure.font;

        let (_, metrics) = font.metrics();
        let baseline_offset = -metrics.ascent;
        let buffer = self.handle.read_buffer();

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);

        let mut renderer = TerminalRenderer {
            canvas: context.canvas,
            paint: &mut paint,
            area,
            char_width: measure.char_width,
            line_height: measure.line_height,
            baseline_offset,
            foreground: self.foreground,
            background: self.background,
            selection_color: self.selection_color,
        };

        renderer.render_background();

        let selection_bounds = buffer.selection.as_ref().and_then(|sel| {
            if sel.is_empty() {
                None
            } else {
                Some(sel.display_positions(buffer.scroll_offset))
            }
        });

        let mut y = area.min_y();
        for (row_idx, row) in buffer.rows.iter().enumerate() {
            if y + measure.line_height > area.max_y() {
                break;
            }

            if let Some(bounds) = &selection_bounds {
                renderer.render_selection(row_idx, row.len(), y, bounds);
            }

            renderer.render_cell_backgrounds(row, y);

            y += measure.line_height;
        }

        {
            let mut text_renderer = TextRenderer {
                canvas: context.canvas,
                font,
                font_collection: context.font_collection,
                paint: renderer.paint,
                row_cache: &mut measure.row_cache.borrow_mut(),
                area_min_x: area.min_x(),
                char_width: measure.char_width,
                line_height: measure.line_height,
                baseline_offset,
                foreground: self.foreground,
                background: self.background,
                font_family: &measure.font_family,
                font_size: measure.font_size,
            };
            text_renderer.render_text(&buffer.rows, area.min_y(), area.max_y());
        }

        if buffer.scroll_offset == 0
            && buffer.cursor_visible
            && let Some(row) = buffer.rows.get(buffer.cursor_row)
        {
            let cursor_y = area.min_y() + (buffer.cursor_row as f32) * measure.line_height;
            if cursor_y + measure.line_height <= area.max_y() {
                renderer.render_cursor(row, cursor_y, buffer.cursor_col, font);
            }
        }

        if buffer.total_scrollback > 0 {
            renderer.render_scrollbar(
                buffer.scroll_offset,
                buffer.total_scrollback,
                buffer.rows_count,
            );
        }
    }
}

impl From<Terminal> for Element {
    fn from(value: Terminal) -> Self {
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(value),
            elements: Vec::new(),
        }
    }
}
