mod board;
mod client;
mod config;
mod events;
mod game;
mod models;
mod snake;
use client::ApiClient;
use config::Config;
use models::{Provider, RequestInfo};

use log::*;
use std::{collections::HashMap, sync::mpsc, thread, time::Duration};
use tui_logger::init_logger;

#[allow(unused_imports)]
use rstest_reuse;

use snake::Snake;

use crate::board::BoardTUI;

fn main() {
    init_logger(log::LevelFilter::Trace).unwrap();
    let config = config::parse();

    let mut clients: HashMap<Provider, Box<dyn ApiClient>> = HashMap::new();
    fill_clients_map(&mut clients, &config);

    let snake = Snake::new();
    let board = BoardTUI::new();

    let (tx_request, rx_request) = mpsc::sync_channel::<RequestInfo>(0);
    let (tx_response, rx_response) = mpsc::sync_channel(0);

    thread::spawn(move || {
        let mut clients: HashMap<Provider, Box<dyn ApiClient>> = HashMap::new();
        fill_clients_map(&mut clients, &config);

        loop {
            let req_info = rx_request.recv();

            if let Ok(req_info) = req_info {
                let input = req_info.input;

                if let Some(client) = clients.get_mut(&req_info.provider) {
                    let commands = client.snake_commands(input);

                    match commands {
                        Ok(res) => {
                            info!("{:?}", res.commands);
                            tx_response.send(res).unwrap();
                        }
                        Err(e) => {
                            error!("{} \n waiting for 5 sec", e);
                            thread::sleep(Duration::from_secs(5))
                        }
                    };
                } else {
                    error!(
                        "No config for provider: {:?} \n Please provide config to config.yaml file",
                        &req_info.provider.to_string()
                    );
                    thread::sleep(Duration::from_secs(60));
                }
            } else {
                break;
            }
        }
    });

    let mut game = game::Game::new(Box::new(board), snake, tx_request, rx_response);

    game.start();
}

fn fill_clients_map(clients: &mut HashMap<Provider, Box<dyn ApiClient>>, config: &Config) {
    if let Some(cfg) = &config.groq_client {
        clients.insert(
            Provider::Groq,
            Box::new(client::groq::GroqClient::new(
                cfg.url.clone(),
                cfg.token.clone(),
            )),
        );
    }

    if let Some(cfg) = &config.ollama_client {
        clients.insert(
            Provider::Ollama,
            Box::new(client::ollama::OllamaClient::new(cfg.url.clone(), cfg.model.clone())),
        );
    }
}
