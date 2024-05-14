use crate::models::Direction;
use core::str;

use serde_derive::{Deserialize, Serialize};

pub static SYSTEM_PROMPT: &str = r#" 
Input: Take coordinates of the snake head and food in JSON format:
{"snake_head_x": int, "snake_head_y": int, "food_x": int, "food_y": int}
Goal: Make the snake head reach the food by giving commands always in JSON format:
{"commands": [{"command": string, "repeat": int}]}
Examples:
     * If food_y is greater than snake_head_y, move up: {"commands": [{"command": "up", "repeat": food_y - snake_head_y}]} 
     * If food_y is less than snake_head_y, move down: {"commands": [{"command": "down", "repeat": snake_head_y - food_y}]}
     * If food_x is greater than snake_head_x, move right: {"commands": [{"command": "right", "repeat": food_x - snake_head_x}]} 
     * If food_x is less than snake_head_x, move left: {"commands": [{"command": "left", "repeat": snake_head_x - food_x}]}
    Be careful not to confuse the direction! Think very carefully choosing the direction. Always calculate the distance, don't answer 10 - 5. Don't answer anything except JSON.
    Just calculate the right distance between the snake and the food and give commands.
For example if food_y is 41, and snake_head_y is 20, snake needs to go up 21 times.
Remember: Your goal is to make "snake_head_x" equal to "food_x" and "snake_head_y" equal to "food_y".
    Make sure to handle cases where the food is directly above, below, to the left or right of the snake head.
"#;
#[allow(dead_code)]
pub enum Role {
    User,
    System,
    Assistant,
}

impl Role {
    pub fn as_string(&self) -> String {
        match self {
            Role::User => "user".to_owned(),
            Role::System => "system".to_owned(),
            Role::Assistant => "assistant".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputContent {
    pub snake_head_x: i32,
    pub snake_head_y: i32,
    pub food_x: i32,
    pub food_y: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OutputContent {
    pub commands: Vec<Commands>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Commands {
    pub command: Direction,
    pub repeat: i32,
}
