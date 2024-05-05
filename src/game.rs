use rand::Rng;
use ratatui::symbols::braille;

use crate::client::{models, GroqClient};
use crate::direction::Direction;
use crate::events::Command;
use crate::point::Point;
use crate::snake::Snake;
use crate::{client, events};

pub trait Board {
    fn prepare_ui(&mut self);
    fn render(&mut self, snake: Snake, food: Point, score: u16);
    fn render_start_screen(&mut self);
    fn clean_up(&mut self);
    fn get_size(&self) -> (u16, u16);
    fn debug(&mut self, line: String);
    fn reset_objects(&mut self);
}

pub struct Game {
    board: Box<dyn Board>,
    snake: Snake,
    food: Point,
    score: u16,
    client: client::GroqClient,
    commands: Vec<String>,
    start_screen: bool,
}

impl Game {
    pub fn new(board: Box<dyn Board>, snake: Snake, client: GroqClient) -> Self {
        Self {
            board,
            snake,
            food: Point::new(0, 0),
            score: 0,
            client,
            commands: Vec::new(),
            start_screen: true,
        }
    }

    pub fn start(&mut self) {
        self.board.prepare_ui();

        loop {
            let user_command = events::get_command();
            if let Some(command) = &user_command {
                match command {
                    Command::Turn(direction) => self.snake.change_direction(direction.clone()),
                    Command::Quit => break,
                }
            };
            if self.start_screen {
                self.board.render_start_screen();
                if user_command.is_some() {
                    self.start_screen = false;

                    let (width, height) = self.board.get_size();
                    self.snake
                        .set_head(Point::new_center(width as i32, height as i32));

                    self.food = Point::new_random(width as i32, height as i32);
                }
                continue;
            }
            self.board
                .render(self.snake.clone(), self.food.clone(), self.score.clone());

            // if let Some(command) = events::get_command() {

            // }

            // if self.commands.len() == 0 {
            //     let head = self.snake.get_head();
            //     let direction = self.snake.get_direction();
            //     self.do_commands_request(head, direction);
            //     if self.commands.len() == 0 {
            //         continue;
            //     }
            // }

            let growing = self.is_food_eaten();
            if growing {
                self.change_food_position();
                self.increment_score();
            }

            // if self.commands.len() > 0 {
            // let command = self.commands.remove(0);
            // self.snake.change_direction(match command.as_str() {
            //     "up" => Direction::Up,
            //     "down" => Direction::Down,
            //     "left" => Direction::Left,
            //     "right" => Direction::Right,
            //     _ => Direction::Down,
            // });
            // }

            if self.crossed_borders_or_eat_itself() {
                self.start_screen = true;
                self.board.reset_objects();
                self.score = 0;
                self.snake = Snake::new();
                continue;
            };
            self.snake.moving(growing);
        }
        self.board.clean_up();
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

    fn do_commands_request(&mut self, s_head: Point, direction: Direction) {
        let food = self.food.clone();
        // let (width, height) = self.board.get_size();

        let commands = self.client.snake_commands(models::InputContent {
            snake_direction: direction.as_string(),
            snake_head_x: s_head.x,
            snake_head_y: s_head.y,
            food_x: food.x,
            food_y: food.y,
        });
        let res = match commands {
            Ok(res) => res,
            Err(e) => {
                self.debug(e);
                return;
            }
        };

        self.board.debug(format!("{:?}", res.commands));

        for c in res.commands {
            for _ in 0..c.repeat {
                self.commands.push(c.command.clone());
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

    fn debug(&mut self, line: String) {
        self.board.debug(line);
    }
}
