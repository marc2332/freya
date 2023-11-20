use crate::icons::ArrowIcon;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_get_theme, FontTheme, TableTheme};

#[allow(non_snake_case)]
#[inline_props]
fn TableArrow(cx: Scope, order_direction: OrderDirection) -> Element {
    let theme = use_get_theme(cx);
    let TableTheme { arrow_fill, .. } = theme.table;
    let rotate = match order_direction {
        OrderDirection::Down => "0",
        OrderDirection::Up => "180",
    };

    render!(ArrowIcon {
        rotate: "{rotate}",
        fill: "{arrow_fill}"
    })
}

/// [`TableHead`] component properties.
#[derive(Props)]
pub struct TableHeadProps<'a> {
    /// The content of this table head.
    children: Element<'a>,
}

/// `TableHead` component.
///
/// # Props
/// See [`TableHeadProps`].
///
#[allow(non_snake_case)]
pub fn TableHead<'a>(cx: Scope<'a, TableHeadProps<'a>>) -> Element {
    render!(
        rect {
            width: "100%",
            &cx.props.children
        }
    )
}

/// [`TableBody`] component properties.
#[derive(Props)]
pub struct TableBodyProps<'a> {
    /// The content of this table body.
    children: Element<'a>,
}

/// `TableBody` component.
///
/// # Props
/// See [`TableBodyProps`].
///
#[allow(non_snake_case)]
pub fn TableBody<'a>(cx: Scope<'a, TableBodyProps<'a>>) -> Element {
    render!(
        rect {
            width: "100%",
            &cx.props.children
        }
    )
}

/// [`TableRow`] component properties.
#[derive(Props)]
pub struct TableRowProps<'a> {
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
    let theme = use_get_theme(cx);
    let TableTheme {
        divider_fill,
        alternate_row_background,
        row_background,
        ..
    } = theme.table;
    let background = if cx.props.alternate_colors {
        alternate_row_background
    } else {
        row_background
    };
    render!(
        rect {
            direction: "horizontal",
            width: "100%",
            background: "{background}",
            &cx.props.children
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
    Up,
    #[default]
    Down,
}

/// [`TableCell`] component properties.
#[derive(Props)]
pub struct TableCellProps<'a> {
    /// The content of this cell.
    children: Element<'a>,
    /// Onclick event handler for the TableCell.
    onclick: Option<EventHandler<'a, MouseEvent>>,
    /// The direction in which this TableCell's column will be ordered.
    #[props(into)]
    order_direction: Option<Option<OrderDirection>>,
    /// The padding of the cell.
    #[props(default = "5 25".to_string(), into)]
    padding: String,
    /// The height of the cell.
    #[props(default = "35".to_string(), into)]
    height: String,
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
    /// Number of columns used in the table.
    columns: usize,
    /// The content of the table.
    children: Element<'a>,
    /// The height of the table.
    #[props(default = "auto".to_string(), into)]
    height: String,
    /// The corner radius of the table.
    #[props(default = "6".to_string(), into)]
    corner_radius: String,
    /// The drop shadow of the table.
    #[props(default = "0 2 15 5 rgb(35, 35, 35, 70)".to_string(), into)]
    shadow: String,
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
    let theme = use_get_theme(cx);
    let TableTheme {
        background,
        font_theme: FontTheme { color },
        ..
    } = theme.table;
    cx.provide_context(TableConfig {
        columns: cx.props.columns,
    });
    let height = &cx.props.height;
    let corner_radius = &cx.props.corner_radius;
    let shadow = &cx.props.shadow;

    render!(
        rect {
            overflow: "clip",
            color: "{color}",
            background: "{background}",
            corner_radius: "{corner_radius}",
            shadow: "{shadow}",
            height: "{height}",
            &cx.props.children
        }
    )
}

#[derive(Clone)]
pub struct TableConfig {
    columns: usize,
}
