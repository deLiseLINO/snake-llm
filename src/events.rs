use std::time::{self};

use crossterm::event;

use crate::models::{Direction, GameMod, Provider};

pub enum Command {
    Quit,
    Turn(Direction),
    SelectingModeCommand(GameMod),
    SelectMode,
    AnyKey,
}

pub fn get_command() -> Option<Command> {
    let wait_for = time::Duration::from_millis(15);
    let key_event = wait_for_key_event(wait_for)?;
    match key_event.code {
        event::KeyCode::Up => Some(Command::Turn(Direction::Up)),
        event::KeyCode::Down => Some(Command::Turn(Direction::Down)),
        event::KeyCode::Left => Some(Command::Turn(Direction::Left)),
        event::KeyCode::Right => Some(Command::Turn(Direction::Right)),
        event::KeyCode::Char('q') => Some(Command::Quit),
        event::KeyCode::Char('m') => Some(Command::SelectMode),

        // Selecting mode
        event::KeyCode::Char('1') => Some(Command::SelectingModeCommand(GameMod::Player)),
        event::KeyCode::Char('2') => {
            Some(Command::SelectingModeCommand(GameMod::Api(Provider::Groq)))
        }
        _ => Some(Command::AnyKey),
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
