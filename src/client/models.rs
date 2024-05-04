use core::str;

use serde_derive::{Deserialize, Serialize};

pub static SYSTEM_PROMPT: &str = r#"
 You take coordinates of the snake head, snake_direction, coordinates of the food in JSON format: 
{
    "snake_direction": string,
    "snake_head_x": int,
    "snake_head_y": int,
    "food_x": int,
    "food_y": int,
}
 x = 0 y = 0 is left bottom corner of the board.
 it's very important for you to consider snake_direction, you cannot make command is opposite direction, it's very important!
 Your goal is to make snake_x = food_x and snake_y = food_y.
 You should give commands to the snake in JSON format:
 (JSON:
    {
        "commands": [
          {"command": string, "repeat": int},
          {"command": string, "repeat": int},
        ]
      }
remember that "up" is snake_y += 1, "down" is snake_y -= 1
also remember that "left" is snake_x -= 1 "right" is snake_x += 1
it's the most important! keep that in mind and don't forget.
don't give too much commands per request
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
    pub snake_direction: String,
    pub snake_head_x: i32,
    pub snake_head_y: i32,
    pub food_x: i32,
    pub food_y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputContent {
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub repeat: i32,
}
