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

pub trait Ai {
    fn get_move(&self, g: &game::Game) -> (i32, i32);
}
