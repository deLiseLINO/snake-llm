use std::io::{self, stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{backend::CrosstermBackend, text::Line, Terminal};

use crate::{game::Board, point::Point, snake::Snake};

use self::draw::ui;

mod draw;

pub struct BoardTUI<'a> {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    std_debug: Vec<Line<'a>>,
    render_objects: Option<RednerObjects>,
    width: u16,
    height: u16,
}

// todo: try add lifetimes
#[derive(Clone)]
struct RednerObjects {
    snake: Snake,
    food: Point,
    score: u16,
}

impl<'a> Board for BoardTUI<'a> {
    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();

        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| ui(frame, &self.render_objects, board_size))
            .unwrap();
    }

    fn render(&mut self, snake: Snake, food: Point, score: u16) {
        self.render_objects = Some(RednerObjects {
            snake: snake,
            food: food,
            score: score,
        });

        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| ui(frame, &self.render_objects, board_size))
            .unwrap();
    }

    fn clean_up(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    fn get_size(&self) -> (&u16, &u16) {
        (&self.width, &self.height)
    }

    fn debug(&mut self, line: String) {
        self.std_debug.push(Line::from(line));
    }
}

impl<'a> BoardTUI<'a> {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            std_debug: Vec::new(),
            render_objects: None,
            width: 0,
            height: 0,
        }
    }
}
