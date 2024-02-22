use crate::utils::{ Point, Direction, get_next_position };

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut body = vec![];
        body.push(Point::new(0, 0));
        body.push(Point::new(-1, 0));
        body.push(Point::new(-2, 0));

        Snake {
            body,
            direction: Direction::Right,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction
        }
    }
}

pub trait SnakeMove {
    fn move_body(&mut self, grow: bool);
}

impl SnakeMove for Snake {
    fn move_body(&mut self, grow: bool) {
        let next_block = get_next_position(&self.body[0], &self.direction);
        let from_zero_x = 0 - next_block.x;
        let from_zero_y = 0 - next_block.y;
        self.body.insert(0, next_block);
        if !grow {
            self.body.pop();
        }
        self.body = self.body
            .clone()
            .iter()
            .map(|pos| { Point::new(pos.x + from_zero_x, pos.y + from_zero_y)})
            .collect();
    }
}