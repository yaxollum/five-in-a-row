use macroquad::prelude::*;

const BOARD_SIZE: i32 = 15;

struct BoardShape {
    corner_x: f32,
    corner_y: f32,
    length: f32,
}

impl BoardShape {
    fn get_cell_size(&self) -> f32 {
        self.length / BOARD_SIZE as f32
    }
    fn px_to_coord(&self, x: f32, y: f32) -> Option<(i32, i32)> {
        let coord_x = ((x - self.corner_x) / self.get_cell_size()) as i32;
        let coord_y = ((y - self.corner_y) / self.get_cell_size()) as i32;
        if coord_x >= 0 && coord_x < BOARD_SIZE && coord_y >= 0 && coord_y < BOARD_SIZE {
            Some((coord_x, coord_y))
        } else {
            None
        }
    }
    fn coord_to_px(&self, coord_x: i32, coord_y: i32) -> (f32, f32) {
        (
            (coord_x as f32 + 0.5) * self.get_cell_size() + self.corner_x,
            (coord_y as f32 + 0.5) * self.get_cell_size() + self.corner_y,
        )
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let background_color = Color::from_rgba(245, 193, 71, 255);
    let text_bottom = 40.0;
    let line_thickness = 2.0;
    loop {
        clear_background(background_color);
        draw_text(
            "White's turn",
            text_bottom / 2.0,
            text_bottom,
            text_bottom,
            BLACK,
        );
        let board_shape = BoardShape {
            corner_x: 0.0,
            corner_y: text_bottom,
            length: f32::min(screen_width(), screen_height() - text_bottom),
        };
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
        next_frame().await
    }
}
