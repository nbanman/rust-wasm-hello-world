mod utils;

use std::fmt::{self, Display};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = vec![Cell::Dead; self.height as usize * width as usize];
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = vec![Cell::Dead; height as usize * self.width as usize];
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        self.cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, &cell)| {
                let live_neighbors = self.live_neighbor_count(index);
                match cell {
                    Cell::Dead => {
                        if live_neighbors == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                    Cell::Alive => {
                        if (2..=3).contains(&live_neighbors) {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                }
            })
            .collect();
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_components(&self, index: usize) -> (u32, u32) {
        (index as u32 % self.width, index as u32 / self.width)
    }

    fn live_neighbor_count(&self, index: usize) -> u8 {
        NEIGHBORS
            .iter()
            .map(|&(dx, dy)| {
                let (x, y) = self.get_components(index);
                let nx = (dx + x as i32).rem_euclid(self.width as i32);
                let ny = (dy + y as i32).rem_euclid(self.height as i32);
                self.cells[self.get_index(nx as u32, ny as u32)] as u8
            })
            .sum()
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for &(y, x) in cells.iter() {
            let idx = self.get_index(y, x);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
