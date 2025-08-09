use crate::game;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct RandomAi;

impl Ai for RandomAi {
    fn get_move(&self, g: &game::Game) -> (i32, i32) {
        let mut rng = rand::rng();
        loop {
            let (move_x, move_y) = (
                rng.random_range(0..game::BOARD_SIZE),
                rng.random_range(0..game::BOARD_SIZE),
            );
            if g.get_cell(move_x, move_y).is_none() {
                return (move_x, move_y);
            }
        }
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
                g_clone.place_piece(move_x, move_y).unwrap();
                if let game::GameState::Winner(_) = g_clone.get_state() {
                    continue;
                } else {
                    return (move_x, move_y);
                }
            }
        }
        loop {
            let (move_x, move_y) = (
                rng.random_range(0..game::BOARD_SIZE),
                rng.random_range(0..game::BOARD_SIZE),
            );
            if g.get_cell(move_x, move_y).is_none() {
                return (move_x, move_y);
            }
        }
    }
}

pub struct SmartAi {
    search_quota: i64,
}

#[derive(Debug)]
enum SearchOutcome {
    Indeterminate { depth: i64 },
    Win,
    Loss,
}

#[derive(Debug)]
struct SearchResult {
    used_quota: i64,
    best_move: (i32, i32),
    outcome: SearchOutcome,
}

impl SmartAi {
    pub fn new(search_quota: i64) -> Self {
        Self { search_quota }
    }
    fn search(g: &game::Game, search_quota: i64, rng: &mut rand::rngs::ThreadRng) -> SearchResult {
        if let game::GameState::InProgress(current_player) = g.get_state() {
            let mut min_termination_depth = i64::MAX;
            let mut best_move: Option<(i32, i32)> = None;
            let mut best_move_depth = i64::MIN;
            let mut possible_moves: Vec<(i32, i32)> = (0..game::BOARD_SIZE)
                .flat_map(|i| {
                    (0..game::BOARD_SIZE).filter_map(move |j| {
                        if g.get_cell(i, j).is_none()
                            && (g.get_cell(i - 1, j - 1).is_some()
                                || g.get_cell(i - 1, j).is_some()
                                || g.get_cell(i - 1, j + 1).is_some()
                                || g.get_cell(i, j - 1).is_some()
                                || g.get_cell(i, j + 1).is_some()
                                || g.get_cell(i + 1, j - 1).is_some()
                                || g.get_cell(i + 1, j).is_some()
                                || g.get_cell(i + 1, j + 1).is_some())
                        {
                            Some((i, j))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            possible_moves.shuffle(rng);
            let mut used_quota = 0;
            for (idx, &(i, j)) in possible_moves.iter().enumerate() {
                if used_quota >= search_quota {
                    return SearchResult {
                        used_quota,
                        best_move: possible_moves[0],
                        outcome: SearchOutcome::Indeterminate { depth: 0 },
                    };
                }
                let mut new_g = g.clone();
                new_g.place_piece(i, j).unwrap();
                used_quota += 1;
                match new_g.get_state() {
                    game::GameState::InProgress(_) => {
                        let search_result = Self::search(
                            &new_g,
                            (search_quota - used_quota) / (possible_moves.len() - idx) as i64,
                            rng,
                        );
                        used_quota += search_result.used_quota;
                        match search_result.outcome {
                            SearchOutcome::Indeterminate { depth } => {
                                min_termination_depth = min_termination_depth.min(depth);
                                if depth > best_move_depth {
                                    best_move_depth = depth;
                                    best_move = Some((i, j));
                                }
                            }
                            SearchOutcome::Loss => {
                                return SearchResult {
                                    used_quota,
                                    best_move: (i, j),
                                    outcome: SearchOutcome::Win,
                                }
                            }
                            SearchOutcome::Win => {}
                        }
                    }
                    game::GameState::Winner(p) => {
                        if p == current_player {
                            return SearchResult {
                                used_quota,
                                best_move: (i, j),
                                outcome: SearchOutcome::Win,
                            };
                        }
                    }
                    game::GameState::Tie => {}
                }
            }

            if let Some(best_move) = best_move {
                SearchResult {
                    used_quota,
                    best_move,
                    outcome: SearchOutcome::Indeterminate {
                        depth: min_termination_depth + 1,
                    },
                }
            } else {
                SearchResult {
                    used_quota,
                    best_move: possible_moves[0],
                    outcome: SearchOutcome::Loss,
                }
            }
        } else {
            panic!("game is not in progress")
        }
    }
}

impl Ai for SmartAi {
    fn get_move(&self, g: &game::Game) -> (i32, i32) {
        let mut rng = rand::rng();
        let search_result = Self::search(g, self.search_quota, &mut rng);
        println!("{:?}", search_result);
        search_result.best_move
    }
}

pub trait Ai {
    fn get_move(&self, g: &game::Game) -> (i32, i32);
}
