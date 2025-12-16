#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn as_itup(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn as_utup(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

pub trait Boundary<T> {
    fn in_bounds(&self, boundary: T) -> bool;
}

impl Boundary<(i32, i32)> for Point {
    fn in_bounds(&self, boundary: (i32, i32)) -> bool {
        self.x >= 0 && self.x < boundary.0 &&
        self.y >= 0 && self.y < boundary.1
    }
}

impl Boundary<(usize, usize)> for Point {
    fn in_bounds(&self, boundary: (usize, usize)) -> bool {
        (self.x as usize).lt(&boundary.0) && (self.y as usize).lt(&boundary.1)
    }
}