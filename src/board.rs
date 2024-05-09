use std::{
    io::{self, stdout},
    thread::scope,
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{backend::CrosstermBackend, text::Line, Terminal};

use crate::{game::Board, models::Point, snake::Snake};

mod draw;

use crate::models::{GameState, UIMode};

use self::draw::ui;

pub struct BoardTUI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    ui_mode: UIMode,
    width: u16,
    height: u16,
}

#[derive(Clone)]
struct RednerObjects<'a> {
    snake: &'a Snake,
    food: &'a Point,
}

impl Board for BoardTUI {
    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();

        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            disable_raw_mode().unwrap();
            crossterm::execute!(io::stdout(), LeaveAlternateScreen).unwrap();
            original_hook(panic);
        }));
        self.autoresize();
        let size = self.terminal.get_frame().size();
        self.width = size.width;
        self.height = size.height;
    }

    fn render_game(&mut self, snake: &Snake, food: &Point, score: u16) {
        let render_objects = Some(RednerObjects {
            snake: snake,
            food: food,
        });

        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| {
                ui(
                    frame,
                    &render_objects,
                    board_size,
                    &self.ui_mode,
                    GameState::Running,
                    score,
                )
            })
            .unwrap();
    }

    fn render_start_screen(&mut self) {
        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| {
                ui(
                    frame,
                    &None,
                    board_size,
                    &self.ui_mode,
                    GameState::NotStarted,
                    0,
                )
            })
            .unwrap();
    }

    fn render_game_over(&mut self, score: u16) {
        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| {
                ui(
                    frame,
                    &None,
                    board_size,
                    &self.ui_mode,
                    GameState::GameOver,
                    score,
                )
            })
            .unwrap();
    }

    fn render_selecting_mode(&mut self) {
        self.terminal
            .draw(|frame| {
                ui(
                    frame,
                    &None,
                    (&mut self.width, &mut self.height),
                    &self.ui_mode,
                    GameState::NotStarted,
                    0,
                )
            })
            .unwrap();
    }

    fn update_mode(&mut self, mode: UIMode) {
        self.ui_mode = mode;
    }

    fn get_mode(&self) -> UIMode {
        self.ui_mode.clone()
    }

    fn clean_up(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn autoresize(&mut self) {
        self.terminal.autoresize().unwrap();
    }
}

impl BoardTUI {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            ui_mode: UIMode::Game,
            width: 0,
            height: 0,
        }
    }
}
