use std::collections::LinkedList;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Color,
    symbols::{self, Marker},
    widgets::{
        canvas::{Canvas, Painter, Points, Shape},
        Block, Borders, Widget,
    },
    Frame,
};

use crate::point::Point;

use super::RednerObjects;

pub fn ui(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(0), Constraint::Length(1)],
    )
    .split(frame.size());

    let game_and_debug_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(0), Constraint::Length(100)],
    )
    .split(main_layout[0]);

    let new_size = terminal_size_to_board_size((
        game_and_debug_layout[0].width,
        game_and_debug_layout[0].height,
    ));

    (*board_size.0, *board_size.1) = new_size;

    if let Some(objects) = render_objects {
        frame.render_widget(
            Block::new()
                .title(format!("Score: {}", objects.score))
                .title_alignment(Alignment::Center),
            main_layout[1],
        );

        frame.render_widget(map_canvas(objects, new_size), game_and_debug_layout[0]);
    }

    // let paragraph = Paragraph::new(self.std_debug.clone())
    //     .gray()
    //     .block(create_block("Debug"))
    //     .scroll((0, 0));
    // frame.render_widget(paragraph, game_and_debug_layout[1]);

    // frame.render_stateful_widget(
    //     Scrollbar::new(ScrollbarOrientation::VerticalLeft)
    //         .symbols(scrollbar::VERTICAL)
    //         .begin_symbol(None)
    //         .track_symbol(None)
    //         .end_symbol(None),
    //     game_and_debug_layout[1].inner(&Margin {
    //         vertical: 1,
    //         horizontal: 0,
    //     }),
    //     &mut ScrollbarState::default(),
    // );
}

fn terminal_size_to_board_size(terminal_size: (u16, u16)) -> (u16, u16) {
    (
        (terminal_size.0 as f32 * 0.97) as u16,
        ((terminal_size.1 * 2) as f32 * 0.95) as u16,
    )
}

fn map_canvas(render_objects: &RednerObjects, canvas_size: (u16, u16)) -> impl Widget + 'static {
    let snake_shape = SnakeShape::new(render_objects.snake.get_list());
    let food = render_objects.food.clone();

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
        .x_bounds([0.0, canvas_size.0 as f64])
        .y_bounds([0.0, canvas_size.1 as f64])
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
