use std::time::Duration;

use freya::prelude::*;
use rand::{
    seq::SliceRandom,
    thread_rng,
};

#[derive(Debug, Clone)]
struct Cell {
    id: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn new(size: usize) -> Self {
        let mut cells = Vec::new();
        for i in 0..size {
            for j in 0..size {
                cells.push(Cell { id: i * size + j });
            }
        }
        Grid { cells }
    }

    pub fn suffle(&mut self) {
        let mut rng = thread_rng();

        self.cells.shuffle(&mut rng);
    }
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut grid = use_signal(|| Grid::new(5));
    rsx!(
        rect {
            spacing: "12",
            main_align: "center",
            cross_align: "center",
            width: "fill",
            height: "fill",
            GlobalAnimatedPositionProvider::<usize> {
                Button {
                    onpress: move |_| grid.write().suffle(),
                    label {
                        "Shuffle"
                    }
                }
                rect {
                    spacing: "6",
                    for row in grid.read().cells.chunks(5) {
                        rect {
                            direction: "horizontal",
                            spacing: "6",
                            for cell in row {
                                GlobalAnimatedPosition::<usize> {
                                    key: "{cell.id:?}",
                                    width: "100",
                                    height: "100",
                                    function: Function::Expo,
                                    duration: Duration::from_millis(600),
                                    id: cell.id,
                                    rect {
                                        width: "100",
                                        height: "100",
                                        background: "rgb({cell.id * 6}, {cell.id * 8}, { cell.id * 2 })",
                                        corner_radius: "32",
                                        color: "white",
                                        main_align: "center",
                                        cross_align: "center",
                                        label {
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
