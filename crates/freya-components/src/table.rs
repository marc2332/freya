use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    prelude::Alignment,
    size::Size,
};

use crate::{
    get_theme,
    icons::arrow::ArrowIcon,
    theming::component_themes::{
        TableTheme,
        TableThemePartial,
    },
};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum OrderDirection {
    Up,
    #[default]
    Down,
}

#[derive(PartialEq)]
pub struct TableArrow {
    pub order_direction: OrderDirection,
    key: DiffKey,
}

impl TableArrow {
    pub fn new(order_direction: OrderDirection) -> Self {
        Self {
            order_direction,
            key: DiffKey::None,
        }
    }
}

impl KeyExt for TableArrow {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for TableArrow {
    fn render(&self) -> impl IntoElement {
        let TableTheme { arrow_fill, .. } = get_theme!(None::<TableThemePartial>, table);
        let rotate = match self.order_direction {
            OrderDirection::Down => 0.,
            OrderDirection::Up => 180.,
        };
        ArrowIcon::new().rotate(rotate).fill(arrow_fill)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// TableHead props (manual)
#[derive(PartialEq, Default)]
pub struct TableHead {
    pub children: Vec<Element>,
    key: DiffKey,
}

impl TableHead {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ChildrenExt for TableHead {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for TableHead {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for TableHead {
    fn render(&self) -> impl IntoElement {
        rect().width(Size::fill()).children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq, Default)]
pub struct TableBody {
    pub children: Vec<Element>,
    key: DiffKey,
}

impl TableBody {
    pub fn new() -> Self {
        Self::default()
    }
}
impl ChildrenExt for TableBody {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for TableBody {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for TableBody {
    fn render(&self) -> impl IntoElement {
        rect().width(Size::fill()).children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq, Clone, Copy)]
enum TableRowState {
    Idle,
    Hovering,
}

#[derive(PartialEq, Default)]
pub struct TableRow {
    pub theme: Option<TableThemePartial>,
    pub children: Vec<Element>,
    key: DiffKey,
}

impl TableRow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ChildrenExt for TableRow {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for TableRow {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for TableRow {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, table);
        let mut state = use_state(|| TableRowState::Idle);
        let TableTheme {
            divider_fill,
            hover_row_background,
            row_background,
            ..
        } = theme;
        let background = if state() == TableRowState::Hovering {
            hover_row_background
        } else {
            row_background
        };

        rect()
            .on_pointer_enter(move |_| state.set(TableRowState::Hovering))
            .on_pointer_leave(move |_| state.set(TableRowState::Idle))
            .background(background)
            .child(
                rect()
                    .width(Size::fill())
                    .horizontal()
                    .children(self.children.clone()),
            )
            .child(
                rect()
                    .height(Size::px(1.))
                    .width(Size::fill())
                    .background(divider_fill),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
pub struct TableCell {
    pub children: Vec<Element>,
    /// optional press handler
    pub on_press: Option<EventHandler<Event<PressEventData>>>,
    /// optional visual order direction
    pub order_direction: Option<OrderDirection>,
    /// padding as typed Gaps
    pub padding: Gaps,
    /// height as typed Size
    pub height: Size,
    key: DiffKey,
}

impl ChildrenExt for TableCell {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Default for TableCell {
    fn default() -> Self {
        Self {
            children: vec![],
            on_press: None,
            order_direction: None,
            padding: Gaps::new_all(5.0),
            height: Size::px(35.0),
            key: DiffKey::None,
        }
    }
}

impl TableCell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, padding: Gaps) -> Self {
        self.padding = padding;
        self
    }

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.height = height.into();
        self
    }

    pub fn on_press(mut self, handler: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(handler.into());
        self
    }

    pub fn order_direction(mut self, dir: Option<OrderDirection>) -> Self {
        self.order_direction = dir;
        self
    }
}

impl KeyExt for TableCell {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for TableCell {
    fn render(&self) -> impl IntoElement {
        let config = use_try_consume::<TableConfig>().unwrap_or(TableConfig::new(1));
        let width_percent = 100.0 / (config.columns as f32);
        let mut container = rect()
            .overflow_mode(OverflowMode::Clip)
            .padding(self.padding)
            .width(Size::percent(width_percent))
            .main_align(Alignment::End)
            .cross_align(Alignment::Center)
            .height(self.height.clone())
            .horizontal();

        if let Some(on_press) = &self.on_press {
            let handler = on_press.clone();
            container = container.on_press(move |e| handler.call(e));
        }

        if let Some(order_direction) = self.order_direction {
            container = container.child(
                rect()
                    .margin(Gaps::new_all(10.0))
                    .width(Size::px(10.0))
                    .height(Size::px(10.0))
                    .child(TableArrow::new(order_direction)),
            );
        }

        container.children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
pub struct Table {
    pub height: Size,
    pub theme: Option<TableThemePartial>,
    pub columns: usize,
    pub children: Vec<Element>,
    key: DiffKey,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            height: Size::fill(),
            theme: None,
            columns: 1,
            children: vec![],
            key: DiffKey::None,
        }
    }
}

impl Table {
    pub fn new(columns: usize) -> Self {
        Self {
            columns,
            ..Default::default()
        }
    }

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.height = height.into();
        self
    }

    pub fn theme(mut self, theme: TableThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ChildrenExt for Table {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for Table {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

#[derive(Clone)]
pub struct TableConfig {
    pub columns: usize,
}

impl TableConfig {
    pub fn new(columns: usize) -> Self {
        Self { columns }
    }
}

impl Render for Table {
    fn render(&self) -> impl IntoElement {
        let TableTheme {
            background,
            corner_radius,
            divider_fill,
            color,
            ..
        } = get_theme!(&self.theme, table);

        provide_context(TableConfig::new(self.columns));

        rect()
            .overflow_mode(OverflowMode::Clip)
            .color(color)
            .background(background)
            .corner_radius(corner_radius)
            .height(self.height.clone())
            .border(
                Border::new()
                    .alignment(BorderAlignment::Outer)
                    .fill(divider_fill)
                    .width(1.0),
            )
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
