pub enum SnakeDirection {
    Left,
    Right,
    Down,
    Up,
}

pub struct Snake {
    pub direction: SnakeDirection,
    pub cells: Vec<(usize, usize)>,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            direction: SnakeDirection::Right,
            cells: vec![(1, 1), (2, 1), (3, 1)],
        }
    }

    pub fn update(&mut self) {
        self.cells.remove(0);
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
        self.direction = dir;
    }
}
