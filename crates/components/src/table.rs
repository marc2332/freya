use crate::icons::ArrowIcon;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_applied_theme, use_get_theme, FontTheme, TableTheme, TableThemeWith};

#[allow(non_snake_case)]
#[component]
fn TableArrow(order_direction: OrderDirection) -> Element {
    let theme = use_get_theme();
    let TableTheme { arrow_fill, .. } = theme.table;
    let rotate = match order_direction {
        OrderDirection::Down => "0",
        OrderDirection::Up => "180",
    };

    rsx!(ArrowIcon {
        rotate: "{rotate}",
        fill: "{arrow_fill}"
    })
}

/// [`TableHead`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    /// The content of this table head.
    pub children: Element,
}

/// `TableHead` component.
///
/// # Props
/// See [`TableHeadProps`].
///
#[allow(non_snake_case)]
pub fn TableHead(TableHeadProps { children }: TableHeadProps) -> Element {
    rsx!(
        rect { width: "100%", {children} }
    )
}

/// [`TableBody`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    /// The content of this table body.
    pub children: Element,
}

/// `TableBody` component.
///
/// # Props
/// See [`TableBodyProps`].
///
#[allow(non_snake_case)]
pub fn TableBody(TableBodyProps { children }: TableBodyProps) -> Element {
    rsx!(
        rect { width: "100%", {children} }
    )
}

/// [`TableRow`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    /// Theme override.
    pub theme: Option<TableThemeWith>,
    /// The content of this row.
    children: Element,
    /// Show the row with a different background, this allows to have a zebra-style table.
    #[props(default = false)]
    alternate_colors: bool,
}

/// `TableRow` component.
///
/// # Props
/// See [`TableRowProps`].
///
/// # Styling
/// Inherits the [`TableTheme`](freya_hooks::TableTheme) theme.
///
#[allow(non_snake_case)]
pub fn TableRow(
    TableRowProps {
        theme,
        children,
        alternate_colors,
    }: TableRowProps,
) -> Element {
    let theme = use_applied_theme!(&theme, table);
    let TableTheme {
        divider_fill,
        alternate_row_background,
        row_background,
        ..
    } = theme;
    let background = if alternate_colors {
        alternate_row_background
    } else {
        row_background
    };

    rsx!(
        rect {
            direction: "horizontal",
            width: "100%",
            background: "{background}",
            {children}
        }
        rect {
            height: "1",
            width: "100%",
            background: "{divider_fill}"
        }
    )
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum OrderDirection {
    /// Alternatively: descending.
    Up,
    /// Alternatively: ascending.
    #[default]
    Down,
}

/// [`TableCell`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    /// The content of this cell.
    pub children: Element,
    /// Onclick event handler for the TableCell.
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// The direction in which this TableCell's column will be ordered.
    ///
    /// **This is only a visual change (it changes the icon), you need to sort stuff yourself.**
    #[props(into)]
    pub order_direction: Option<Option<OrderDirection>>,
    /// The padding of the cell.
    #[props(default = "5 25".to_string(), into)]
    pub padding: String,
    /// The height of the cell.
    #[props(default = "35".to_string(), into)]
    pub height: String,
}

/// `TableCell` component.
///
/// # Props
/// See [`TableCellProps`].
///
#[allow(non_snake_case)]
pub fn TableCell(props: TableCellProps) -> Element {
    let config = consume_context::<TableConfig>();
    let width = 100.0 / config.columns as f32;
    let TableCellProps {
        children,
        order_direction,
        padding,
        height,
        ..
    } = &props;

    rsx!(
        rect {
            overflow: "clip",
            padding: "{padding}",
            width: "{width}%",
            main_align: "center",
            cross_align: "center",
            height: "{height}",
            text_align: "right",
            direction: "horizontal",
            onclick: move |e| {
                if let Some(onclick) = &props.onclick {
                    onclick.call(e);
                }
            },
            if let Some(order_direction) = &order_direction {
                rect {
                    margin: "10",
                    width: "10",
                    height: "10",
                    if let Some(order_direction) = &order_direction {
                        TableArrow {
                            order_direction: *order_direction
                        }
                    }
                }
            }
            {children}
        }
    )
}

/// [`Table`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Theme override.
    pub theme: Option<TableThemeWith>,
    /// Number of columns used in the table.
    pub columns: usize,
    /// The content of the table.
    pub children: Element,
}

/// `Table` component.
///
/// # Props
/// See [`TableProps`].
///
/// # Styling
/// Inherits the [`TableTheme`](freya_hooks::TableTheme) theme.
///
#[allow(non_snake_case)]
pub fn Table(
    TableProps {
        theme,
        columns,
        children,
    }: TableProps,
) -> Element {
    let TableTheme {
        background,
        height,
        corner_radius,
        shadow,
        font_theme: FontTheme { color },
        ..
    } = use_applied_theme!(&theme, table);
    provide_context(TableConfig { columns });

    rsx!(rect {
        overflow: "clip",
        color: "{color}",
        background: "{background}",
        corner_radius: "{corner_radius}",
        shadow: "{shadow}",
        height: "{height}",
        {children}
    })
}

#[derive(Clone)]
pub struct TableConfig {
    columns: usize,
}
