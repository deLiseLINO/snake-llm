use std::io::{self, stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{backend::CrosstermBackend, text::Line, Terminal};

use crate::{game::Board, point::Point, snake::Snake};

use self::draw::{ui, UIMode};

mod draw;

pub struct BoardTUI<'a> {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    std_debug: Vec<Line<'a>>,
    render_objects: Option<RednerObjects>,
    ui_mode: UIMode,
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

        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            disable_raw_mode().unwrap();
            crossterm::execute!(io::stdout(), LeaveAlternateScreen).unwrap();
            original_hook(panic);
        }));

        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| ui(frame, &self.render_objects, board_size, &self.ui_mode, true))
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
            .draw(|frame| {
                ui(
                    frame,
                    &self.render_objects,
                    board_size,
                    &self.ui_mode,
                    false,
                )
            })
            .unwrap();
    }

    fn clean_up(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn debug(&mut self, line: String) {
        self.std_debug.push(Line::from(line));
    }

    fn render_start_screen(&mut self) {
        let board_size = (&mut self.width, &mut self.height);

        self.terminal
            .draw(|frame| ui(frame, &self.render_objects, board_size, &self.ui_mode, true))
            .unwrap();
    }

    fn reset_objects(&mut self) {
        self.render_objects = None;
    }
}

impl<'a> BoardTUI<'a> {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            std_debug: Vec::new(),
            render_objects: None,
            ui_mode: UIMode::Game,
            width: 0,
            height: 0,
        }
    }
}
