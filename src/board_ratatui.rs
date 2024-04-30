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
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{
        self,
        canvas::{Canvas, Map, MapResolution, Painter, Shape},
        Block, Borders, Paragraph, Widget,
    },
    Frame, Terminal,
};

use crate::{game::Board, main, point::Point, snake::Snake};

pub struct BoardRataTUI {
    terminal: Rc<RefCell<Terminal<CrosstermBackend<io::Stdout>>>>,
    width: u16,
    height: u16,
    food: Point,
    snake: Rc<RefCell<Snake>>,
}

impl Board for BoardRataTUI {
    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
    }

    fn render(&mut self) {
        self.terminal
            .borrow_mut()
            .draw(|frame| self.ui(frame))
            .unwrap();
    }

    fn clean_up(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
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
            food: Point::new(0, 0),
            snake,
        }
    }

    fn ui(&self, frame: &mut Frame) {
        let s = frame.size();
        // dbg!(s);
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(1)],
        )
        .split(s);

        frame.render_widget(
            Block::new()
                .title("Score: 0")
                .title_alignment(Alignment::Center),
            main_layout[1],
        );

        frame.render_widget(self.map_canvas(), main_layout[0])
    }

    fn map_canvas(&self) -> impl Widget + 'static {
        let snake_shape = SnakeShape::new(self.snake.borrow_mut().get_list());

        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Snake game"))
            .marker(Marker::HalfBlock)
            .paint(move |ctx| {
                // ctx.draw(&Map {
                //     color: Color::Cyan,
                //     resolution: MapResolution::Low,
                // });
                ctx.draw(&snake_shape);
            })
            .x_bounds([-190.0, 190.0])
            .y_bounds([-100.0, 100.0])
    }
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(0), Constraint::Length(1)],
    )
    .split(frame.size());

    // frame.render_widget(
    //     Block::new()
    //         .borders(Borders::ALL)
    //         .title("Snake game")
    //         .title_alignment(Alignment::Center),
    //     main_layout[0],
    // );

    frame.render_widget(
        Block::new()
            .title("Score: 0")
            .title_alignment(Alignment::Center),
        main_layout[1],
    );

    // frame.render_widget(map_canvas(), main_layout[0])
    // frame.render_widget(
    //     Block::new().borders(Borders::TOP).title("Status Bar"),
    //     main_layout[2],
    // );

    // let inner_layout = Layout::new(
    //     Direction::Horizontal,
    //     [Constraint::Percentage(50), Constraint::Percentage(50)],
    // )
    // .split(main_layout[1]);
    // frame.render_widget(
    //     Block::default().borders(Borders::ALL).title("Left"),
    //     inner_layout[0],
    // );
    // frame.render_widget(
    //     Block::default().borders(Borders::ALL).title("Right"),
    //     inner_layout[1],
    // );
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
        // let mut i = -20;
        // while i < 20 {
        //     if let Some((x, y)) = painter.get_point(i as f64, 0 as f64) {
        //         painter.paint(x, y, Color::LightMagenta)
        //     }
        //     if let Some((x, y)) = painter.get_point(0 as f64, i as f64) {
        //         painter.paint(x, y, Color::LightMagenta)
        //     }
        //     i += 1;
        // }
    }
}
