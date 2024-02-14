use std::fmt::Display;
use std::result::Result;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use std::cmp::Ordering;
use std::cell::RefCell;
use freya_hooks::theme_with;
use freya_hooks::ScrollViewThemeWith;
use crate::*;
pub use paste::paste;

pub trait SortableTableColumnPointer: Copy where Self: 'static {
    const COLUMN_TITLES: &'static [(Self, &'static str)];
}

pub trait SortableTableRow {
    type Column;
    type ColumnPointer: SortableTableColumnPointer;
    fn compare(&self, with_other: &Self, by_column_pointer: Self::ColumnPointer) -> Ordering;
    fn get_column(&self, column_pointer: Self::ColumnPointer) -> Self::Column;
    fn get_columns(&self) -> Vec<Self::Column>;
}

#[macro_export]
macro_rules! impl_sortable_table_row {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident {
            $(
            $(#[$field_attrs:meta])*
            $field_vis:vis $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$attrs])*
        $vis struct $name {
            $(
            $(#[$field_attrs])*
            $field_vis $field_name: $field_type,
            )*
        }

        $crate::paste! {
            #[derive(PartialEq, PartialOrd, Eq, Ord)]
            $vis enum [<$name Column>] {
                $(
                [<$field_name:camel>] ($field_type),
                )*
            }

            #[derive(PartialEq, Clone, Copy)]
            $vis enum [<$name ColumnPointer>] {
                $(
                [<$field_name:camel>],
                )*
            }

            impl $crate::SortableTableColumnPointer for [<$name ColumnPointer>] {
                const COLUMN_TITLES: &'static [(Self, &'static str)] = &[$((Self::[<$field_name:camel>], stringify!([<$field_name:camel>])),)*];
            }

            impl $crate::SortableTableRow for $name {
                type Column = [<$name Column>];
                type ColumnPointer = [<$name ColumnPointer>];

                fn compare(&self, with_other: &Self, by_column_pointer: Self::ColumnPointer) -> ::core::cmp::Ordering {
                    self.get_column(by_column_pointer).cmp(&with_other.get_column(by_column_pointer))
                }

                fn get_column(&self, column_pointer: Self::ColumnPointer) -> Self::Column {
                    match column_pointer {
                        $(
                        [<$name ColumnPointer>] :: [<$field_name:camel>] => [<$name Column>] :: [<$field_name:camel>] (self.$field_name),
                        )*
                    }
                }

                fn get_columns(&self) -> Vec<Self::Column> {
                    vec![$([<$name Column>] :: [<$field_name:camel>] (self.$field_name),)*]
                }
            }
        }
    };
}

// impl_row! {
//     pub struct Person {
//         pub name: &'static str,
//         pub dollars: i64,
//     }
// }

pub struct SortableTable<R> where R: SortableTableRow {
    pub rows: Vec<R>,
}

impl<R> SortableTable<R> where R: SortableTableRow {
    pub fn new(rows: Vec<R>) -> Self {
        Self { rows }
    }

    pub fn sort_by(&mut self, column_pointer: R::ColumnPointer, reverse_sort: bool) -> Result<(), String> {
        self.rows.sort_by(|a, b| {
            if reverse_sort {
                b.compare(a, column_pointer)
            } else {
                a.compare(b, column_pointer)
            }
        });

        Ok(())
    }
}

pub enum SortableTableType {
    NonScrollable,
    Scrollable,
    ScrollableVirtualized,
}

#[derive(Props)]
pub struct SortableTableProps<R> where R: SortableTableRow {
    pub table: RefCell<SortableTable<R>>,
    pub default_order_direction: OrderDirection,
    pub default_sorted_column: R::ColumnPointer,
    #[props(default = false)]
    pub alternate_colors: bool,
    #[props(default = SortableTableType::NonScrollable)]
    pub r#type: SortableTableType,
}

#[allow(non_snake_case)]
pub fn SortableTable<R>(
    cx: Scope<SortableTableProps<R>>,
) -> Element where R: SortableTableRow, R::Column: Display, R::ColumnPointer: PartialEq {
    let SortableTableProps {
        table,
        default_order_direction,
        default_sorted_column,
        alternate_colors,
        r#type
    } = &cx.props;
    let current_sorted_column = use_state(cx, || *default_sorted_column);
    let order_direction = use_state(cx, || *default_order_direction);

    let on_header_click = |new_sorted_column: R::ColumnPointer| {
        if *current_sorted_column.get() == new_sorted_column {
            if *order_direction.get() == OrderDirection::Up {
                order_direction.set(OrderDirection::Down);
            } else {
                order_direction.set(OrderDirection::Up);
            }
        } else {
            current_sorted_column.set(new_sorted_column);
            order_direction.set(*default_order_direction);
        }
    };

    if *order_direction.get() == OrderDirection::Down {
        table.borrow_mut().sort_by(*current_sorted_column.get(), false).unwrap();
    } else {
        table.borrow_mut().sort_by(*current_sorted_column.get(), true).unwrap();
    }

    let table_ref = table.borrow();

    let rows_rsx = rsx! {
        for (row_i, row) in table_ref.rows.iter().enumerate() {
            TableRow {
                key: "{row_i}",
                alternate_colors: if *alternate_colors { row_i % 2 == 0 } else { false },
                for (cell_i, cell) in row.get_columns().iter().enumerate() {
                    TableCell {
                        key: "{cell_i}",
                        label {
                            width: "100%",
                            "{cell}"
                        }
                    }
                }
            }
        }
    };

    let table_ref = table.borrow();

    render! {
        Table {
            columns: R::ColumnPointer::COLUMN_TITLES.len(),
            TableHead {
                TableRow {
                    for (title_i, (pointer, title)) in R::ColumnPointer::COLUMN_TITLES.iter().enumerate() {
                        TableCell {
                            key: "{title_i}",
                            order_direction: if *current_sorted_column.get() == *pointer { Some(*order_direction.get()) } else { None },
                            onclick: move |_| on_header_click(*pointer),
                            label {
                                title.to_string()
                            }
                        }
                    }
                }
            }
            TableBody {
                match r#type {
                    SortableTableType::NonScrollable => rows_rsx,
                    SortableTableType::Scrollable => rsx! {ScrollView { rows_rsx } },
                    SortableTableType::ScrollableVirtualized => rsx! {
                        VirtualScrollView {
                            theme: theme_with!(ScrollViewTheme {
                                height: "fill".into(),
                            }),
                            length: table_ref.rows.len(),
                            item_size: 25.0,
                            builder_values: table,
                            direction: "vertical",
                            builder: Box::new(|(_k, i, _cx, values)| {
                                let values = values.as_ref().unwrap();
                                let table = values.borrow();
                                rsx! {
                                    TableRow {
                                        key: "{i}",
                                        for (cell_i, cell) in table.rows[i].get_columns().iter().enumerate() {
                                            TableCell {
                                                key: "{cell_i}",
                                                height: "25",
                                                label {
                                                    width: "100%",
                                                    "{cell}"
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    },
                }
            }
        }
    }
}