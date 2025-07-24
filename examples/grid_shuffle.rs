use std::time::Duration;

use freya::prelude::*;
use rand::{
    seq::SliceRandom,
    thread_rng,
};

const SIZE: usize = 8;

#[derive(Debug, Clone)]
struct Cell {
    id: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn new() -> Self {
        let mut cells = Vec::new();
        for i in 0..SIZE {
            for j in 0..SIZE {
                cells.push(Cell { id: i * SIZE + j });
            }
        }
        Grid { cells }
    }

    pub fn suffle(&mut self) {
        let mut rng = thread_rng();

        self.cells.shuffle(&mut rng);
    }

    pub fn order(&mut self) {
        *self = Self::new()
    }

    pub fn reverse(&mut self) {
        *self = Self::new();
        self.cells.reverse();
    }
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut grid = use_signal(Grid::new);
    rsx!(
        rect {
            spacing: "12",
            main_align: "center",
            cross_align: "center",
            width: "fill",
            height: "fill",
            GlobalAnimatedPositionProvider::<usize> {
                rect {
                    direction: "horizontal",
                    Button {
                        onpress: move |_| grid.write().suffle(),
                        label {
                            "Shuffle"
                        }
                    }
                    Button {
                        onpress: move |_| grid.write().order(),
                        label {
                            "Order"
                        }
                    }
                    Button {
                        onpress: move |_| grid.write().reverse(),
                        label {
                            "Reverse"
                        }
                    }
                }
                rect {
                    spacing: "6",
                    for row in grid.read().cells.chunks(SIZE) {
                        rect {
                            direction: "horizontal",
                            spacing: "6",
                            for cell in row {
                                GlobalAnimatedPosition::<usize> {
                                    key: "{cell.id:?}",
                                    width: "80",
                                    height: "80",
                                    function: Function::Expo,
                                    duration: Duration::from_millis(500),
                                    id: cell.id,
                                    rect {
                                        width: "80",
                                        height: "80",
                                        background: "rgb({cell.id * 1}, {cell.id * 2}, { cell.id * 1 })",
                                        corner_radius: "32",
                                        color: "white",
                                        main_align: "center",
                                        cross_align: "center",
                                        label {
                                            font_size: "14",
                                            "{cell.id:?}"
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
