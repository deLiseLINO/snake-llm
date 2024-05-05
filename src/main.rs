mod board;
mod client;
mod config;
mod direction;
mod events;
mod game;
mod models;
mod point;
mod snake;
use client::ApiClient;
use models::Provider;
use std::collections::HashMap;

use snake::Snake;

use crate::board::BoardTUI;

fn main() {
    let config = config::parse();
    let groq_client = client::GroqClient::new(config.groq_client.url, config.groq_client.token);
    let mut clients: HashMap<Provider, Box<dyn ApiClient>> = HashMap::new();
    clients.insert(Provider::Groq, Box::new(groq_client));

    let snake = Snake::new();
    let board = BoardTUI::new();

    let mut game = game::Game::new(Box::new(board), snake, clients);

    game.start();
}
