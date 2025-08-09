pub const BOARD_SIZE: i32 = 15;

const BOARD_SIZE_UZ: usize = BOARD_SIZE as usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    fn other(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

type Cell = Option<Player>;

#[derive(Clone)]
pub enum GameState {
    InProgress(Player),
    Winner(Player),
    Tie,
}

#[derive(Clone)]
pub struct Game {
    board: [[Cell; BOARD_SIZE_UZ]; BOARD_SIZE_UZ],
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[None; BOARD_SIZE_UZ]; BOARD_SIZE_UZ],
            state: GameState::InProgress(Player::Black),
        }
    }
    pub fn get_cell(&self, i: i32, j: i32) -> Cell {
        self.board
            .get(i as usize)
            .and_then(|row| row.get(j as usize).copied().flatten())
    }
    pub fn get_state(&self) -> GameState {
        self.state.clone()
    }
    fn calc_next_state(&self) -> GameState {
        match self.state {
            GameState::InProgress(current_player) => {
                let mut board_is_full = true;
                for i in 0..BOARD_SIZE {
                    for j in 0..BOARD_SIZE {
                        if let Some(player) = self.get_cell(i, j) {
                            if (0..=4).all(|x| self.get_cell(i + x, j) == Some(player)) {
                                return GameState::Winner(player);
                            }
                            if (0..=4).all(|x| self.get_cell(i, j + x) == Some(player)) {
                                return GameState::Winner(player);
                            }
                            if (0..=4).all(|x| self.get_cell(i + x, j + x) == Some(player)) {
                                return GameState::Winner(player);
                            }
                            if (0..=4).all(|x| self.get_cell(i + x, j - x) == Some(player)) {
                                return GameState::Winner(player);
                            }
                        } else {
                            board_is_full = false;
                        }
                    }
                }
                if board_is_full {
                    GameState::Tie
                } else {
                    GameState::InProgress(current_player.other())
                }
            }
            _ => self.state.clone(),
        }
    }
    pub fn place_piece(&mut self, i: i32, j: i32) -> Result<(), ()> {
        if let GameState::InProgress(current_player) = self.state {
            let i = i as usize;
            let j = j as usize;
            if self.board[i][j].is_none() {
                self.board[i][j] = Some(current_player);
                self.state = self.calc_next_state();
                return Ok(());
            }
        }
        Err(())
    }
}
