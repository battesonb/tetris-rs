use macroquad::{miniquad::date, prelude::*, window};

mod board;
mod color;
mod renderer;
mod tetromino;

use board::Board;
use renderer::Renderer;
use tetromino::Tetromino;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
pub const CELL_SIZE: usize = 30;
const DEBOUNCE_TIME: f32 = 0.3;
const FAST_DEBOUNCE_TIME: f32 = 0.05;

fn window_conf() -> window::Conf {
    window::Conf {
        window_height: (BOARD_HEIGHT * CELL_SIZE) as i32,
        window_width: (BOARD_WIDTH * CELL_SIZE) as i32,
        window_title: "Tetris in Rust".into(),
        window_resizable: false,
        ..Default::default()
    }
}

pub struct State {
    board: Board,
    current_tetromino: Option<Tetromino>,
    debounce_timer: f32,
    speed_mode: bool,
}

impl State {
    pub fn new() -> State {
        let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
        State {
            board,
            current_tetromino: None,
            debounce_timer: 0.0,
            speed_mode: false,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();
    let renderer = Renderer::new();
    rand::srand(date::now() as u64);

    loop {
        clear_background(Color::new(0.15, 0.15, 0.17, 1.0));
        renderer.draw(&state);
        update(&mut state);
        next_frame().await
    }
}

fn update(state: &mut State) {
    let delta = get_frame_time();
    state.debounce_timer += delta;

    if let Some(tetromino) = &mut state.current_tetromino {
        if let Some(key_code) = get_last_key_pressed() {
            match key_code {
                KeyCode::W | KeyCode::Space => {
                    tetromino.rotation.next();
                    if tetromino.collides(&state.board) {
                        // Some implementations "kick off" the collision point, but skipping that for now
                        tetromino.rotation.previous();
                    }
                }
                KeyCode::A => {
                    tetromino.x -= 1;
                    if tetromino.collides(&state.board) {
                        tetromino.x += 1;
                    }
                }
                KeyCode::D => {
                    tetromino.x += 1;
                    if tetromino.collides(&state.board) {
                        tetromino.x -= 1;
                    }
                }
                KeyCode::S => {
                    state.speed_mode = true;
                }
                _ => {}
            }
        }
    }

    if is_key_released(KeyCode::S) {
        state.speed_mode = false;
    }

    let debounce_time = if state.speed_mode {
        FAST_DEBOUNCE_TIME
    } else {
        DEBOUNCE_TIME
    };

    if state.debounce_timer > debounce_time {
        state.debounce_timer = 0.0;

        if let Some(tetromino) = &mut state.current_tetromino {
            // Try and move the tetromino down
            tetromino.y -= 1;

            let mut collides = false;
            if tetromino.collides(&state.board) {
                collides = true;
                tetromino.y += 1;
            }

            if collides {
                let cells = tetromino.get_cells();
                for &(x, y) in cells.iter() {
                    if x >= 0 && y >= 0 && x < state.board.columns() as isize {
                        if y >= state.board.rows() as isize {
                            state.board.clear();
                            break;
                        }
                        state.board.set_cell(
                            x as usize,
                            y as usize,
                            Some(tetromino.structure.color),
                        );
                    }
                }

                for (_, y) in cells {
                    if y >= 0 && y < state.board.rows() as isize {
                        state.board.try_line_clear(y);
                    }
                }

                state.current_tetromino = None;
            }
        } else {
            state.current_tetromino = Some(Tetromino::random(&state.board))
        }
    }
}
