mod snake_shape;
use self::snake_shape::SnakeShape;

use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::{self, Marker},
    text::{Line, Text},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph, Widget,
    },
    Frame,
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

use crate::{
    models::{GameState, Point, UIMode},
    snake::Snake,
};

use super::RednerObjects;

pub fn ui(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
    ui_mode: &UIMode,
    game_state: GameState,
    score: u16,
) {
    match ui_mode {
        UIMode::Game => render_game(frame, render_objects, board_size, game_state, score),
        UIMode::GameWithDebug => {
            render_game_with_debug(frame, render_objects, board_size, game_state, score)
        }
        UIMode::SelectingMode => {
            let main_layout = main_layout(frame);

            frame.render_widget(Block::bordered().title("Snake game"), main_layout[0]);
            let paragraph = Paragraph::new(Text::raw("Select game mode")).centered();
            let area = centered_rect(60, 20, main_layout[0]);
            frame.render_widget(paragraph, area);
        }
    }
}

fn render_game(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
    game_state: GameState,
    score: u16,
) {
    let main_layout = main_layout(frame);

    render_game_state(
        frame,
        game_state,
        render_objects,
        score,
        board_size,
        main_layout[0],
        main_layout[1],
    );
}

fn render_game_state(
    frame: &mut Frame,
    game_state: GameState,
    render_objects: &Option<RednerObjects>,
    score: u16,
    board_size: (&mut u16, &mut u16),
    canvas_layout: Rect,
    score_layout: Rect,
) {
    let new_size = new_size_board(&canvas_layout, board_size);
    let mut content = vec![Line::from("Press any arrow to start or 'q' to quit".bold())];

    match game_state {
        GameState::Running => {
            if let Some(objects) = render_objects {
                frame.render_widget(score_block(score), score_layout);
                frame.render_widget(
                    map_canvas(&objects.snake, &objects.food, new_size),
                    canvas_layout,
                );
            }
            return;
        }
        GameState::NotStarted => {}
        GameState::GameOver => {
            content.push(Line::from(""));
            content.push(Line::from(
                format!("Game over! your score was: {}", score).bold(),
            ));
        }
    }
    frame.render_widget(Block::bordered().title("Snake game"), canvas_layout);
    let paragraph = Paragraph::new(content).centered();
    let area = centered_rect(60, 20, canvas_layout);
    frame.render_widget(paragraph, area);
}

fn render_game_with_debug(
    frame: &mut Frame,
    render_objects: &Option<RednerObjects>,
    board_size: (&mut u16, &mut u16),
    game_state: GameState,
    score: u16,
) {
    let main_layout = main_layout(frame);
    let game_and_debug_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Min(50), Constraint::Length(50)],
    )
    .split(main_layout[0]);

    render_game_state(
        frame,
        game_state,
        render_objects,
        score,
        board_size,
        game_and_debug_layout[0],
        main_layout[1],
    );

    let log = TuiLoggerWidget::default()
        .block(Block::bordered().title("Debug"))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(false)
        .output_timestamp(None)
        .output_file(false)
        .output_line(false);

    frame.render_widget(log, game_and_debug_layout[1]);
}

fn main_layout(frame: &Frame) -> Rc<[Rect]> {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(0), Constraint::Length(1)],
    )
    .split(frame.size());

    main_layout
}

fn score_block(score: u16) -> impl Widget {
    Block::new()
        .title(format!("Score: {}", score))
        .title_alignment(Alignment::Center)
}

fn map_canvas(snake: &Snake, food: &Point, canvas_size: (u16, u16)) -> impl Widget {
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

fn new_size_board(main_layout: &Rect, board_size: (&mut u16, &mut u16)) -> (u16, u16) {
    let new_size = terminal_size_to_board_size((main_layout.width, main_layout.height));

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
