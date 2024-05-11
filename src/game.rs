use std::sync::mpsc::{Receiver, SyncSender};

use log::*;
use rand::Rng;

use crate::client::models::OutputContent;
use crate::client::{self};
use crate::events::Command;
use crate::models::{GameMod, GameState, Point, Provider, RequestInfo, UIMode};
use crate::snake::Snake;
use crate::{events, models};

pub trait Board {
    fn prepare_ui(&mut self);
    fn render_game(&mut self, snake: &Snake, food: &Point, score: u16);
    fn render_start_screen(&mut self);
    fn render_game_over(&mut self, score: u16);
    fn render_selecting_mode(&mut self);
    fn clean_up(&mut self);
    fn get_size(&self) -> (u16, u16);
    fn update_mode(&mut self, mode: UIMode);
    fn get_mode(&self) -> UIMode;
    fn autoresize(&mut self);
}

pub struct Game {
    board: Box<dyn Board>,
    snake: Snake,
    food: Point,
    score: u16,
    client: Option<Provider>,
    commands: Vec<models::Direction>,
    game_state: GameState,
    game_mod: GameMod,
    tx_request: SyncSender<RequestInfo>,
    rx_response: Receiver<OutputContent>,
}

impl Game {
    pub fn new(
        board: Box<dyn Board>,
        snake: Snake,
        tx_request: SyncSender<RequestInfo>,
        rx_response: Receiver<OutputContent>,
    ) -> Self {
        Self {
            board,
            snake,
            food: Point::new(0, 0),
            score: 0,
            client: None,
            commands: Vec::new(),
            game_state: GameState::NotStarted,
            game_mod: GameMod::Player,
            tx_request,
            rx_response,
        }
    }

    pub fn start(&mut self) {
        self.board.prepare_ui();
        self.new_game();

        loop {
            let user_command = events::get_command();
            if let Some(command) = &user_command {
                match command {
                    Command::Quit => break,
                    Command::SelectMode => {
                        self.board.update_mode(UIMode::SelectingMode);
                    }
                    _ => (),
                }
            }

            if self.board.get_mode() == UIMode::SelectingMode {
                self.handle_selecting_mode(&user_command);
                continue;
            }

            match self.game_state {
                GameState::NotStarted => {
                    if user_command.is_some() {
                        self.game_state = GameState::Running;
                        self.board.render_game(&self.snake, &self.food, self.score);
                        continue;
                    }
                    self.board.render_start_screen();
                }
                GameState::Running => {
                    if self.crossed_borders_or_eat_itself() {
                        self.game_state = GameState::GameOver;
                        continue;
                    };

                    if self.is_food_eaten() {
                        self.change_food_position();
                        self.increment_score();
                        self.snake.grow();
                    }
                    self.board.render_game(&self.snake, &self.food, self.score);
                }
                GameState::GameOver => {
                    self.board.render_game_over(self.score);
                    if user_command.is_some() {
                        self.game_state = GameState::Running;
                        self.new_game();
                        continue;
                    }
                }
            }

            match &self.game_mod {
                GameMod::Player => {
                    if matches!(self.game_state, GameState::Running) {
                        if let Some(command) = &user_command {
                            match command {
                                Command::Turn(direction) => {
                                    self.snake.change_direction(direction.clone());
                                }
                                _ => (),
                            }
                        };
                    }
                }
                GameMod::Api(_provider) => {
                    if matches!(self.game_state, GameState::Running) {
                        let output = self.rx_response.try_recv();
                        if let Ok(output) = output {
                            for c in output.commands {
                                for _ in 0..c.repeat {
                                    self.commands.push(c.command.clone());
                                }
                            }
                        }
                        if self.commands.len() == 0 {
                            self.do_commands_request(self.snake.get_head())
                        }
                        if self.commands.len() > 0 {
                            let command = self.commands.remove(0);
                            self.snake.change_direction(command);
                        } else {
                            continue;
                        }
                    }
                }
            }

            if matches!(self.game_state, GameState::Running) {
                self.snake.moving();
            }
        }
        self.board.clean_up();
    }

    fn handle_selecting_mode(&mut self, user_command: &Option<Command>) {
        if let Some(command) = &user_command {
            match command {
                Command::SelectingModeCommand(GameMod::Player) => {
                    self.game_state = GameState::NotStarted;
                    self.board.update_mode(UIMode::Game);
                    self.game_mod = GameMod::Player;
                }
                Command::SelectingModeCommand(GameMod::Api(provider)) => {
                    self.game_state = GameState::NotStarted;
                    self.board.update_mode(UIMode::GameWithDebug);
                    self.game_mod = GameMod::Api(provider.clone());
                    self.client = Some(provider.clone());
                }
                _ => (),
            }
        } else {
            self.board.render_selecting_mode();
        }
        self.new_game();
    }

    fn change_food_position(&mut self) {
        let (width, height) = self.board.get_size();
        self.food = Point::new(
            rand::thread_rng().gen_range(0..width) as i32,
            rand::thread_rng().gen_range(0..height) as i32,
        );
    }

    fn increment_score(&mut self) {
        self.score += 1;
    }

    fn new_game(&mut self) {
        self.snake.reset();
        self.commands.clear();
        self.score = 0;

        self.board.render_game(&self.snake, &self.food, self.score);
        let (width, height) = self.board.get_size();
        self.snake
            .set_head(Point::new_center(width as i32, height as i32));

        self.food = Point::new_random(width as i32, height as i32);
    }

    fn do_commands_request(&mut self, s_head: Point) {
        if let Some(client) = self.client.clone() {
            let food = self.food.clone();
            let input = client::models::InputContent {
                snake_head_x: s_head.x,
                snake_head_y: s_head.y,
                food_x: food.x,
                food_y: food.y,
            };

            let req_info = models::RequestInfo {
                provider: client,
                input: input,
            };

            if let Ok(_) = self.tx_request.try_send(req_info) {
                info!("Sending request...")
            }
        }
    }

    fn is_food_eaten(&self) -> bool {
        let head = self.snake.get_head();
        let food = self.food.clone();
        if (head.x, head.y) == (food.x, food.y) {
            return true;
        }
        false
    }

    fn crossed_borders_or_eat_itself(&self) -> bool {
        let mut snake_iter = self.snake.get_list().into_iter();
        let head = snake_iter.next().unwrap();
        let (width, height) = self.board.get_size();
        for point in snake_iter {
            if head.x == point.x && head.y == point.y {
                return true;
            }
        }
        head.x < 0 || head.y < 0 || head.x >= width as i32 || head.y >= height as i32
    }
}
