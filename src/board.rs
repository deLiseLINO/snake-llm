use crate::point::Point;
use crate::snake::Snake;
use crate::{direction, events};

use std::{
    io::{stdout, Stdout},
    process::Command,
    thread,
};

use crossterm::{
    cursor, event, execute,
    style::{self, style},
    terminal, ExecutableCommand,
};

pub struct Board {
    stdout: Stdout,
    width: u16,
    height: u16,
    food: Point,
    snake: Snake,
}

impl Board {
    pub fn new(witdh: u16, height: u16) -> Self {
        Self {
            stdout: stdout(),
            width: witdh,
            height: height,
            food: Point::new(15, 5),
            snake: Snake::new(witdh / 2, height / 2),
        }
    }

    pub fn start(&mut self) {
        self.prepare_ui();
        let mut done = false;
        while !done {
            self.snake.moving();
            self.render();
            if let Some(command) = events::get_command() {
                match command {
                    events::Command::Turn(direction) => self.snake.change_direction(direction),
                    events::Command::Quit => done = true,
                }
            }
            thread::sleep_ms(100)
        }
    }

    fn prepare_ui(&mut self) {
        terminal::enable_raw_mode().unwrap();
        self.stdout
            // .execute(terminal::SetSize(self.width + 10, self.height + 10))
            // .unwrap()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap()
            .execute(cursor::Hide)
            .unwrap();
    }

    fn render(&mut self) {
        self.draw_background();
        self.draw_borders();
        self.draw_food();
        self.draw_snake();
    }

    fn draw_snake(&mut self) {
        let list = self.snake.get_list();
        let iter = list.into_iter();
        // self.stdout
        //     .execute(style::SetBackgroundColor(style::Color::Green))
        //     .unwrap();

        for point in iter {
            self.stdout
                .execute(cursor::MoveTo(point.x, point.y))
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
            .execute(cursor::MoveTo(self.food.x, self.food.y))
            .unwrap()
            .execute(style::Print("•"))
            .unwrap();
    }
}
