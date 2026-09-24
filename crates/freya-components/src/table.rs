use freya_core::prelude::*;
use torin::{
    content::Content,
    gaps::Gaps,
    node::Node,
    prelude::Alignment,
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
};

define_theme! {
    for = Table;
    theme_field = theme;

    %[component]
    pub Table {
        %[fields]
        background: Color,
        hover_row_background: Color,
        row_background: Color,
        divider_fill: Color,
        corner_radius: CornerRadius,
        color: Color,
    }
}

/// Resolved theme and column widths a [Table] shares with its rows.
#[derive(Clone)]
pub struct TableConfig {
    pub theme: TableTheme,
    pub column_widths: Option<Vec<Size>>,
}

#[derive(PartialEq)]
pub struct TableRow {
    pub children: Vec<Element>,
    layout: LayoutData,
    key: DiffKey,
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            children: vec![],
            layout: Node {
                width: Size::fill(),
                ..Default::default()
            }
            .into(),
            key: DiffKey::None,
        }
    }
}

impl LayoutExt for TableRow {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for TableRow {}

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

impl Component for TableRow {
    fn render(&self) -> impl IntoElement {
        let TableConfig {
            theme,
            column_widths,
        } = use_try_consume::<TableConfig>().unwrap_or_else(|| TableConfig {
            theme: get_theme!(None::<TableThemePartial>, TableThemePreference, "table"),
            column_widths: None,
        });
        let mut hovering = use_state(|| false);
        let background = if hovering() {
            theme.hover_row_background
        } else {
            theme.row_background
        };

        rect()
            .layout(self.layout.clone())
            .horizontal()
            .content(Content::Flex)
            .cross_align(Alignment::Center)
            .background(background)
            .border(Border::new().fill(theme.divider_fill).width(BorderWidth {
                bottom: 1.,
                ..Default::default()
            }))
            .on_pointer_enter(move |_| hovering.set(true))
            .on_pointer_leave(move |_| hovering.set(false))
            .children(self.children.iter().enumerate().map(|(index, child)| {
                let width = column_widths
                    .as_ref()
                    .and_then(|widths| widths.get(index).cloned())
                    .unwrap_or_else(|| Size::flex(1.));

                rect()
                    .width(width)
                    .overflow(Overflow::Clip)
                    .padding(Gaps::new_all(5.0))
                    .horizontal()
                    .main_align(Alignment::End)
                    .cross_align(Alignment::Center)
                    .child(child.clone())
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// A table component with rows and columns.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Table::new()
///         .child(TableRow::new().child("Header 1").child("Header 2"))
///         .child(TableRow::new().child("Data 1").child("Data 2"))
///         .child(TableRow::new().child("Data 3").child("Data 4"))
/// }
/// ```
///
/// See the [interactive components demo](https://freyaui.dev/demo).
#[derive(PartialEq, Default)]
pub struct Table {
    pub theme: Option<TableThemePartial>,
    pub column_widths: Option<Vec<Size>>,
    pub children: Vec<Element>,
    layout: LayoutData,
    key: DiffKey,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn theme(mut self, theme: TableThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Set custom widths for each column.
    ///
    /// Accepts any [Size], defaults to [Size::Flex].
    pub fn column_widths(mut self, widths: impl Into<Vec<Size>>) -> Self {
        self.column_widths = Some(widths.into());
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

impl LayoutExt for Table {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for Table {}

impl Component for Table {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, TableThemePreference, "table");
        provide_context(TableConfig {
            theme: theme.clone(),
            column_widths: self.column_widths.clone(),
        });

        rect()
            .layout(self.layout.clone())
            .overflow(Overflow::Clip)
            .color(theme.color)
            .background(theme.background)
            .corner_radius(theme.corner_radius)
            .border(
                Border::new()
                    .alignment(BorderAlignment::Outer)
                    .fill(theme.divider_fill)
                    .width(1.0),
            )
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
