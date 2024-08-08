use crate::defines::*;
use rand::Rng;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl World {

    pub fn new() -> World {
        let width = WIDTH as u32;
        let height = HEIGHT as u32;

        let cells = World::initialize_cells(width, height);
        
        World {
            width,
            height,
            cells,
        }
    }

    pub fn reset_world(&mut self) {
        self.cells = World::initialize_cells(self.width, self.height)
    }

    fn initialize_cells(width: u32, height: u32) -> Vec<Cell> {
        let mut rng = rand::thread_rng();

        (0..width * height)
            .map(|_| {
                if rng.gen_bool(0.2) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;       
    }

    pub fn get_value_at(&self, row: u32, col: u32) -> Cell {
        self.cells[self.get_index(row, col)]
    }
    
}