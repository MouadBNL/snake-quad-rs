use macroquad::rand::rand;

#[derive(Debug, PartialEq)]
pub struct Fruit {
    x: usize,
    y: usize,
}

impl Fruit {
    pub fn new() -> Self {
        Fruit {
            x: (rand() % 32) as usize,
            y: (rand() % 16) as usize,
        }
    }
    pub fn cell(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn respawn(&mut self) {
        self.x = (rand() % 32) as usize;
        self.y = (rand() % 16) as usize;
    }
}
