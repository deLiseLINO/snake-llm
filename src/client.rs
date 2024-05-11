use serde_derive::{Deserialize, Serialize};

use crate::client::models::{Choice, InputContent};

use self::models::OutputContent;

pub mod groq;
pub mod models;

#[derive(Serialize, Deserialize, Debug)]
struct GroqRequest {
    messages: Vec<models::Message>,
    model: String,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GroqResponse {
    choices: Vec<Choice>,
}

pub trait ApiClient {
    fn snake_commands(&mut self, input: InputContent) -> Result<OutputContent, String>;
}
