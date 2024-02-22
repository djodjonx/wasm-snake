use wasm_bindgen::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

pub fn to_integer(number: u32) -> i32{
    number as i32
}

pub fn get_next_position(position: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => Point::new(position.x, position.y - 1),
        Direction::Down => Point::new(position.x, position.y + 1),
        Direction::Right => Point::new(position.x + 1, position.y),
        Direction::Left => Point::new(position.x - 1, position.y),
    }
}

pub fn real_pos_from_max(position: i32, max: i32) -> i32 {
    match (position, max) {
        (position, max) if position >= max => position - max,
        (position, max) if position < 0 => position + max,
        (position, _) => position
    }
}