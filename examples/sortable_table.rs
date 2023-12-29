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

impl_sortable_table_row! {
    pub struct Person {
        pub name: &'static str,
        pub dollars: i64,
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

impl Person {
    pub fn new(name: &'static str, dollars: i64) -> Self {
        Person { name, dollars }
    }
}

fn app(cx: Scope) -> Element {
    let rows = vec![
        Person::new("John Smith", 120),
        Person::new("Alice", -81),
        Person::new("John Doe", 18),
        Person::new("Just made a bank account", 0),
        Person::new("100 number guy 9", 1),
        Person::new("Richie Rich", 999_999_999_999),
        Person::new("Mose Schrute", 2),
        Person::new("Michael Scott", 2500),
        Person::new("Michael Scarn", 5000),
        Person::new("Prison Mike", -25000),
        Person::new("Gavin Belson", 4_500_000_000),
    ];

    let table = SortableTable::new(rows);

    render!(
        SortableTable {
            table: RefCell::new(table),
            default_sorted_column: PersonColumnPointer::Name,
            default_order_direction: OrderDirection::Down,
            alternate_colors: true,
        }
    )
}