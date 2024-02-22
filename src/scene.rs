use crate::utils::{Point, to_integer};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[wasm_bindgen]
pub enum Cell {
    Activated = 1,
    Deactivated = 0
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    cells: Vec<Cell>,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|_| {
                Cell::Deactivated
            })
            .collect();

        Scene {
            width,
            height,
            cells
        }
    }

    pub fn get_cell_pos(&self, x: i32, y: i32) -> usize {
        (x * to_integer(self.width) + y) as usize
    }

    pub fn update_frame(&mut self, positions: Vec<Point>) {

        let active_points = positions.iter()
            .map(|v: &Point| { self.get_cell_pos(v.x, v.y) })
            .collect::<Vec<usize>>();

        let next_cells = (0..self.width * self.height)
            .map(|i| {
                let value = i as usize;
                if active_points.contains(&value) {
                    Cell::Activated
                } else {
                    Cell::Deactivated
                }
            })
            .collect();
        self.cells = next_cells;
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

}
