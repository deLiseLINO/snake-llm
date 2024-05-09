#[derive(PartialEq, Clone)]
pub enum UIMode {
    Game,
    GameWithDebug,
    SelectingMode,
}

pub enum GameMod {
    Player,
    Api(Provider),
}

#[derive(PartialEq, Eq, Hash)]
pub enum Provider {
    Groq,
}

pub enum GameState {
    Running,
    NotStarted,
    GameOver,
}

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
