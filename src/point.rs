use rand::Rng;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new_random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0..width), rng.gen_range(0..height))
    }

    pub fn new_center(width: i32, height: i32) -> Self {
        Self::new(width / 2, height / 2)
    }
}
