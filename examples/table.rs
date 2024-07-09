#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fmt::Display;

use freya::prelude::*;
use itertools::{
    Either,
    Itertools,
};

fn main() {
    launch(app);
}

#[derive(PartialEq, Clone)]
enum OrderBy {
    Name,
    Type,
    Rank,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Name => f.write_str("Name"),
            OrderBy::Type => f.write_str("Type"),
            OrderBy::Rank => f.write_str("Rank"),
        }
    }
}

fn app() -> Element {
    let mut order_direction = use_signal(|| OrderDirection::Down);
    let mut order = use_signal(|| OrderBy::Name);
    let data = use_signal(|| {
        vec![
            vec!["Zeus".to_owned(), "Sky".to_owned(), "01".to_owned()],
            vec!["Poseidon".to_owned(), "Sea".to_owned(), "03".to_owned()],
            vec!["Ares".to_owned(), "War".to_owned(), "08".to_owned()],
            vec!["Aphrodite".to_owned(), "Love".to_owned(), "10".to_owned()],
            vec!["Hera".to_owned(), "Women".to_owned(), "02".to_owned()],
            vec!["Demeter".to_owned(), "Harvest".to_owned(), "04".to_owned()],
            vec!["Athena".to_owned(), "Strategy".to_owned(), "05".to_owned()],
            vec!["Apollo".to_owned(), "Sun".to_owned(), "06".to_owned()],
            vec!["Artemis".to_owned(), "Hunt".to_owned(), "07".to_owned()],
            vec!["Hephaestus".to_owned(), "Fire".to_owned(), "09".to_owned()],
            vec!["Hermes".to_owned(), "Messenger".to_owned(), "11".to_owned()],
            vec!["Dionysus".to_owned(), "Wine".to_owned(), "12".to_owned()],
        ]
    });
    let columns = use_hook(|| {
        vec![
            ("Name", OrderBy::Name),
            ("Type", OrderBy::Type),
            ("Rank", OrderBy::Rank),
        ]
    });
    let data = data.read();

    let filtered_data = {
        let filtered_data = data.iter().sorted_by(|a, b| match *order.read() {
            OrderBy::Name => Ord::cmp(&a[0].to_lowercase(), &b[0].to_lowercase()),
            OrderBy::Type => Ord::cmp(&a[1].to_lowercase(), &b[1].to_lowercase()),
            OrderBy::Rank => Ord::cmp(&a[2].to_lowercase(), &b[2].to_lowercase()),
        });

        if *order_direction.read() == OrderDirection::Down {
            Either::Left(filtered_data)
        } else {
            Either::Right(filtered_data.rev())
        }
    };

    let mut on_column_head_click = move |column_order: &OrderBy| {
        // Change order diection
        if &*order.read() == column_order {
            if *order_direction.read() == OrderDirection::Up {
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

    rsx!(
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
                        for (n, (text, order_by)) in columns.into_iter().enumerate() {
                            TableCell {
                                key: "{n}",
                                order_direction: if *order.read() == order_by { Some(*order_direction.read()) } else { None },
                                onclick: move |_| on_column_head_click(&order_by),
                                label {
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
                                        label {
                                            width: "100%",
                                            text_align: "right",
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
