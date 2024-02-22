extern crate rand;

use rand::{Rng, thread_rng};
use std::option::Option;

use crate::scene::{Scene, Cell};
use crate::snake::{Snake, SnakeMove};
use crate::utils::{Direction, get_next_position, Point, to_integer, real_pos_from_max};
use crate::fruit::Fruit;


pub struct Game {
    snake: Snake,
    scene: Scene,
    snake_head_pos: Point,
    fruit: Option<Fruit>,
    game_over: bool,
    pub score: u32
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let center_x = width / 2;
        let center_y = height / 2;
        let snake_head_pos = Point::new(to_integer(center_x), to_integer(center_y));

        let snake = Snake::new();

        let scene = Scene::new(width, height);

        Game {
            snake,
            scene,
            snake_head_pos,
            fruit: None,
            game_over: false,
            score: 0
        }
    }

    pub fn get_scene_cells(&self) -> *const Cell {
        self.scene.cells()
    }

    pub fn update(&mut self) {
        self.move_snake();
        match &self.fruit {
            Some(_) => {},
            None => self.gen_fruit(),
        };

        let mut items = vec![];

        let mut snake_body = self.get_real_snake_position();

        items.append(&mut snake_body);

        let mut fruit_pos = match &self.fruit {
            Some(fruit) => vec![fruit.position],
            None => vec![],
        };

        items.append(&mut fruit_pos);

        self.scene.update_frame(items);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn is_snake_cell(&self, cell_number: usize) -> bool {
        let snake_cells_ids = self.get_real_snake_position()
            .iter()
            .map(|point| { self.scene.get_cell_pos(point.x, point.y) })
            .collect::<Vec<usize>>();

        for cell in snake_cells_ids {
            if cell == cell_number {
                return true
            }
        };
        return false
    }

    fn gen_fruit(&mut self) {
        let mut rng = thread_rng();
        let width = to_integer(self.scene.width);
        let height = to_integer(self.scene.height);
        let mut x = rng.gen_range(1..width -1 );
        let mut y = rng.gen_range(1..height - 1);
        while self.overlap_snake(x, y) {
            x = rng.gen_range(1..width - 1);
            y = rng.gen_range(1..height - 1);
        };
        self.fruit = Some(Fruit::new(x, y));
    }

    fn to_real_x_pos(&self, position: i32) -> i32 {
        position + self.snake_head_pos.x
    }
    fn to_real_y_pos(&self, position: i32) -> i32 {
        position + self.snake_head_pos.y
    }

    fn get_real_snake_position(&self) -> Vec<Point> {
        self.snake.body
            .iter()
            .map(|point| {
                let x = real_pos_from_max(self.to_real_x_pos(point.x), to_integer(self.scene.width));
                let y = real_pos_from_max(self.to_real_y_pos(point.y), to_integer(self.scene.height));
                Point::new(x, y)
            })
            .collect::<Vec<Point>>()
    }

    fn set_head_position(&mut self, x: i32, y: i32) {
        let real_x_pos = real_pos_from_max(x, to_integer(self.scene.width));
        let real_y_pos = real_pos_from_max(y, to_integer(self.scene.height));
        self.snake_head_pos = Point::new(real_x_pos, real_y_pos);
    }
    fn move_snake(&mut self) {
        let mut snake_grow = false;
        match &self.fruit {
            Some(fruit) => snake_grow = self.overlap_snake(fruit.position.x, fruit.position.y),
            None => {}
        }
        if snake_grow {
            self.score += 1;
            self.fruit = None
        }
        self.snake.move_body(snake_grow);
        let next_head_position = get_next_position(&self.snake_head_pos, &self.snake.direction);

        if self.overlap_snake(next_head_position.x, next_head_position.y) {
            self.game_over = true;
        } else {
            self.set_head_position(next_head_position.x, next_head_position.y)
        }
    }

    fn overlap_snake(&self, x: i32, y: i32) -> bool {
        for point in &self.get_real_snake_position() {
            if x == point.x && y == point.y {
                return true;
            }
        }
        return false;
    }
}

