use std::{
    any::Any,
    borrow::Cow,
    rc::Rc,
};

use freya_core::{
    data::LayoutData,
    diff_key::DiffKey,
    element::{
        Element,
        ElementExt,
    },
    prelude::*,
    tree::DiffModifies,
};
use freya_engine::prelude::{
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    SkRect,
    TextStyle,
};

use crate::{
    colors::{
        map_vt100_bg_color,
        map_vt100_fg_color,
    },
    handle::TerminalHandle,
};

/// Internal terminal rendering element
#[derive(Clone)]
pub struct TerminalElement {
    handle: TerminalHandle,
    layout_data: LayoutData,
    font_family: String,
    font_size: f32,
    fg: Color,
    bg: Color,
}

impl PartialEq for TerminalElement {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
            && self.font_size == other.font_size
            && self.font_family == other.font_family
            && self.fg == other.fg
            && self.bg == other.bg
    }
}

impl TerminalElement {
    pub(crate) fn new(handle: TerminalHandle) -> Self {
        Self {
            handle,
            layout_data: Default::default(),
            font_family: "Cascadia Code".to_string(),
            font_size: 14.,
            fg: (220, 220, 220).into(),
            bg: (10, 10, 10).into(),
        }
    }

    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn colors(mut self, fg: impl Into<Color>, bg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self.bg = bg.into();
        self
    }
}

impl ElementExt for TerminalElement {
    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(el) = (other.as_ref() as &dyn Any).downcast_ref::<TerminalElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.font_size != el.font_size
            || self.font_family != el.font_family
            || self.handle != el.handle
        {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout_data)
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(
        &self,
        context: freya_core::element::LayoutContext,
    ) -> Option<(torin::prelude::Size2D, Rc<dyn Any>)> {
        let mut measure_builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());
        let mut text_style = TextStyle::new();
        text_style.set_font_size(self.font_size);
        text_style.set_font_families(&[self.font_family.as_str()]);
        measure_builder.push_style(&text_style);
        measure_builder.add_text("W");
        let mut measure_paragraph = measure_builder.build();
        measure_paragraph.layout(f32::MAX);
        let mut line_height = measure_paragraph.height();
        if line_height <= 0.0 || line_height.is_nan() {
            line_height = (self.font_size * 1.2).max(1.0);
        }

        let mut height = context.area_size.height;
        if height <= 0.0 {
            height = (line_height * 24.0).max(200.0);
        }

        let char_width = measure_paragraph.max_intrinsic_width();
        let mut target_cols = if char_width > 0.0 {
            (context.area_size.width / char_width).floor() as u16
        } else {
            1
        };
        if target_cols == 0 {
            target_cols = 1;
        }
        let mut target_rows = if line_height > 0.0 {
            (height / line_height).floor() as u16
        } else {
            1
        };
        if target_rows == 0 {
            target_rows = 1;
        }

        self.handle.resize(target_rows, target_cols);

        Some((
            torin::prelude::Size2D::new(context.area_size.width.max(100.0), height),
            Rc::new(()),
        ))
    }

    fn render(&self, context: freya_core::element::RenderContext) {
        let area = context.layout_node.visible_area();

        let buffer = self.handle.read_buffer();

        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.bg);
        context.canvas.draw_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &paint,
        );

        let mut text_style = TextStyle::new();
        text_style.set_color(self.fg);
        text_style.set_font_families(&[self.font_family.as_str()]);
        text_style.set_font_size(self.font_size);

        let mut measure_builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());
        measure_builder.push_style(&text_style);
        measure_builder.add_text("W");
        let mut measure_paragraph = measure_builder.build();
        measure_paragraph.layout(f32::MAX);
        let char_width = measure_paragraph.max_intrinsic_width();
        let mut line_height = measure_paragraph.height();
        if line_height <= 0.0 || line_height.is_nan() {
            line_height = (self.font_size * 1.2).max(1.0);
        }

        let mut y = area.min_y();

        for (row_idx, row) in buffer.rows.iter().enumerate() {
            if y + line_height > area.max_y() {
                break;
            }

            for (col_idx, cell) in row.iter().enumerate() {
                if cell.is_wide_continuation() {
                    continue;
                }
                let cell_bg = map_vt100_bg_color(cell.bgcolor(), self.fg, self.bg);
                if cell_bg != self.bg {
                    let left = area.min_x() + (col_idx as f32) * char_width;
                    let top = y;
                    let cell_width = if cell.is_wide() {
                        char_width * 2.0
                    } else {
                        char_width
                    };
                    let right = left + cell_width;
                    let bottom = top + line_height;

                    let mut bg_paint = Paint::default();
                    bg_paint.set_style(PaintStyle::Fill);
                    bg_paint.set_color(cell_bg);
                    context
                        .canvas
                        .draw_rect(SkRect::new(left, top, right, bottom), &bg_paint);
                }
            }

            let mut builder =
                ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());
            builder.push_style(&text_style);
            for cell in row.iter() {
                let text = if cell.has_contents() {
                    cell.contents()
                } else if cell.is_wide_continuation() {
                    continue;
                } else {
                    " "
                };
                let mut cell_style = TextStyle::new();
                cell_style.set_color(map_vt100_fg_color(cell.fgcolor(), self.fg, self.bg));
                cell_style.set_font_families(&[self.font_family.as_str()]);
                cell_style.set_font_size(self.font_size);
                builder.push_style(&cell_style);
                builder.add_text(text);
            }
            let mut paragraph = builder.build();
            paragraph.layout(f32::MAX);
            paragraph.paint(context.canvas, (area.min_x(), y));

            if row_idx == buffer.cursor_row {
                let cursor_idx = buffer.cursor_col;
                let left = area.min_x() + (cursor_idx as f32) * char_width;
                let top = y;
                let right = left + char_width.max(1.0);
                let bottom = top + line_height.max(1.0);

                let mut cursor_paint = Paint::default();
                cursor_paint.set_style(PaintStyle::Fill);
                cursor_paint.set_color(self.fg);
                context
                    .canvas
                    .draw_rect(SkRect::new(left, top, right, bottom), &cursor_paint);

                let content = row
                    .get(cursor_idx)
                    .map(|cell| {
                        if cell.has_contents() {
                            cell.contents().to_string()
                        } else {
                            " ".to_string()
                        }
                    })
                    .unwrap_or_else(|| " ".to_string());

                let mut fg_text_style = TextStyle::new();
                fg_text_style.set_color(self.bg);
                fg_text_style.set_font_size(self.font_size);
                fg_text_style.set_font_families(&[self.font_family.as_str()]);
                let mut fg_builder = ParagraphBuilder::new(
                    &ParagraphStyle::default(),
                    context.font_collection.clone(),
                );
                fg_builder.push_style(&fg_text_style);
                fg_builder.add_text(&content);
                let mut fg_paragraph = fg_builder.build();
                fg_paragraph.layout((right - left).max(1.0));
                fg_paragraph.paint(context.canvas, (left, top));
            }

            y += line_height;
        }
    }
}

impl From<TerminalElement> for Element {
    fn from(value: TerminalElement) -> Self {
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(value),
            elements: Vec::new(),
        }
    }
}

/// User-facing Terminal component
#[derive(Clone, PartialEq)]
pub struct Terminal {
    handle: TerminalHandle,
    font_family: String,
    font_size: f32,
    fg: Color,
    bg: Color,
    layout: LayoutData,
    key: DiffKey,
}

impl Terminal {
    /// Create a terminal with a handle for interactive PTY
    pub fn with_handle(handle: TerminalHandle) -> Self {
        Self {
            handle,
            font_family: "Cascadia Code".to_string(),
            font_size: 14.,
            fg: (220, 220, 220).into(),
            bg: (10, 10, 10).into(),
            layout: LayoutData::default(),
            key: DiffKey::default(),
        }
    }

    /// Set the font family for the terminal
    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    /// Set the font size for the terminal
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Set the foreground and background colors
    pub fn colors(mut self, fg: impl Into<Color>, bg: impl Into<Color>) -> Self {
        self.fg = fg.into();
        self.bg = bg.into();
        self
    }
}

impl Component for Terminal {
    fn render(&self) -> impl IntoElement {
        TerminalElement::new(self.handle.clone())
            .font_family(self.font_family.clone())
            .font_size(self.font_size)
            .colors(self.fg, self.bg)
    }
}

impl KeyExt for Terminal {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for Terminal {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}
impl ContainerExt for Terminal {}
