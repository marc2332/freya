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
        EventHandlerType,
    },
    events::name::EventName,
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
use rustc_hash::FxHashMap;

use crate::{
    colors::{
        map_vt100_bg_color,
        map_vt100_fg_color,
    },
    handle::TerminalHandle,
};

/// Internal terminal rendering element
#[derive(Clone)]
pub struct Terminal {
    handle: TerminalHandle,
    layout_data: LayoutData,
    font_family: String,
    font_size: f32,
    fg: Color,
    bg: Color,
    selection_color: Color,
    on_measured: Option<EventHandler<(f32, f32)>>,
    event_handlers: FxHashMap<EventName, EventHandlerType>,
}

impl PartialEq for Terminal {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
            && self.font_size == other.font_size
            && self.font_family == other.font_family
            && self.fg == other.fg
            && self.bg == other.bg
            && self.event_handlers.len() == other.event_handlers.len()
    }
}

impl Terminal {
    pub fn new(handle: TerminalHandle) -> Self {
        Self {
            handle,
            layout_data: Default::default(),
            font_family: "Cascadia Code".to_string(),
            font_size: 14.,
            fg: (220, 220, 220).into(),
            bg: (10, 10, 10).into(),
            selection_color: (60, 179, 214, 0.3).into(),
            on_measured: None,
            event_handlers: FxHashMap::default(),
        }
    }

    /// Set the selection highlight color
    pub fn selection_color(mut self, selection_color: impl Into<Color>) -> Self {
        self.selection_color = selection_color.into();
        self
    }

    /// Set callback for when dimensions are measured (char_width, line_height)
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
        self.fg = foreground.into();
        self
    }

    pub fn background(mut self, background: impl Into<Color>) -> Self {
        self.bg = background.into();
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

impl ElementExt for Terminal {
    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(el) = (other.as_ref() as &dyn Any).downcast_ref::<Terminal>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.font_size != el.font_size
            || self.font_family != el.font_family
            || self.handle != el.handle
            || self.event_handlers.len() != el.event_handlers.len()
        {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout_data)
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
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

        // Store dimensions and notify callback
        if let Some(on_measured) = &self.on_measured {
            on_measured.call((char_width, line_height));
        }

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

            if let Some(selection) = &buffer.selection {
                let (start_row, start_col, end_row, end_col) = selection.normalized();

                if row_idx >= start_row && row_idx <= end_row {
                    let sel_start_col = if row_idx == start_row { start_col } else { 0 };
                    let sel_end_col = if row_idx == end_row {
                        end_col + 1
                    } else {
                        row.len()
                    };

                    for col_idx in sel_start_col..sel_end_col.min(row.len()) {
                        let left = area.min_x() + (col_idx as f32) * char_width;
                        let top = y;
                        let right = left + char_width;
                        let bottom = top + line_height;

                        let mut sel_paint = Paint::default();
                        sel_paint.set_style(PaintStyle::Fill);
                        sel_paint.set_color(self.selection_color);
                        context
                            .canvas
                            .draw_rect(SkRect::new(left, top, right, bottom), &sel_paint);
                    }
                }
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
            for cell in row.iter() {
                if cell.is_wide_continuation() {
                    continue;
                }
                let text = if cell.has_contents() {
                    cell.contents()
                } else {
                    " "
                };
                let mut cell_style = text_style.clone();
                cell_style.set_color(map_vt100_fg_color(cell.fgcolor(), self.fg, self.bg));
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
                            cell.contents()
                        } else {
                            " "
                        }
                    })
                    .unwrap_or(" ");

                let mut fg_text_style = text_style.clone();
                fg_text_style.set_color(self.bg);
                let mut fg_builder = ParagraphBuilder::new(
                    &ParagraphStyle::default(),
                    context.font_collection.clone(),
                );
                fg_builder.push_style(&fg_text_style);
                fg_builder.add_text(content);
                let mut fg_paragraph = fg_builder.build();
                fg_paragraph.layout((right - left).max(1.0));
                fg_paragraph.paint(context.canvas, (left, top));
            }

            y += line_height;
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
