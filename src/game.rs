use crate::direction::Direction;
use crate::events;
use crate::point::Point;
use crate::snake::Snake;
use std::cell::RefCell;
use std::option::Iter;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

pub trait Board {
    fn prepare_ui(&mut self);
    fn render(&mut self);
    fn clean_up(&mut self);
    fn get_food(&self) -> Point;
    fn change_food_position(&mut self);
    fn increment_score(&mut self);
    fn get_size(&self) -> (&u16, &u16);
}

pub struct Game {
    board: Box<dyn Board>,
    snake: Rc<RefCell<Snake>>,
}

impl Game {
    pub fn new(board: Box<dyn Board>, snake: Rc<RefCell<Snake>>) -> Self {
        Self { board, snake }
    }

    pub fn start(&mut self) {
        self.board.prepare_ui();
        let mut done = false;
        while !done {
            let growing = self.is_food_eaten();
            if growing {
                self.board.change_food_position();
                self.board.increment_score();
            }

            if let dead = self.crossed_borders_or_eat_itself() {
                done = dead;
            }

            self.board.render();
            let mut snake = self.snake.borrow_mut();
            snake.moving(growing);
            if let Some(command) = events::get_command() {
                match command {
                    events::Command::Turn(direction) => snake.change_direction(direction),
                    events::Command::Quit => done = true,
                }
            }
        }
        self.board.clean_up();
    }

    fn is_food_eaten(&self) -> bool {
        let snake = self.snake.borrow();
        let head = snake.get_head();
        let food = self.board.get_food();
        if (head.x, head.y) == (food.x, food.y) {
            return true;
        }
        false
    }

    fn crossed_borders_or_eat_itself(&self) -> bool {
        let mut snake_iter = self.snake.borrow().get_list().into_iter();
        let head = snake_iter.next().unwrap();
        let (width, height) = self.board.get_size();
        for point in snake_iter {
            if head.x == point.x && head.y == point.y {
                return true;
            }
        }
        head.x < 0 || head.y < 0 || head.x >= *width as i32 || head.y >= *height as i32
    }
}
