mod board;
mod client;
mod config;
mod events;
mod game;
mod models;
mod snake;
use client::ApiClient;
use config::Config;
use models::Provider;

use std::collections::HashMap;
use tui_logger::init_logger;

use snake::Snake;

use crate::board::BoardTUI;

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    let config = config::parse();

    let mut clients: HashMap<Provider, Box<dyn ApiClient>> = HashMap::new();
    fill_clients_map(&mut clients, &config);

    let snake = Snake::new();
    let board = BoardTUI::new();

    let mut game = game::Game::new(Box::new(board), snake, clients);

    game.start();
}

fn fill_clients_map(clients: &mut HashMap<Provider, Box<dyn ApiClient>>, config: &Config) {
    if let Some(qroq_cfg) = &config.groq_client {
        clients.insert(
            Provider::Groq,
            Box::new(client::groq::GroqClient::new(
                qroq_cfg.url.clone(),
                qroq_cfg.token.clone(),
            )),
        );
    }

    if let Some(qroq_cfg) = &config.ollama_client {
        clients.insert(
            Provider::Groq,
            Box::new(client::ollama::OllamaClient::new(qroq_cfg.url.clone())),
        );
    }
}
