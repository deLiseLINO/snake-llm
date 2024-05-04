mod board;
mod client;
mod config;
mod direction;
mod events;
mod game;
mod point;
mod snake;
use snake::Snake;

use crate::board::BoardTUI;

fn main() {
    let config = config::parse();
    let client = client::GroqClient::new(config.groq_client.url, config.groq_client.token);

    let snake = Snake::new();
    let board = BoardTUI::new();

    let mut game = game::Game::new(Box::new(board), snake, client);

    game.start();
}
