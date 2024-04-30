use crate::game::Board;
use crate::point::Point;
use crate::snake::Snake;
use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

use std::io::{stdout, Stdout};

use crossterm::{
    cursor, event, execute,
    style::{self, style},
    terminal, ExecutableCommand,
};

pub struct BoardCrossterm {
    stdout: Stdout,
    width: u16,
    height: u16,
    food: Point,
    snake: Rc<RefCell<Snake>>,
}

impl Board for BoardCrossterm {
    fn render(&mut self) {
        self.draw_background();
        self.draw_borders();
        self.draw_food();
        self.draw_snake();
    }

    fn prepare_ui(&mut self) {
        terminal::enable_raw_mode().unwrap();
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap()
            .execute(cursor::Hide)
            .unwrap();
    }

    fn clean_up(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}

impl BoardCrossterm {
    pub fn new(witdh: u16, height: u16, snake: Rc<RefCell<Snake>>) -> Self {
        Self {
            stdout: stdout(),
            width: witdh,
            height: height,
            food: Point::new(15, 5),
            snake: snake,
        }
    }

    fn draw_snake(&mut self) {
        let list = self.snake.borrow().get_list();
        let iter = list.into_iter();
        // self.stdout
        //     .execute(style::SetBackgroundColor(style::Color::Green))
        //     .unwrap();

        for point in iter {
            self.stdout
                .execute(cursor::MoveTo(point.x as u16, point.y as u16))
                .unwrap()
                .execute(style::Print("O"))
                .unwrap();
        }
    }

    fn draw_background(&mut self) {
        self.stdout.execute(style::ResetColor).unwrap();

        for y in 0..self.height + 1 {
            for x in 0..self.width + 1 {
                self.stdout
                    .execute(cursor::MoveTo(x, y))
                    .unwrap()
                    .execute(style::Print(" "))
                    .unwrap();
            }
        }
    }

    fn draw_borders(&mut self) {
        self.stdout
            .execute(style::SetBackgroundColor(style::Color::Cyan))
            .unwrap();

        for y in 1..self.height + 2 {
            self.stdout
                .execute(cursor::MoveTo(0, y))
                .unwrap()
                .execute(style::Print(" "))
                .unwrap()
                .execute(cursor::MoveTo(self.width + 1, y))
                .unwrap()
                .execute(style::Print(" "))
                .unwrap();
        }
        for x in 0..self.width + 2 {
            self.stdout
                .execute(cursor::MoveTo(x, 0))
                .unwrap()
                .execute(style::Print(" "))
                .unwrap()
                .execute(cursor::MoveTo(x, self.height + 1))
                .unwrap()
                .execute(style::Print(" "))
                .unwrap();
        }
    }

    fn draw_food(&mut self) {
        self.stdout.execute(style::ResetColor).unwrap();
        self.stdout
            .execute(style::SetForegroundColor(style::Color::White))
            .unwrap();
        self.stdout
            .execute(cursor::MoveTo(self.food.x as u16, self.food.y as u16))
            .unwrap()
            .execute(style::Print("•"))
            .unwrap();
    }
}
