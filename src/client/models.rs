use crate::models::Direction;
use core::str;

use serde_derive::{Deserialize, Serialize};

// pub static SYSTEM_PROMPT: &str = "You take coordinates of the snake head, coordinates of the food in JSON format: { \"snake_head_x\": int, \"snake_head_y\": int, \"food_x\": int, \"food_y\": int, }. 0, 0 is the coorinates of left bottom corner. Your goal is to make snake_x = food_x and snake_y = food_y. You should give commands to the snake in JSON format: { \"commands\": [ {\"command\": string, \"repeat\": int} ] }. remember that \"up\" is snake_y += 1, \"down\" is snake_y -= 1. also remember that \"left\" is snake_x -= 1 \"right\" is snake_x += 1";

pub static SYSTEM_PROMPT: &str = r#"Snake Game Instructions

Input: Take coordinates of the snake head and food in JSON format: {"snake_head_x": int, "snake_head_y": int, "food_x": int, "food_y": int}

Note: The origin (0, 0) is the bottom-left corner.

Goal: Make the snake head reach the food by giving commands in JSON format: {"commands": [{"command": string, "repeat": int}]}

Key Requirement: You need to provide enough commands in a single request to reach the food. The snake should reach the food in one step, so plan your commands accordingly.

Important: It is crucial to give the right commands to reach the food. One wrong move can lead to failure. Please think carefully before sending your commands.

Commands:

"up": Move the snake up (increment "snake_y" by 1)
"down": Move the snake down (decrement "snake_y" by 1)
"left": Move the snake left (decrement "snake_x" by 1)
"right": Move the snake right (increment "snake_x" by 1)
Remember: Your goal is to make "snake_x" equal to "food_x" and "snake_y" equal to "food_y"."#;

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

#[allow(dead_code)]
pub enum Models {
    Llama3b70,
    LLama3b8,
    Mixtrel8b,
}

impl Models {
    pub fn as_string(&self) -> String {
        match self {
            Models::LLama3b8 => "llama3-8b-8192".to_owned(),
            Models::Llama3b70 => "llama3-70b-8192".to_owned(),
            Models::Mixtrel8b => "Mixtral-8x7b-32768".to_owned(),
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
    // pub snake_direction: String,
    pub snake_head_x: i32,
    pub snake_head_y: i32,
    pub food_x: i32,
    pub food_y: i32,
}

// pub struct InputContent {
//     // pub snake_direction: String,
//     pub snake: (i32, i32),
//     pub food: (i32, i32),
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputContent {
    pub commands: Vec<Commands>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands {
    pub command: Direction,
    pub repeat: i32,
}
