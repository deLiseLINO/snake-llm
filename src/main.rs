mod board;
mod client;
mod config;
mod events;
mod game;
mod models;
mod snake;
use client::ApiClient;
use models::Provider;
use std::collections::HashMap;
use tui_logger::init_logger;

use snake::Snake;

use crate::board::BoardTUI;

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    let config = config::parse();
    let groq_client = client::groq::GroqClient::new(config.groq_client.url, config.groq_client.token);
    let mut clients: HashMap<Provider, Box<dyn ApiClient>> = HashMap::new();
    clients.insert(Provider::Groq, Box::new(groq_client));

    let snake = Snake::new();
    let board = BoardTUI::new();

    let mut game = game::Game::new(Box::new(board), snake, clients);

    game.start();
}
