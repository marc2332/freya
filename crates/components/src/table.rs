use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

/// [`TableHead`] component properties.
#[derive(Props)]
pub struct TableHeadProps<'a> {
    children: Element<'a>
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
    children: Element<'a>
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
    children: Element<'a>
}

#[allow(non_snake_case)]
pub fn TableRow<'a>(cx: Scope<'a, TableRowProps<'a>>) -> Element {
    render!(
        rect {
            direction: "horizontal",
            width: "100%",
            min_height: "50",
            &cx.props.children
        }
        rect {
            height: "1",
            width: "100%",
            background: "rgb(200, 200, 200)"
        }
    )
}

/// [`TableCell`] component properties.
#[derive(Props)]
pub struct TableCellProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn TableCell<'a>(cx: Scope<'a, TableCellProps<'a>>) -> Element {
    let config = cx.consume_context::<TableConfig>().unwrap();
    let width = 100.0 / config.columns as f32;

    render!(
        rect {
            overflow: "clip",
            padding: "5 25",
            width: "{width}%",
            display: "center",
            height: "50",
            align: "right",
            &cx.props.children
        }
    )
}

/// [`Table`] component properties.
#[derive(Props)]
pub struct TableProps<'a> {
    columns: usize,
    children: Element<'a>
}

#[allow(non_snake_case)]
pub fn Table<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    cx.provide_context(TableConfig {
        columns: cx.props.columns
    });

    render!(
        rect {
            overflow: "clip",
            background: "white",
            corner_radius: "6",
            shadow: "0 2 15 5 rgb(35, 35, 35, 70)",
            &cx.props.children
        }
    )
}

#[derive(Clone)]
pub struct TableConfig {
    columns: usize,
}
