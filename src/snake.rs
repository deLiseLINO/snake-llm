use rand::Rng;

use std::collections::LinkedList;

use crate::models::{Direction, Point};

#[derive(Clone)]
pub struct Snake {
    list: LinkedList<Point>,
    direction: Direction,
    length: usize,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
            direction: match rand::thread_rng().gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            },
            length: 2,
        }
    }

    pub fn set_head(&mut self, point: Point) {
        self.list.push_front(point);
    }

    pub fn moving(&mut self) {
        let front = self.list.front().unwrap();
        let mut new_x = front.x;
        let mut new_y = front.y;
        match self.direction {
            Direction::Up => new_y += 1,
            Direction::Down => new_y -= 1,
            Direction::Left => new_x -= 1,
            Direction::Right => new_x += 1,
        };
        let new_point = Point { x: new_x, y: new_y };

        self.list.push_front(new_point);

        if self.list.len() > self.length {
            self.list.pop_back();
        }
    }

    pub fn grow(&mut self) {
        self.length += 1;
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if direction != self.direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn get_list(&self) -> LinkedList<Point> {
        self.list.clone()
    }

    pub fn get_head(&self) -> Point {
        self.list.front().unwrap().clone()
    }

    pub fn reset(&mut self) {
        self.list.clear();
        self.length = 2;
    }
}
