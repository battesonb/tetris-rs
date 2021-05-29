use macroquad::prelude::*;

use crate::{board::Board, color::ColorExtension, tetromino::Tetromino, State};
pub struct Renderer {}

const GRID_THICKNESS: f32 = 1.0;
const GRID_COLOR: Color = BLACK;

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn draw(&self, state: &State) {
        self.draw_cells(&state.board);
        if let Some(tetromino) = &state.current_tetromino {
            self.draw_tetromino(&state.board, tetromino, false);
        }
        self.draw_grid(state.board.columns(), state.board.rows());
        if let Some(tetromino) = &state.current_tetromino {
            self.draw_tetromino(&state.board, tetromino, true);
        }
    }

    fn draw_cells(&self, board: &Board) {
        let h_size = screen_width() / (board.columns() as f32);
        let v_size = screen_height() / (board.rows() as f32);
        for i in 0..board.columns() {
            for j in 0..board.rows() {
                if let Some(color) = board.cell_at(i, j) {
                    draw_rectangle(
                        (i as f32) * h_size,
                        ((board.rows() - j - 1) as f32) * v_size,
                        h_size,
                        v_size,
                        color,
                    );
                }
            }
        }
    }

    fn draw_tetromino(&self, board: &Board, tetromino: &Tetromino, outline: bool) {
        let h_size = screen_width() / (board.columns() as f32);
        let v_size = screen_height() / (board.rows() as f32);
        let cells = tetromino.get_cells();

        for (x, y) in cells {
            let x_point = (x as f32) * h_size;
            let y_point = ((board.rows() as isize - y - 1) as f32) * v_size;
            if outline {
                draw_rectangle_lines(
                    x_point,
                    y_point,
                    h_size,
                    v_size,
                    GRID_THICKNESS,
                    tetromino.structure.color.add(0.25),
                );
            } else {
                draw_rectangle(
                    x_point,
                    y_point,
                    h_size,
                    v_size,
                    tetromino.structure.color.add(-0.25),
                );
            }
        }
    }

    fn draw_grid(&self, h_slices: usize, v_slices: usize) {
        let h_size = screen_width() / (h_slices as f32);
        for i in 0..h_slices {
            let x = (i as f32) * h_size;
            draw_line(x, 0.0, x, screen_height(), GRID_THICKNESS, GRID_COLOR);
        }
        let v_size = screen_height() / (v_slices as f32);
        for j in 0..v_slices {
            let y = (j as f32) * v_size;
            draw_line(0.0, y, screen_width(), y, GRID_THICKNESS, GRID_COLOR);
        }
    }
}
