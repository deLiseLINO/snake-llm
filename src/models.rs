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

#[derive(PartialEq, Eq, Hash, Clone, EnumIter, Display)]
pub enum Provider {
    Groq,
    Ollama,
}

pub enum GameState {
    Running,
    NotStarted,
    GameOver,
}

pub struct RequestInfo {
    pub provider: Provider,
    pub input: InputContent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
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

    pub fn _as_string(&self) -> String {
        match self {
            Direction::Up => String::from("up"),
            Direction::Down => String::from("down"),
            Direction::Left => String::from("left"),
            Direction::Right => String::from("right"),
        }
    }
}

use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::client::models::InputContent;

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
