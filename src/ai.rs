use crate::game;
use rand::Rng;

pub struct RandomAi;

impl Ai for RandomAi {
    fn get_move(&self, _g: &game::Game) -> (i32, i32) {
        let mut rng = rand::rng();
        (
            rng.random_range(0..game::BOARD_SIZE),
            rng.random_range(0..game::BOARD_SIZE),
        )
    }
}

// Same as random AI, but avoids winning
pub struct NonconfrontationalAi;

impl Ai for NonconfrontationalAi {
    fn get_move(&self, g: &game::Game) -> (i32, i32) {
        let mut rng = rand::rng();
        for _ in 0..2000 {
            let (move_x, move_y) = (
                rng.random_range(0..game::BOARD_SIZE),
                rng.random_range(0..game::BOARD_SIZE),
            );
            if g.get_cell(move_x, move_y).is_none() {
                let mut g_clone = g.clone();
                g_clone.place_piece(move_x, move_y);
                if let game::GameState::Winner(_) = g_clone.get_state() {
                    continue;
                } else {
                    return (move_x, move_y);
                }
            }
        }
        (
            rng.random_range(0..game::BOARD_SIZE),
            rng.random_range(0..game::BOARD_SIZE),
        )
    }
}

pub trait Ai {
    fn get_move(&self, g: &game::Game) -> (i32, i32);
}
