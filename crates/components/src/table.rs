use crate::icons::ArrowIcon;
use crate::theme::get_theme;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_get_theme, FontTheme, TableTheme, TableThemeWith};

#[allow(non_snake_case)]
#[component]
fn TableArrow(cx: Scope, order_direction: OrderDirection) -> Element {
    let theme = use_get_theme(cx);
    let TableTheme { arrow_fill, .. } = theme.table;
    let rotate = match order_direction {
        OrderDirection::Down => "0",
        OrderDirection::Up => "180",
    };

    render!( ArrowIcon { rotate: "{rotate}", fill: "{arrow_fill}" } )
}

/// [`TableHead`] component properties.
#[derive(Props)]
pub struct TableHeadProps<'a> {
    /// The content of this table head.
    pub children: Element<'a>,
}

/// `TableHead` component.
///
/// # Props
/// See [`TableHeadProps`].
///
#[allow(non_snake_case)]
pub fn TableHead<'a>(cx: Scope<'a, TableHeadProps<'a>>) -> Element {
    render!(
        rect { width: "100%", &cx.props.children }
    )
}

/// [`TableBody`] component properties.
#[derive(Props)]
pub struct TableBodyProps<'a> {
    /// The content of this table body.
    pub children: Element<'a>,
}

/// `TableBody` component.
///
/// # Props
/// See [`TableBodyProps`].
///
#[allow(non_snake_case)]
pub fn TableBody<'a>(cx: Scope<'a, TableBodyProps<'a>>) -> Element {
    render!(
        rect { width: "100%", &cx.props.children }
    )
}

/// [`TableRow`] component properties.
#[derive(Props)]
pub struct TableRowProps<'a> {
    /// Theme override.
    pub theme: Option<TableThemeWith>,
    /// The content of this row.
    children: Element<'a>,
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
pub fn TableRow<'a>(cx: Scope<'a, TableRowProps<'a>>) -> Element {
    let theme = get_theme!( cx, &cx.props.theme, table );
    let TableTheme {
        divider_fill,
        alternate_row_background,
        row_background,
        ..
    } = theme;
    let background = if cx.props.alternate_colors {
        alternate_row_background
    } else {
        row_background
    };

    render!(
        rect { direction: "horizontal", width: "100%", background: "{background}", &cx.props.children }
        rect { height: "1", width: "100%", background: "{divider_fill}" }
    )
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum OrderDirection {
    Up,
    #[default]
    Down,
}

/// [`TableCell`] component properties.
#[derive(Props)]
pub struct TableCellProps<'a> {
    /// The content of this cell.
    pub children: Element<'a>,
    /// Onclick event handler for the TableCell.
    pub onclick: Option<EventHandler<'a, MouseEvent>>,
    /// The direction in which this TableCell's column will be ordered.
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
pub fn TableCell<'a>(cx: Scope<'a, TableCellProps<'a>>) -> Element {
    let config = cx.consume_context::<TableConfig>().unwrap();
    let width = 100.0 / config.columns as f32;
    let TableCellProps {
        children,
        order_direction,
        padding,
        height,
        ..
    } = &cx.props;

    render!(
        rect {
            overflow: "clip",
            padding: "{padding}",
            width: "{width}%",
            main_align: "center",
            cross_align: "center",
            height: "{height}",
            align: "right",
            direction: "horizontal",
            onclick: |e| {
                if let Some(onclick) = &cx.props.onclick {
                    onclick.call(e);
                }
            },
            if let Some(order_direction) = &order_direction {
                rsx!(
                    rect {
                        margin: "10",
                        width: "10",
                        height: "10",
                        if let Some(order_direction) = &order_direction {
                            rsx!(
                                TableArrow {
                                    order_direction: *order_direction
                                }
                            )
                        }
                    }
                )
            }
            children
        }
    )
}

/// [`Table`] component properties.
#[derive(Props)]
pub struct TableProps<'a> {
    /// Theme override.
    pub theme: Option<TableThemeWith>,
    /// Number of columns used in the table.
    pub columns: usize,
    /// The content of the table.
    pub children: Element<'a>,
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
pub fn Table<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    let TableProps {
        theme,
        columns,
        children,
    } = cx.props;
    let TableTheme {
        background,
        height,
        corner_radius,
        shadow,
        font_theme: FontTheme { color },
        ..
    } = get_theme!( cx, theme, table );
    cx.provide_context(TableConfig { columns: *columns });

    render!(
        rect {
            overflow: "clip",
            color: "{color}",
            background: "{background}",
            corner_radius: "{corner_radius}",
            shadow: "{shadow}",
            height: "{height}",
            children
        }
    )
}

#[derive(Clone)]
pub struct TableConfig {
    columns: usize,
}
