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
            let start = Instant::now();
            self.board.render();
            let mut snake = self.snake.borrow_mut();
            snake.moving();
            // dbg!(snake.get_list());
            if let Some(command) = events::get_command() {
                match command {
                    events::Command::Turn(direction) => snake.change_direction(direction),
                    events::Command::Quit => done = true,
                }
            }
            // thread::sleep_ms(500);
            let duration = start.elapsed();
            // dbg!(duration);
        }
        self.board.clean_up();
    }
}
