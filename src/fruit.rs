use crate::utils::Point;

pub struct Fruit {
    pub position: Point,
    pub exist: bool
}

impl Fruit {
    pub fn new(x: i32, y: i32) -> Self {
        Fruit {
            position: Point::new(x,y),
            exist: true
        }
    }
}