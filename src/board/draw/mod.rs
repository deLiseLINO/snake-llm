mod snake_shape;
use self::snake_shape::SnakeShape;

use std::{collections::LinkedList, rc::Rc, time};

use crossterm::terminal::Clear;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::Color,
    symbols::{self, scrollbar, Marker},
    text::Text,
    widgets::{
        canvas::{Canvas, Painter, Points, Shape},
        Block, Borders, Clear as ClearWidget, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Widget,
    },
    Frame,
};

use crate::{point::Point, snake::Snake};

use super::RednerObjects;

pub enum UIMode {
    Game,
    GameWithDebug,
}

pub fn ui(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
    ui_mode: &UIMode,
    start_screen: bool,
) {
    match ui_mode {
        UIMode::Game => render_game(frame, render_objects, board_size, start_screen),
        UIMode::GameWithDebug => render_game_with_debug(frame, render_objects, board_size),
    }
}

fn render_game(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
    start_screen: bool,
) {
    let main_layout = main_layout(frame);

    let new_size = new_size_board(&main_layout, board_size);

    if start_screen {
        frame.render_widget(Block::bordered().title("Snake game"), main_layout[0]);
        let paragraph =
            Paragraph::new(Text::raw("Press arrows to start or 'q' to quit")).centered();
        let area = centered_rect(60, 20, main_layout[0]);
        frame.render_widget(paragraph, area);
    }

    if let Some(objects) = render_objects {
        frame.render_widget(score_block(objects.score), main_layout[1]);
        frame.render_widget(
            map_canvas(&objects.snake, &objects.food, new_size),
            main_layout[0],
        );
    }
}

fn render_game_with_debug(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
) {
    let main_layout = main_layout(frame);
    let game_and_debug_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(50), Constraint::Length(50)],
    )
    .split(main_layout[0]);

    let new_size = new_size_board(&main_layout, board_size);

    if let Some(objects) = render_objects {
        frame.render_widget(score_block(objects.score), main_layout[1]);
        frame.render_widget(
            map_canvas(&objects.snake, &objects.food, new_size),
            game_and_debug_layout[0],
        );
    }

    let paragraph = Paragraph::new(Text::raw("some debug messages"))
        .block(Block::bordered().title("Debug"))
        .scroll((0, 0));
    frame.render_widget(paragraph, game_and_debug_layout[1]);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        game_and_debug_layout[1].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut ScrollbarState::default(),
    );
}

fn main_layout(frame: &Frame) -> Rc<[Rect]> {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(0), Constraint::Length(1)],
    )
    .split(frame.size());

    main_layout
}

fn score_block(score: u16) -> impl Widget + 'static {
    Block::new()
        .title(format!("Score: {}", score))
        .title_alignment(Alignment::Center)
}

fn map_canvas(snake: &Snake, food: &Point, canvas_size: (u16, u16)) -> impl Widget + 'static {
    // dbg!(canvas_size);
    let snake_shape = SnakeShape::new(snake.get_list());
    let food = food.clone();

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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn new_size_board(main_layout: &Rc<[Rect]>, board_size: (&mut u16, &mut u16)) -> (u16, u16) {
    let new_size = terminal_size_to_board_size((main_layout[0].width, main_layout[0].height));

    (*board_size.0, *board_size.1) = new_size;
    new_size
}

fn terminal_size_to_board_size(terminal_size: (u16, u16)) -> (u16, u16) {
    (
        // - 1 cos of the borders
        (terminal_size.0 - 2) as u16,
        ((terminal_size.1 - 2) * 2) as u16,
    )
}
