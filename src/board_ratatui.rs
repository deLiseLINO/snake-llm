use std::{
    array::IntoIter,
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::LinkedList,
    io::{self, stdout},
    rc::Rc,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::{self, Marker},
    widgets::{
        self,
        canvas::{Canvas, Map, MapResolution, Painter, Points, Shape},
        Block, Borders, Padding, Paragraph, Widget,
    },
    Frame, Terminal,
};

use crate::{game::Board, main, point::Point, snake::Snake};

pub struct BoardRataTUI {
    terminal: Rc<RefCell<Terminal<CrosstermBackend<io::Stdout>>>>,
    width: u16,
    height: u16,
    score: u16,
    food: Point,
    snake: Rc<RefCell<Snake>>,
}

impl Board for BoardRataTUI {
    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
    }

    fn render(&mut self) {
        let terminal = Rc::clone(&self.terminal);
        terminal.borrow_mut().draw(|frame| self.ui(frame)).unwrap();
    }

    fn clean_up(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    fn get_food(&self) -> Point {
        self.food.clone()
    }

    fn change_food_position(&mut self) {
        self.food = Point::new(
            rand::thread_rng().gen_range(0..self.width) as i32,
            rand::thread_rng().gen_range(0..self.height) as i32,
        );
    }

    fn increment_score(&mut self) {
        self.score += 1;
    }

    fn get_size(&self) -> (&u16, &u16) {
        (&self.width, &self.height)
    }
}

impl BoardRataTUI {
    pub fn new(width: u16, height: u16, snake: Rc<RefCell<Snake>>) -> Self {
        Self {
            terminal: Rc::new(RefCell::new(
                Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            )),
            width,
            height,
            score: 0,
            food: Point::new(20, 20),
            snake,
        }
    }

    fn set_size(&mut self, width: u16, height: u16) {
        self.width = (width as f64 * 0.98) as u16;
        self.height = ((height * 2) as f64 * 0.94) as u16;
    }

    fn ui(&mut self, frame: &mut Frame) {
        let frame_size = frame.size();
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(1)],
        )
        .split(frame_size);

        self.set_size(main_layout[0].width, main_layout[0].height);

        frame.render_widget(
            Block::new()
                .title(format!("Score: {}", self.score))
                .title_alignment(Alignment::Center),
            main_layout[1],
        );

        frame.render_widget(self.map_canvas(), main_layout[0])
    }

    fn map_canvas(&self) -> impl Widget + 'static {
        let snake_shape: SnakeShape = SnakeShape::new(self.snake.borrow_mut().get_list());
        let food = self.food.clone();

        Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title("Snake game"),
            )
            .marker(Marker::HalfBlock)
            .paint(move |ctx| {
                ctx.draw(&snake_shape);
                ctx.draw(&Points {
                    coords: &[(food.x as f64, food.y as f64)],
                    color: Color::Green,
                })
            })
            .x_bounds([0.0, self.width as f64])
            .y_bounds([0.0, self.height as f64])
    }
}

struct SnakeShape {
    list: LinkedList<Point>,
}

impl SnakeShape {
    fn new(list: LinkedList<Point>) -> Self {
        Self { list: list }
    }
}

impl Shape for SnakeShape {
    fn draw(&self, painter: &mut Painter) {
        let iter = self.list.clone().into_iter();
        for point in iter {
            if let Some((x, y)) = painter.get_point(point.x as f64, point.y as f64) {
                painter.paint(x, y, Color::LightMagenta)
            }
        }
    }
}
