#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use freya::prelude::*;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};

fn main() {
    launch_with_props(app, "Sortable table", (500.0, 500.0));
}

#[derive(PartialOrd, PartialEq, Eq)]
enum PersonColumn {
    Name(&'static str),
    Dollars(i64),
}

impl Ord for PersonColumn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PersonColumn::Name(s1), PersonColumn::Name(s2)) => s1.cmp(s2),
            (PersonColumn::Dollars(n1), PersonColumn::Dollars(n2)) => n1.cmp(n2),
            (PersonColumn::Name(_), PersonColumn::Dollars(_)) => std::cmp::Ordering::Equal,
            (PersonColumn::Dollars(_), PersonColumn::Name(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl Display for PersonColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonColumn::Name(name) => f.write_str(name),
            PersonColumn::Dollars(dollars) => f.write_str(&dollars.to_string()),
        }
    }
}

fn app(cx: Scope) -> Element {
    let headers = vec![
        SortableTableHeader::new("Name", Box::new(|a: &PersonColumn, b: &PersonColumn| a.cmp(b))),
        SortableTableHeader::new("Dollars", Box::new(|a: &PersonColumn, b: &PersonColumn| a.cmp(b))),
    ];

    let rows = vec![
        vec![PersonColumn::Name("John Smith"), PersonColumn::Dollars(120)],
        vec![PersonColumn::Name("Alice"), PersonColumn::Dollars(-81)],
        vec![PersonColumn::Name("John Doe"), PersonColumn::Dollars(18)],
        vec![PersonColumn::Name("Just made a bank account"), PersonColumn::Dollars(0)],
        vec![PersonColumn::Name("100 number guy 9"), PersonColumn::Dollars(1)],
        vec![PersonColumn::Name("Richie Rich"), PersonColumn::Dollars(999_999_999_999)],
        vec![PersonColumn::Name("Mose Schrute"), PersonColumn::Dollars(2)],
        vec![PersonColumn::Name("Michael Scott"), PersonColumn::Dollars(2500)],
        vec![PersonColumn::Name("Michael Scarn"), PersonColumn::Dollars(5000)],
        vec![PersonColumn::Name("Prison Mike"), PersonColumn::Dollars(-25000)],
        vec![PersonColumn::Name("Gavin Belson"), PersonColumn::Dollars(4_500_000_000)],
    ];

    let table = SortableTable::new(headers, rows);

    render!(
        SortableTable {
            table: RefCell::new(table),
            default_order_direction: OrderDirection::Down,
        }
    )
}
