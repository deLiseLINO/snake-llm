#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get_point(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }
}
