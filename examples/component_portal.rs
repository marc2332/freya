#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{
    ops::{
        Add,
        Sub,
    },
    time::Duration,
};

use freya::{
    animation::Function,
    prelude::*,
};
use rand::{
    rng,
    seq::SliceRandom,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(900., 800.)))
}

const SIZE: usize = 8;

#[derive(Debug, Clone)]
struct Cell {
    id: usize,
}

#[derive(Debug)]
struct Grid {
    size: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(size: usize) -> Self {
        let mut cells = Vec::with_capacity(size * size);
        for id in 0..size * size {
            cells.push(Cell { id });
        }
        Grid { cells, size }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();

        self.cells.shuffle(&mut rng);
    }

    pub fn order(&mut self) {
        *self = Self::new(self.size)
    }

    pub fn reverse(&mut self) {
        *self = Self::new(self.size);
        self.cells.reverse();
    }

    pub fn increase(&mut self) {
        *self = Self::new(self.size.add(1).min(15));
    }

    pub fn decrease(&mut self) {
        *self = Self::new(self.size.sub(1).max(1));
    }
}

fn app() -> impl IntoElement {
    let mut grid = use_state(|| Grid::new(SIZE));

    // Just some values to generate a different size and color based on the grid size
    let size = 600. / grid.read().size as f32;
    let color_ratio = 255. / (grid.read().size as f32 * grid.read().size as f32);

    rect()
        .spacing(12.)
        .expanded()
        .center()
        .child(
            rect()
                .layer(999)
                .center()
                .width(Size::fill())
                .position(Position::new_global().top(50.))
                .horizontal()
                .child(
                    Button::new()
                        .on_press(move |_| grid.write().shuffle())
                        .child("Shuffle"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| grid.write().order())
                        .child("Order"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| grid.write().reverse())
                        .child("Reverse"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| grid.write().increase())
                        .child("Increase"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| grid.write().decrease())
                        .child("Decrease"),
                ),
        )
        .child(
            rect()
                .spacing(6.)
                .children(grid.read().cells.chunks(grid.read().size).map(|row| {
                    rect()
                        .spacing(6.)
                        .horizontal()
                        .children(row.iter().map(|cell| {
                            Portal::new(cell.id)
                                .key(cell.id)
                                .width(Size::px(size))
                                .height(Size::px(size))
                                .function(Function::Expo)
                                .duration(Duration::from_millis(1000))
                                .child(
                                    rect()
                                        .width(Size::px(size))
                                        .height(Size::px(size))
                                        .corner_radius(32.)
                                        .color(Color::WHITE)
                                        .background((
                                            cell.id as u8,
                                            (cell.id as f32 * color_ratio) as u8,
                                            cell.id as u8,
                                        ))
                                        .center()
                                        .child(cell.id.to_string()),
                                )
                                .into()
                        }))
                        .into()
                })),
        )
}
