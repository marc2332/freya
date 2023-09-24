use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

#[allow(non_snake_case)]
#[inline_props]
fn TableArrow(cx: Scope, order_direction: OrderDirection) -> Element {
    let color = "rgb(40, 40, 40)";
    let rotate = match order_direction {
        OrderDirection::Down => "0",
        OrderDirection::Up => "180",
    };

    render!(svg {
        height: "10",
        width: "10",
        rotate: "{rotate}deg",
        svg_content: r#"
            <svg viewBox="0 0 18 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M7.18177 9.58579L0 2.40401L1.81823 0.585785L9 7.76756L16.1818 0.585787L18 2.40402L10.8182 9.58579L10.8185 9.58601L9.00023 11.4042L9 11.404L8.99977 11.4042L7.18154 9.58602L7.18177 9.58579Z" fill="{color}"  stroke="{color}" stroke-width="2"/>
            </svg>
        "#
    })
}

/// [`TableHead`] component properties.
#[derive(Props)]
pub struct TableHeadProps<'a> {
    /// The content of this table head.
    children: Element<'a>,
}

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

#[allow(non_snake_case)]
pub fn TableRow<'a>(cx: Scope<'a, TableRowProps<'a>>) -> Element {
    let background = if cx.props.alternate_colors {
        "rgb(240, 240, 240)"
    } else {
        "transparent"
    };
    render!(
        rect {
            direction: "horizontal",
            width: "100%",
            min_height: "35",
            background: "{background}",
            &cx.props.children
        }
        rect {
            height: "1",
            width: "100%",
            background: "rgb(200, 200, 200)"
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
    /// Show a line separator to the left of this TableCell.
    #[props(default = true, into)]
    separator: bool,
}

#[allow(non_snake_case)]
pub fn TableCell<'a>(cx: Scope<'a, TableCellProps<'a>>) -> Element {
    let config = cx.consume_context::<TableConfig>().unwrap();
    let width = 100.0 / config.columns as f32;

    render!(
        if cx.props.separator {
            rsx!(
                rect {
                    width: "1",
                    height: "35",
                    background: "rgb(200, 200, 200)"
                }
            )
        }
        rect {
            width: "0",
            height: "0",
            padding: "10",
            if let Some(Some(order_direction)) = &cx.props.order_direction {
                rsx!(
                    TableArrow {
                        order_direction: *order_direction
                    }
                )
            }
        }
        rect {
            overflow: "clip",
            padding: "5 25",
            width: "{width}%",
            display: "center",
            height: "35",
            align: "right",
            onclick: |e| {
                if let Some(onclick) = &cx.props.onclick {
                    onclick.call(e);
                }
            },
            &cx.props.children
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
}

#[allow(non_snake_case)]
pub fn Table<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    cx.provide_context(TableConfig {
        columns: cx.props.columns,
    });
    let height = &cx.props.height;

    render!(
        rect {
            overflow: "clip",
            background: "white",
            corner_radius: "6",
            shadow: "0 2 15 5 rgb(35, 35, 35, 70)",
            height: "{height}",
            &cx.props.children
        }
    )
}

#[derive(Clone)]
pub struct TableConfig {
    columns: usize,
}
