use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
};

use alacritty_terminal::{
    grid::Dimensions,
    term::{
        TermMode,
        cell::Cell,
    },
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
    Font,
    FontEdging,
    FontHinting,
    FontStyle,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    TextStyle,
};
use rustc_hash::FxHashMap;
use torin::prelude::Size2D;

use crate::{
    handle::TerminalHandle,
    rendering::{
        CachedRow,
        Renderer,
    },
};

/// Cached layout measurements and font for text drawing.
struct TerminalMeasure {
    char_width: f32,
    line_height: f32,
    baseline_offset: f32,
    font: Font,
    font_family: String,
    font_size: f32,
    row_cache: RefCell<FifoCache<u64, CachedRow>>,
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
        let scaled_font_size = self.font_size * context.scale_factor as f32;

        // Measure char width and line height using a reference glyph
        let mut builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());

        let mut style = TextStyle::new();
        style.set_font_size(scaled_font_size);
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
            let scale = context.scale_factor as f32;
            on_measured.call((char_width / scale, line_height / scale));
        }

        let typeface = context
            .font_collection
            .find_typefaces(&[&self.font_family], FontStyle::default())
            .into_iter()
            .next()
            .expect("Terminal font family not found");

        let mut font = Font::from_typeface(typeface, scaled_font_size);
        font.set_subpixel(true);
        font.set_edging(FontEdging::SubpixelAntiAlias);
        font.set_hinting(match scaled_font_size as u32 {
            0..=6 => FontHinting::Full,
            7..=12 => FontHinting::Normal,
            13..=24 => FontHinting::Slight,
            _ => FontHinting::None,
        });

        let baseline_offset = paragraph.alphabetic_baseline();

        Some((
            Size2D::new(context.area_size.width.max(100.0), height),
            Rc::new(TerminalMeasure {
                char_width,
                line_height,
                baseline_offset,
                font,
                font_family: self.font_family.clone(),
                font_size: scaled_font_size,
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

        let term = self.handle.term();
        let grid = term.grid();
        let columns = grid.columns();
        let screen_lines = grid.screen_lines();
        let display_offset = grid.display_offset();
        let total_scrollback = grid.history_size();
        let selection = term.selection.as_ref().and_then(|s| s.to_range(&*term));

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);

        let mut renderer = Renderer {
            canvas: context.canvas,
            paint: &mut paint,
            font: &measure.font,
            font_collection: context.font_collection,
            row_cache: &mut measure.row_cache.borrow_mut(),
            area,
            char_width: measure.char_width,
            line_height: measure.line_height,
            baseline_offset: measure.baseline_offset,
            foreground: self.foreground,
            background: self.background,
            selection_color: self.selection_color,
            font_family: &measure.font_family,
            font_size: measure.font_size,
            selection,
            display_offset,
        };

        renderer.render_background();

        // Reused row buffer so redraws don't allocate a `Vec<Vec<Cell>>`.
        let mut row: Vec<Cell> = Vec::with_capacity(columns);
        let mut display_iter = grid.display_iter();
        let mut y = area.min_y();
        for row_idx in 0..screen_lines {
            if y + measure.line_height > area.max_y() {
                break;
            }
            row.clear();
            row.extend(display_iter.by_ref().take(columns).map(|c| c.cell.clone()));
            renderer.render_row(row_idx, &row, y);
            y += measure.line_height;
        }

        if display_offset == 0 && term.mode().contains(TermMode::SHOW_CURSOR) {
            let cursor_point = grid.cursor.point;
            let cursor_y = area.min_y() + (cursor_point.line.0 as f32) * measure.line_height;
            if cursor_y + measure.line_height <= area.max_y() {
                renderer.render_cursor(&grid[cursor_point], cursor_y, cursor_point.column.0);
            }
        }

        if total_scrollback > 0 {
            renderer.render_scrollbar(display_offset, total_scrollback, screen_lines);
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
