extern crate rand;
mod snake;
mod scene;
mod utils;
mod fruit;
mod game;

use wasm_bindgen::prelude::*;

use crate::game::Game;
use crate::scene::Cell;
use crate::utils::Direction;

#[wasm_bindgen]
pub struct SnakeGame {
    game: Game,
}

#[wasm_bindgen]
impl SnakeGame {
    pub fn new(width: u32, height: u32) -> Self {
        let game = Game::new(width, height);

        SnakeGame {
            game
        }
    }

    pub fn get_cells(&self) -> *const Cell {
        self.game.get_scene_cells()
    }

    pub fn update(&mut self) {
        self.game.update();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.game.set_direction(direction);
    }

    pub fn is_game_over(&self) -> bool {
        self.game.is_game_over()
    }

    pub fn is_snake_cell(&self, cell_number: usize) -> bool {
        self.game.is_snake_cell(cell_number)
    }

    pub fn get_score(&self) -> u32 {
        self.game.score
    }
}

