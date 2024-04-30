mod board_crossterm;
mod board_ratatui;
mod direction;
mod events;
mod game;
mod point;
mod snake;
use snake::Snake;

use crate::board_crossterm::BoardCrossterm;
use crate::board_ratatui::BoardRataTUI;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let snake = Rc::new(RefCell::new(Snake::new(0, 0)));
    // let board = BoardCrossterm::new(100, 50, Rc::clone(&snake));
    let board = BoardRataTUI::new(100, 50, Rc::clone(&snake));

    let mut game = game::Game::new(Box::new(board), Rc::clone(&snake));

    game.start();
}
