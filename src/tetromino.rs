use macroquad::{color::Color, rand};

use crate::board::Board;

#[derive(Clone)]
pub struct Tetromino {
    pub name: TetrominoName,
    pub structure: TetrominoStructure,
    pub rotation: TetrominoRotation,
    // bottom-left of Tetromino
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy)]
pub enum TetrominoRotation {
    Zero,
    HalfPi,
    Pi,
    TwoPiOverThree,
}

impl TetrominoRotation {
    pub fn next(&mut self) {
        *self = match self {
            Self::Zero => Self::HalfPi,
            Self::HalfPi => Self::Pi,
            Self::Pi => Self::TwoPiOverThree,
            Self::TwoPiOverThree => Self::Zero,
        }
    }

    pub fn previous(&mut self) {
        *self = match self {
            Self::Zero => Self::TwoPiOverThree,
            Self::TwoPiOverThree => Self::Pi,
            Self::Pi => Self::HalfPi,
            Self::HalfPi => Self::Zero,
        }
    }
}

impl Tetromino {
    pub fn new(tetromino_name: TetrominoName, board: &Board) -> Tetromino {
        let structure = match tetromino_name {
            TetrominoName::I => TetrominoStructure {
                color: Color::new(0.0, 1.0, 1.0, 1.0), // cyan
                points: vec![(1, 0), (1, 3)],
            },
            TetrominoName::J => TetrominoStructure {
                color: Color::new(0.0, 0.0, 1.0, 1.0), // blue
                points: vec![(0, 0), (1, 0), (1, 2)],
            },
            TetrominoName::L => TetrominoStructure {
                color: Color::new(1.0, 0.5, 0.0, 1.0), // orange
                points: vec![(1, 2), (1, 0), (2, 0)],
            },
            TetrominoName::O => TetrominoStructure {
                color: Color::new(1.0, 1.0, 0.0, 1.0), // yellow
                points: vec![(0, 1), (0, 0), (1, 0), (1, 1)],
            },
            TetrominoName::S => TetrominoStructure {
                color: Color::new(0.0, 1.0, 0.0, 1.0), // green
                points: vec![(0, 1), (1, 1), (1, 2), (2, 2)],
            },
            TetrominoName::T => TetrominoStructure {
                color: Color::new(0.5, 0.0, 1.0, 1.0), // purple
                points: vec![(0, 1), (2, 1), (1, 2)],
            },
            TetrominoName::Z => TetrominoStructure {
                color: Color::new(1.0, 0.0, 0.0, 1.0), // red
                points: vec![(0, 2), (1, 2), (1, 1), (2, 1)],
            },
        };

        let structure_length = structure.get_length();
        let x = rand::gen_range(
            0,
            ((board.columns() as isize) - structure_length) as i32 + 1,
        );

        Tetromino {
            name: tetromino_name,
            structure,
            rotation: TetrominoRotation::Zero,
            x: x as isize,
            y: board.rows() as isize,
        }
    }

    pub fn random(board: &Board) -> Tetromino {
        let name = match rand::gen_range(0, 7) {
            0 => TetrominoName::I,
            1 => TetrominoName::J,
            2 => TetrominoName::L,
            3 => TetrominoName::O,
            4 => TetrominoName::S,
            5 => TetrominoName::T,
            _ => TetrominoName::Z,
        };
        Tetromino::new(name, board)
    }

    fn get_points_for_rotation(&self) -> Vec<(isize, isize)> {
        let points = &self.structure.points;
        let length = self.get_length();
        return match self.rotation {
            TetrominoRotation::Zero => points.clone(),
            TetrominoRotation::HalfPi => points.iter().map(|&(a, b)| (length - b - 1, a)).collect(),
            TetrominoRotation::Pi => points
                .iter()
                .map(|&(a, b)| (length - a - 1, length - b - 1))
                .collect(),
            TetrominoRotation::TwoPiOverThree => {
                points.iter().map(|&(a, b)| (b, length - a - 1)).collect()
            }
        };
    }

    pub fn get_cells(&self) -> Vec<(isize, isize)> {
        let mut cells = Vec::new();
        let points = self.get_points_for_rotation();
        let mut points_iter = points.iter();
        let first_point = points_iter
            .next()
            .expect("Received a tetromino without any points")
            .clone();
        let mut lx = first_point.0;
        let mut ly = first_point.1;
        for point in points {
            let (cx, cy) = point.clone();
            if cx == lx {
                if ly < cy {
                    for v in ly..cy {
                        cells.push((self.x + cx, self.y + v));
                    }
                } else {
                    for v in (cy + 1)..=ly {
                        cells.push((self.x + cx, self.y + v));
                    }
                }
                lx = cx;
                ly = cy;
            } else if cy == ly {
                if lx < cx {
                    for h in lx..cx {
                        cells.push((self.x + h, self.y + cy));
                    }
                } else {
                    for h in (cx + 1)..=lx {
                        cells.push((self.x + h, self.y + cy));
                    }
                }
                lx = cx;
                ly = cy;
            } else {
                // disjoint points (specifically for the T tetromino)
                cells.push((self.x + lx, self.y + ly));
                lx = cx;
                ly = cy;
            }
        }
        cells.push((self.x + lx, self.y + ly));
        cells
    }

    pub fn get_length(&self) -> isize {
        self.structure.get_length()
    }

    pub fn collides(&self, board: &Board) -> bool {
        let cells = self.get_cells();
        for (x, y) in cells {
            if x < 0 || y < 0 || x >= board.columns() as isize {
                return true;
            }
            if y >= board.rows() as isize {
                continue;
            }
            if board.occupied_at(x as usize, y as usize) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy)]
pub enum TetrominoName {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Clone)]
pub struct TetrominoStructure {
    pub color: Color,
    pub points: Vec<(isize, isize)>,
}

impl TetrominoStructure {
    pub fn get_length(&self) -> isize {
        let x = self.points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let y = self.points.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
        let max = x.max(y);
        max + 1
    }
}
