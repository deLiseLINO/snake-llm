use rand::Rng;

use crate::direction::Direction;
use crate::point::Point;
use std::collections::LinkedList;

pub struct Snake {
    list: LinkedList<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new(start_x: u16, start_y: u16) -> Self {
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

    pub fn moving(&mut self) {
        let front = self.list.front().unwrap();
        let mut new_x = front.x;
        let mut new_y = front.y;
        match self.direction {
            Direction::Up => new_y -= 1,
            Direction::Down => new_y += 1,
            Direction::Left => new_x -= 1,
            Direction::Right => new_x += 1,
        };
        let new_point = Point { x: new_x, y: new_y };

        self.list.push_front(new_point);

        if self.list.len() > 5 {
            self.list.pop_back();
        }
    }

    pub fn get_list(&self) -> LinkedList<Point> {
        self.list.clone()
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    // pub fn print_list(&self) {
    //     for point in self.list.clone().into_iter() {
    //         println!("{}, {}", point.x, point.y)
    //     }
    // }
}
