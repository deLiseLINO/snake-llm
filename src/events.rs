use std::time::{self};

use crossterm::event;

use crate::models::{Direction, GameMod, Provider};

pub enum Command {
    Quit,
    Turn(Direction),
    SelectMode,
}

pub fn get_command() -> Option<Command> {
    let wait_for = time::Duration::from_millis(30);
    let key_event = wait_for_key_event(wait_for)?;
    match key_event.code {
        event::KeyCode::Up => Some(Command::Turn(Direction::Up)),
        event::KeyCode::Down => Some(Command::Turn(Direction::Down)),
        event::KeyCode::Left => Some(Command::Turn(Direction::Left)),
        event::KeyCode::Right => Some(Command::Turn(Direction::Right)),
        event::KeyCode::Char('q') => Some(Command::Quit),
        event::KeyCode::Char('m') => Some(Command::SelectMode),
        _ => None,
    }
}

pub fn get_mod_command() -> Option<GameMod> {
    let wait_for = time::Duration::from_millis(30);
    let key_event = wait_for_key_event(wait_for)?;
    match key_event.code {
        event::KeyCode::Char('1') => Some(GameMod::Player),
        event::KeyCode::Char('2') => Some(GameMod::Api(Provider::Groq)),
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
