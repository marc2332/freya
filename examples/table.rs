#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fmt::Display;

use freya::prelude::*;
use itertools::{Either, Itertools};

fn main() {
    launch(app);
}

#[derive(PartialEq, Clone)]
enum OrderBy {
    Name,
    OtherName,
    MoreData,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Name => f.write_str("Name"),
            OrderBy::OtherName => f.write_str("Other Name"),
            OrderBy::MoreData => f.write_str("More Data"),
        }
    }
}

fn app(cx: Scope) -> Element {
    let order_direction = use_state(cx, || OrderDirection::Down);
    let order = use_state(cx, || OrderBy::Name);
    let data = use_state(cx, || {
        vec![
            vec![
                "aaaa".to_string(),
                "bbbb".to_string(),
                "111".to_string(),
            ],
            vec![
                "bbbb".to_string(),
                "aaaa".to_string(),
                "333".to_string(),
            ],
            vec![
                "wwww".to_string(),
                "777".to_string(),
                "ccc".to_string(),
            ],
            vec![
                "dddd".to_string(),
                "222".to_string(),
                "111".to_string(),
            ],
            vec![
                "hhhh".to_string(),
                "444".to_string(),
                "aaa".to_string(),
            ],
            vec![
                "555".to_string(),
                "ffff".to_string(),
                "zzzz".to_string(),
            ],
            vec![
                "llll".to_string(),
                "999".to_string(),
                "eeee".to_string(),
            ],
            vec![
                "abcd".to_string(),
                "987".to_string(),
                "wwww".to_string(),
            ],
            vec![
                "rrrr".to_string(),
                "333".to_string(),
                "888".to_string(),
            ],
        ]
    });
    let columns = cx.use_hook(|| {
        vec![
            ("Name", OrderBy::Name),
            ("OtherName", OrderBy::OtherName),
            ("MoreData", OrderBy::MoreData),
        ]
    });

    let filtered_data = {
        let filtered_data = data.iter().sorted_by(|a, b| match *order.get() {
            OrderBy::Name => Ord::cmp(&a[0].to_lowercase(), &b[0].to_lowercase()),
            OrderBy::OtherName => Ord::cmp(&a[1].to_lowercase(), &b[1].to_lowercase()),
            OrderBy::MoreData => Ord::cmp(&a[2].to_lowercase(), &b[2].to_lowercase()),
        });

        if *order_direction.get() == OrderDirection::Down {
            Either::Left(filtered_data)
        } else {
            Either::Right(filtered_data.rev())
        }
    };

    let on_column_head_click = |column_order: &OrderBy| {
        // Change order diection
        if order.get() == column_order {
            if *order_direction.get() == OrderDirection::Up {
                order_direction.set(OrderDirection::Down)
            } else {
                order_direction.set(OrderDirection::Up)
            }
        // Change order column
        } else {
            order.set(column_order.clone());
            order_direction.set(OrderDirection::default())
        }
    };

    render!(
        rect {
            padding: "10",
            label {
                height: "25",
                "Ordering by {order}"
            }
            Table {
                columns: 3,
                TableHead {
                    TableRow {
                        for (n, (text, order_by)) in columns.iter().enumerate() {
                            TableCell {
                                key: "{n}",
                                separator: false,
                                order_direction: if *order.get() == *order_by { Some(*order_direction.get()) } else { None },
                                onclick: move  |_| on_column_head_click(order_by),
                                label {
                                    width: "100%",
                                    align: "center",
                                    "{text}"
                                }
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
                                        separator: n > 0,
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
