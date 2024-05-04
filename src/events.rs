use std::time::{self};

use crate::direction::Direction;
use crossterm::event;

pub enum Command {
    Quit,
    Turn(Direction),
}

pub fn get_command() -> Option<Command> {
    let wait_for = time::Duration::from_millis(50);
    let key_event = wait_for_key_event(wait_for)?;
    match key_event.code {
        event::KeyCode::Up => Some(Command::Turn(Direction::Up)),
        event::KeyCode::Down => Some(Command::Turn(Direction::Down)),
        event::KeyCode::Left => Some(Command::Turn(Direction::Left)),
        event::KeyCode::Right => Some(Command::Turn(Direction::Right)),
        event::KeyCode::Char('q') => Some(Command::Quit),
        _ => None,
    }
}

fn wait_for_key_event(wait_for: time::Duration) -> Option<event::KeyEvent> {
    if event::poll(wait_for).ok()? {
        let event = event::read().ok()?;
        if let event::Event::Key(key_event) = event {
            return Some(key_event);
        }
    }
    None
}
