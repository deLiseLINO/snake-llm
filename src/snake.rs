use rand::Rng;

use crate::direction::Direction;
// use crate::game::Snake as SnakeT;
use crate::point::Point;
use std::collections::LinkedList;

pub struct Snake {
    list: LinkedList<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        Self {
            list: LinkedList::from([Point {
                x: start_x,
                y: start_y,
            }]),
            direction: match rand::thread_rng().gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            },
        }
    }

    pub fn moving(&mut self, growing: bool) {
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

        if growing {
            return;
        }

        if self.list.len() > 20 {
            self.list.pop_back();
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn get_list(&self) -> LinkedList<Point> {
        self.list.clone()
    }

    pub fn get_head(&self) -> Point {
        self.list.front().unwrap().clone()
    }
}
