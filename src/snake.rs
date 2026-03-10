use std::collections::{HashSet, VecDeque};

use crate::scene::game::GRID_HEIGHT;
use crate::scene::game::GRID_WIDTH;

#[derive(Debug, PartialEq)]
pub enum SnakeDirection {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
pub struct Snake {
    direction: SnakeDirection,
    has_eaten: bool,
    cells: VecDeque<(usize, usize)>,
}

impl Default for Snake {
    fn default() -> Self {
        Snake::new()
    }
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            direction: SnakeDirection::Right,
            has_eaten: false,
            cells: VecDeque::from([(1, 1), (2, 1), (3, 1)]),
        }
    }

    pub fn eat(&mut self) {
        self.has_eaten = true;
    }

    pub fn update(&mut self) {
        if self.has_eaten {
            self.has_eaten = false;
        } else {
            self.cells.pop_front();
        }
        let head = self.head();

        let (dx, dy) = match self.direction {
            SnakeDirection::Left => (-1, 0),
            SnakeDirection::Right => (1, 0),
            SnakeDirection::Up => (0, -1),
            SnakeDirection::Down => (0, 1),
        };

        let x = (head.0 as isize + dx).rem_euclid(GRID_WIDTH as isize) as usize;
        let y = (head.1 as isize + dy).rem_euclid(GRID_HEIGHT as isize) as usize;

        self.cells.push_back((x, y));

        println!("dx {dx}, dy {dy}");
        println!("snake: {:?}", self.cells);
    }

    pub fn change_direction(&mut self, dir: SnakeDirection) {
        match (&self.direction, &dir) {
            (SnakeDirection::Left, SnakeDirection::Right)
            | (SnakeDirection::Right, SnakeDirection::Left)
            | (SnakeDirection::Down, SnakeDirection::Up)
            | (SnakeDirection::Up, SnakeDirection::Down) => {}
            _ => self.direction = dir,
        };
    }

    pub fn head(&self) -> (usize, usize) {
        *self.cells.iter().last().unwrap()
    }

    pub fn occupies(&self, cell: (usize, usize)) -> bool {
        self.cells.contains(&cell)
    }

    pub fn is_colliding_with_self(&self) -> bool {
        let mut uniq = HashSet::new();
        !self.cells.iter().all(|x| uniq.insert(x))
    }
}
