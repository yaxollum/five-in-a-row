pub const BOARD_SIZE: i32 = 15;

const BOARD_SIZE_UZ: usize = BOARD_SIZE as usize;

#[derive(Clone, Copy, PartialEq, Eq)]
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

enum GameState {
    InProgress,
    Winner(Player),
    Tie,
}

pub struct Game {
    current_player: Player,
    board: [[Cell; BOARD_SIZE_UZ]; BOARD_SIZE_UZ],
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            current_player: Player::Black,
            board: [[None; BOARD_SIZE_UZ]; BOARD_SIZE_UZ],
            state: GameState::InProgress,
        }
    }
    pub fn get_cell(&self, i: i32, j: i32) -> Cell {
        self.board[i as usize][j as usize]
    }
    pub fn get_current_player(&self) -> Player {
        self.current_player
    }
    fn calc_state(&self) -> GameState {
        for i in 0..BOARD_SIZE_UZ {
            for j in 0..BOARD_SIZE_UZ {
                if let Some(player) = self.board[i][j] {
                    if self.board[i + 1][j] == Some(player) {
                        return GameState::Winner(player);
                    }
                }
            }
        }
        GameState::InProgress
    }
    pub fn place_piece(&mut self, i: i32, j: i32) {
        let i = i as usize;
        let j = j as usize;
        if self.board[i][j].is_none() {
            self.board[i][j] = Some(self.current_player);
            self.current_player = self.current_player.other();
            self.state = self.calc_state();
        }
    }
}
