use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use std::cmp::Ordering;
use std::fmt::Display;
use std::cell::RefCell;
use crate::*;

type Sorter<T> = Box<dyn Fn(&T, &T) -> Ordering>;

pub struct SortableTableHeader<T, TTitle> where TTitle: Display {
    title: TTitle,
    sorter: Sorter<T>,
}

impl<T, TTitle> SortableTableHeader<T, TTitle> where TTitle: Display {
    pub fn new(title: TTitle, sorter: Sorter<T>) -> Self {
        Self { title, sorter }
    }
}

// Define a struct for the sortable table
pub struct SortableTable<T, TTitle> where TTitle: Display {
    pub headers: Vec<SortableTableHeader<T, TTitle>>,
    pub rows: Vec<Vec<T>>,
}

impl<T, TTitle> SortableTable<T, TTitle> where TTitle: Display {
    pub fn new(headers: Vec<SortableTableHeader<T, TTitle>>, rows: Vec<Vec<T>>) -> Self {
        Self { headers, rows }
    }

    pub fn sort_by_header(&mut self, header_index: usize, reverse_sort: bool) -> Result<(), String> {
        let Some(header) = self.headers.get(header_index) else {
            return Err(format!("the header at requested index {header_index} does not exist"));
        };

        let sorter = &header.sorter;

        self.rows.sort_by(|a, b| {
            let Some(cell_a) = a.get(header_index) else {
                return Ordering::Equal;
            };
            let Some(cell_b) = b.get(header_index) else {
                return Ordering::Equal;
            };
            if reverse_sort {
                sorter(cell_b, cell_a)
            } else {
                sorter(cell_a, cell_b)
            }
        });

        Ok(())
    }

    pub fn push_header(&mut self, header: SortableTableHeader<T, TTitle>) {
        self.headers.push(header);
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.rows.push(row);
    }
}

#[derive(Props)]
pub struct SortableTableProps<T, TTitle> where TTitle: Display {
    pub table: RefCell<SortableTable<T, TTitle>>,
    pub default_order_direction: OrderDirection,
    #[props(default = false)]
    pub alternate_colors: bool,
}

#[allow(non_snake_case)]
pub fn SortableTable<T, TTitle>(
    cx: Scope<SortableTableProps<T, TTitle>>,
) -> Element where T: Display, TTitle: Display {
    let SortableTableProps { table, default_order_direction, alternate_colors } = &cx.props;
    let current_header = use_state(cx, || 0);
    let order_direction = use_state(cx, || *default_order_direction);

    let on_header_click = |new_header_i: usize| {
        if *current_header.get() == new_header_i {
            if *order_direction.get() == OrderDirection::Up {
                order_direction.set(OrderDirection::Down);
            } else {
                order_direction.set(OrderDirection::Up);
            }
        } else {
            current_header.set(new_header_i);
            order_direction.set(*default_order_direction);
        }
    };

    if *order_direction.get() == OrderDirection::Down {
        table.borrow_mut().sort_by_header(*current_header.get(), false).unwrap();
    } else {
        table.borrow_mut().sort_by_header(*current_header.get(), true).unwrap();
    }

    let table = table.borrow();

    render! {
        Table {
            columns: table.headers.len(),
            TableHead {
                TableRow {
                    for (header_i, header) in table.headers.iter().enumerate() {
                        TableCell {
                            key: "{header_i}",
                            order_direction: if *current_header.get() == header_i { Some(*order_direction.get()) } else { None },
                            onclick: move |_| on_header_click(header_i),
                            label {
                                header.title.to_string()
                            }
                        }
                    }
                }
            }
            TableBody {
                ScrollView {
                    for (row_i, row) in table.rows.iter().enumerate() {
                        TableRow {
                            key: "{row_i}",
                            alternate_colors: if *alternate_colors { row_i % 2 == 0 } else { false },
                            for (cell_i, cell) in row.iter().enumerate() {
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
                }
            }
        }
    }
}
