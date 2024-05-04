#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Direction::Up => String::from("up"),
            Direction::Down => String::from("down"),
            Direction::Left => String::from("left"),
            Direction::Right => String::from("right"),
        }
    }
}
