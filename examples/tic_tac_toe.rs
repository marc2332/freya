#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[derive(Clone, Debug, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn change_turn(&mut self) {
        match self {
            Self::O => *self = Self::X,
            Self::X => *self = Self::O,
        }
    }
}

struct Board {
    pub winner: Option<Player>,
    pub board: Vec<Vec<Option<Player>>>,
    pub size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            winner: None,
            size,
            board: vec![vec![None; size]; size],
        }
    }

    pub fn put_player(&mut self, player: Player, row: usize, col: usize) {
        *self.board.get_mut(row).unwrap().get_mut(col).unwrap() = Some(player);
        self.evaluate_board();
    }

    pub fn evaluate_board(&mut self) {
        // Horizontal checks
        for row in self.board.iter() {
            let is_row_filled_with_x = row.iter().all(|p| *p == Some(Player::X));
            let is_row_filled_with_o = row.iter().all(|p| *p == Some(Player::O));

            if is_row_filled_with_x {
                self.winner = Some(Player::X);
                return;
            } else if is_row_filled_with_o {
                self.winner = Some(Player::O);
                return;
            }
        }

        // Vertical checks
        for col_n in 0..self.size {
            let is_col_filled_with_x = self
                .board
                .iter()
                .all(|row| *row.get(col_n).unwrap() == Some(Player::X));
            let is_col_filled_with_o = self
                .board
                .iter()
                .all(|row| *row.get(col_n).unwrap() == Some(Player::O));

            if is_col_filled_with_x {
                self.winner = Some(Player::X);
                return;
            } else if is_col_filled_with_o {
                self.winner = Some(Player::O);
                return;
            }
        }

        // TODO: Diagonal checks
    }
}

fn app(cx: Scope) -> Element {
    let board = use_ref(cx, || Board::new(3));
    let current_player = use_ref(cx, || Player::X);

    let message = match &board.read().winner {
        Some(winner) => format!("Winner is player {winner:?}!!"),
        None => format!("Turn for player {:?}", current_player.read()),
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            display: "center",
            direction: "both",
            rect {
                width: "150",
                height: "170",
                label {
                    height: "20",
                    width: "100%",
                    align: "center",
                    "{message}"
                }
                for (n_row, row) in board.read().board.iter().enumerate() {
                    rect {
                        key: "{n_row}",
                        direction: "horizontal",
                        for (n_col, col) in row.iter().enumerate() {
                            rect {
                                key: "{n_col}",
                                margin: "1",
                                width: "48",
                                height: "48",
                                border: "2 solid rgb(35, 35, 35)",
                                background: "rgb(240, 240, 240)",
                                display: "center",
                                direction: "both",
                                onclick: move |_| {
                                    board.write().put_player(current_player.read().clone(), n_row, n_col);
                                    current_player.write().change_turn();
                                },
                                if let Some(col) = col {
                                    rsx!(
                                        label {
                                            "{col:?}"
                                        }
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
