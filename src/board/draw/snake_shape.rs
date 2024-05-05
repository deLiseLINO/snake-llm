use std::collections::LinkedList;

use ratatui::{style::Color, widgets::canvas::{Painter, Shape}};

use crate::point::Point;

pub struct SnakeShape {
    list: LinkedList<Point>,
}

impl SnakeShape {
    pub fn new(list: LinkedList<Point>) -> Self {
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
