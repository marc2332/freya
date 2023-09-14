#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{cmp::Ordering, fmt::Display};

use freya::prelude::*;
use itertools::Itertools;

fn main() {
    launch(app);
}

#[derive(PartialEq)]
enum OrderBy {
    None,
    Name,
    OtherName,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::None => f.write_str("Nothing"),
            OrderBy::Name => f.write_str("Name"),
            OrderBy::OtherName => f.write_str("Other Name"),
        }
    }
}

fn app(cx: Scope) -> Element {
    let order = use_state(cx, || OrderBy::None);
    let data = use_state(cx, || {
        vec![
            vec!["test".to_string(), "Just some data".to_string()],
            vec!["test".to_string(), "aaaaaaaa".to_string()],
            vec!["Nice!".to_string(), "Awesome!!".to_string()],
            vec!["test".to_string(), "ccccc".to_string()],
            vec!["Woah".to_string(), "2222222".to_string()],
            vec!["Nice".to_string(), "hola".to_string()],
            vec!["Rust".to_string(), "hello".to_string()],
            vec!["is".to_string(), "hi!".to_string()],
            vec!["really nice!".to_string(), "test".to_string()],
        ]
    });

    let filtered_data = data.iter().sorted_by(|a, b| match *order.get() {
        OrderBy::None => Ordering::Equal,
        OrderBy::Name => Ord::cmp(&a[0], &b[0]),
        OrderBy::OtherName => Ord::cmp(&a[1], &b[1]),
    });

    render!(
        rect {
            padding: "10",
            label {
                height: "25",
                "Odering by {order}"
            }
            Table {
                columns: 2,
                TableHead {
                    TableRow {
                        TableCell {
                            onclick: |_| {
                                if *order.get() == OrderBy::Name {
                                    order.set(OrderBy::None)
                                } else {
                                    order.set(OrderBy::Name)
                                }
                            },
                            label {
                                width: "100%",
                                align: "center",
                                "Name"
                            }
                        }
                        TableCell {
                            onclick: |_| {
                                if *order.get() == OrderBy::OtherName {
                                    order.set(OrderBy::None)
                                } else {
                                    order.set(OrderBy::OtherName)
                                }
                            },
                            label {
                                width: "100%",
                                align: "center",
                                "Other name"
                            }
                        }
                    }
                }
                TableBody {
                    ScrollView {
                        for (i, items) in filtered_data.enumerate() {
                            TableRow {
                                key: "{i}",
                                alternate_colors: i % 2 == 0,
                                for (n, item) in items.iter().enumerate() {
                                    TableCell {
                                        key: "{n}",
                                        label {
                                            width: "100%",
                                            align: "right",
                                            "{item}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
