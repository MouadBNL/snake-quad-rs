use crate::fruit::Fruit;

pub enum SnakeDirection {
    Left,
    Right,
    Down,
    Up,
}

pub struct Snake {
    pub direction: SnakeDirection,
    pub has_eaten: bool,
    pub cells: Vec<(usize, usize)>,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            direction: SnakeDirection::Right,
            has_eaten: false,
            cells: vec![(1, 1), (2, 1), (3, 1)],
        }
    }

    pub fn update(&mut self) {
        if self.has_eaten {
            self.has_eaten = false;
        } else {
            self.cells.remove(0);
        }
        let corr = self.cells.last().unwrap();

        let x = match self.direction {
            SnakeDirection::Left => (corr.0 + 32 - 1) % 32,
            SnakeDirection::Right => (corr.0 + 1) % 32,
            SnakeDirection::Up => corr.0,
            SnakeDirection::Down => corr.0,
        };
        let y = match self.direction {
            SnakeDirection::Down => (corr.1 + 1) % 16,
            SnakeDirection::Up => (corr.1 + 16 - 1) % 16,
            SnakeDirection::Right => corr.1,
            SnakeDirection::Left => corr.1,
        };
        self.cells.push((x, y));
    }

    pub fn change_direction(&mut self, dir: SnakeDirection) {
        match (&self.direction, &dir) {
            (SnakeDirection::Left, SnakeDirection::Right)
            | (SnakeDirection::Right, SnakeDirection::Left)
            | (SnakeDirection::Down, SnakeDirection::Up)
            | (SnakeDirection::Up, SnakeDirection::Down) => {
                eprintln!("Movement forbiden");
            }
            _ => self.direction = dir,
        };
    }

    pub fn get_head(&self) -> Option<(usize, usize)> {
        self.cells.last().cloned()
    }
}
