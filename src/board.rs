use macroquad::prelude::Color;

pub struct Board {
    columns: usize,
    rows: usize,
    cells: Vec<Option<Color>>,
}

impl Board {
    pub fn new(columns: usize, rows: usize) -> Board {
        let cells = vec![None; rows * columns];
        Board {
            columns,
            rows,
            cells,
        }
    }

    pub fn columns(&self) -> usize {
        return self.columns;
    }

    pub fn rows(&self) -> usize {
        return self.rows;
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Option<Color> {
        return self
            .cells
            .get(x + y * self.columns)
            .expect("Invalid state, board did not allocate enough space!")
            .clone();
    }

    pub fn set_cell(&mut self, x: usize, y: usize, color: Option<Color>) {
        self.cells[x + y * self.columns] = color;
    }

    pub fn occupied_at(&self, x: usize, y: usize) -> bool {
        if let None = self.cell_at(x, y) {
            false
        } else {
            true
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![None; self.rows * self.columns];
    }

    pub fn try_line_clear(&mut self, row: isize) {
        let mut all_occupied = true;
        for x in 0..self.columns {
            if let None = self.cells.get(x + (row as usize) * self.columns).unwrap() {
                all_occupied = false;
                break;
            }
        }

        if all_occupied {
            for x in 0..self.columns {
                for y in (row as usize)..(self.rows - 1) {
                    self.cells[x + y * self.columns] = self.cells[x + (y + 1) * self.columns];
                }

                self.cells[x + (self.rows - 1) * self.columns] = None;
            }

            self.try_line_clear(row);
        }
    }
}
