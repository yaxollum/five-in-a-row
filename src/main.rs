mod ai;
mod game;

use game::BOARD_SIZE;
use macroquad::prelude::*;

struct BoardShape {
    corner_x: f32,
    corner_y: f32,
    length: f32,
}

impl BoardShape {
    fn from_rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        let length = f32::min(x2 - x1, y2 - y1);
        Self {
            corner_x: (x1 + x2 - length) / 2.0,
            corner_y: (y1 + y2 - length) / 2.0,
            length,
        }
    }
    fn get_cell_size(&self) -> f32 {
        self.length / BOARD_SIZE as f32
    }
    fn get_circle_radius(&self) -> f32 {
        self.get_cell_size() * 0.4
    }
    fn px_to_coord(&self, x: f32, y: f32) -> Option<(i32, i32)> {
        let cell_size = self.get_cell_size();
        let coord_x = ((x - self.corner_x) / cell_size) as i32;
        let coord_y = ((y - self.corner_y) / cell_size) as i32;
        if coord_x >= 0 && coord_x < BOARD_SIZE && coord_y >= 0 && coord_y < BOARD_SIZE {
            let (center_x, center_y) = self.coord_to_px(coord_x, coord_y);
            if f32::powi(center_x - x, 2) + f32::powi(center_y - y, 2)
                < f32::powi(self.get_circle_radius(), 2)
            {
                return Some((coord_x, coord_y));
            }
        }
        None
    }
    fn coord_to_px(&self, coord_x: i32, coord_y: i32) -> (f32, f32) {
        (
            (coord_x as f32 + 0.5) * self.get_cell_size() + self.corner_x,
            (coord_y as f32 + 0.5) * self.get_cell_size() + self.corner_y,
        )
    }
}

enum Player {
    Human,
    Ai(Box<dyn ai::Ai>),
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let background_color = Color::from_rgba(245, 193, 71, 255);
    let text_bottom = 40.0;
    let line_thickness = 2.0;
    let pending_move_white = Color::from_rgba(255, 255, 255, 100);
    let pending_move_black = Color::from_rgba(0, 0, 0, 100);

    let mut game = game::Game::new();

    let white_player = Player::Human;
    //let white_player = Player::Ai(Box::new(ai::RandomAi));
    let black_player = Player::Ai(Box::new(ai::RandomAi));
    loop {
        clear_background(background_color);
        draw_text(
            match game.get_state() {
                game::GameState::InProgress(game::Player::White) => "White's turn",
                game::GameState::InProgress(game::Player::Black) => "Black's turn",
                game::GameState::Winner(game::Player::White) => "White wins!",
                game::GameState::Winner(game::Player::Black) => "Black wins!",
                game::GameState::Tie => "Tie!",
            },
            text_bottom / 2.0,
            text_bottom,
            text_bottom,
            match game.get_state() {
                game::GameState::InProgress(game::Player::White) => WHITE,
                game::GameState::InProgress(game::Player::Black) => BLACK,
                game::GameState::Winner(game::Player::White) => WHITE,
                game::GameState::Winner(game::Player::Black) => BLACK,
                game::GameState::Tie => GRAY,
            },
        );
        let board_shape = BoardShape::from_rect(0.0, text_bottom, screen_width(), screen_height());
        for i in 0..BOARD_SIZE {
            let (x1, y1) = board_shape.coord_to_px(i, 0);
            let (x2, y2) = board_shape.coord_to_px(i, BOARD_SIZE - 1);
            draw_line(x1, y1, x2, y2, line_thickness, BLACK);
        }
        for i in 0..BOARD_SIZE {
            let (x1, y1) = board_shape.coord_to_px(0, i);
            let (x2, y2) = board_shape.coord_to_px(BOARD_SIZE - 1, i);
            draw_line(x1, y1, x2, y2, line_thickness, BLACK);
        }
        if let game::GameState::InProgress(current_player) = game.get_state() {
            let current_player_obj = match current_player {
                game::Player::Black => &black_player,
                game::Player::White => &white_player,
            };
            match current_player_obj {
                Player::Human => {
                    let (mouse_x, mouse_y) = mouse_position();
                    if let Some((coord_x, coord_y)) = board_shape.px_to_coord(mouse_x, mouse_y) {
                        if game.get_cell(coord_x, coord_y).is_none() {
                            if is_mouse_button_pressed(MouseButton::Left) {
                                game.place_piece(coord_x, coord_y);
                            } else {
                                let (circle_x, circle_y) =
                                    board_shape.coord_to_px(coord_x, coord_y);

                                draw_circle(
                                    circle_x,
                                    circle_y,
                                    board_shape.get_circle_radius(),
                                    match current_player {
                                        game::Player::White => pending_move_white,
                                        game::Player::Black => pending_move_black,
                                    },
                                );
                            }
                        }
                    }
                }
                Player::Ai(ai) => {
                    let (ai_x, ai_y) = ai.get_move(&game);
                    game.place_piece(ai_x, ai_y);
                }
            }
        }
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(player) = game.get_cell(i, j) {
                    let (circle_x, circle_y) = board_shape.coord_to_px(i, j);
                    draw_circle(
                        circle_x,
                        circle_y,
                        board_shape.get_circle_radius(),
                        match player {
                            game::Player::White => WHITE,
                            game::Player::Black => BLACK,
                        },
                    );
                }
            }
        }
        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::Ai;

    enum GameResult {
        Winner(game::Player),
        Tie,
    }

    fn run_game() -> GameResult {
        let ai_obj: ai::RandomAi = ai::RandomAi;
        let mut game = game::Game::new();
        loop {
            let (x, y) = ai_obj.get_move(&game);
            game.place_piece(x, y);
            match game.get_state() {
                game::GameState::InProgress(_) => {}
                game::GameState::Winner(player) => return GameResult::Winner(player),
                game::GameState::Tie => return GameResult::Tie,
            }
        }
    }
    #[test]
    fn test_game() {
        let mut white_win = 0;
        let mut black_win = 0;
        let mut tie = 0;
        for i in 1..=1000000 {
            if i % 1000 == 0 {
                println!("run #{}", i);
            }
            match run_game() {
                GameResult::Winner(game::Player::White) => white_win += 1,
                GameResult::Winner(game::Player::Black) => black_win += 1,
                GameResult::Tie => tie += 1,
            }
        }
        println!("{} {} {}", white_win, black_win, tie);
    }
}
