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

    pub fn get_player(&mut self, row: usize, col: usize) -> Option<Player> {
        self.board.get(row)?.get(col)?.clone()
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

        // Diagonal checks
        for (row_n, row) in self.board.clone().iter().enumerate() {
            for (col_n, _) in row.iter().enumerate() {
                let lines = match row_n {
                    0 if col_n < 2 => vec![(
                        (row_n, col_n),
                        (row_n + 1, col_n + 1),
                        (row_n + 2, col_n + 2),
                    )],
                    0 if col_n > 2 => vec![(
                        (row_n, col_n),
                        (row_n - 1, col_n - 1),
                        (row_n - 2, col_n - 2),
                    )],
                    1 if col_n >= 1 => vec![
                        (
                            (row_n - 1, col_n - 1),
                            (row_n, col_n),
                            (row_n + 1, col_n + 1),
                        ),
                        (
                            (row_n - 1, col_n + 1),
                            (row_n, col_n),
                            (row_n + 1, col_n - 1),
                        ),
                    ],
                    2 if col_n >= 2 => vec![(
                        (row_n - 2, col_n - 2),
                        (row_n - 1, col_n - 1),
                        (row_n, col_n),
                    )],
                    2 if col_n < 2 => vec![(
                        (row_n + 2, col_n + 2),
                        (row_n + 1, col_n + 1),
                        (row_n, col_n),
                    )],
                    _ => vec![],
                };

                for (top, mid, bot) in lines {
                    let line = vec![
                        self.get_player(top.0, top.1),
                        self.get_player(mid.0, mid.1),
                        self.get_player(bot.0, bot.1),
                    ];

                    let is_line_filled_with_x =
                        line.iter().all(|player| *player == Some(Player::X));
                    let is_line_filled_with_o =
                        line.iter().all(|player| *player == Some(Player::O));

                    if is_line_filled_with_x {
                        self.winner = Some(Player::X);
                        return;
                    } else if is_line_filled_with_o {
                        self.winner = Some(Player::O);
                        return;
                    }
                }
            }
        }
    }
}

fn app() -> Element {
    let mut board = use_signal(|| Board::new(3));
    let mut current_player = use_signal(|| Player::X);

    let message = match &board.read().winner {
        Some(winner) => format!("Winner is player {winner:?}!!"),
        None => format!("Turn for player {:?}", current_player.read()),
    };

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            rect {
                width: "150",
                height: "170",
                label {
                    height: "20",
                    width: "100%",
                    text_align: "center",
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
                                corner_radius: "6",
                                border: "2 solid rgb(40, 40, 40)",
                                background: "rgb(250, 250, 250)",
                                main_align: "center",
                                cross_align: "center",
                                onclick: move |_| {
                                    let mut board = board.write();
                                    if board.winner.is_none(){
                                        board.put_player(current_player.read().clone(), n_row, n_col);
                                    }
                                    current_player.write().change_turn();
                                },
                                if let Some(col) = col {
                                    label {
                                        "{col:?}"
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
