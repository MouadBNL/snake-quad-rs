use macroquad::prelude::*;

const CELL_SIZE: f32 = 32.0;
const CELL_PADDING: f32 = 2.0;

#[derive(Debug)]
pub struct Grid {
    x: f32,
    y: f32,
    w: usize,
    h: usize,
}

impl Grid {
    pub fn new(x: f32, y: f32, w: usize, h: usize) -> Self {
        Grid { x, y, w, h }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            self.y,
            (self.w as f32) * CELL_SIZE,
            (self.h as f32) * CELL_SIZE,
            DARKGRAY,
        );

        for i in 0..self.w {
            for j in 0..self.h {
                draw_rectangle(
                    (i as f32) * CELL_SIZE + CELL_PADDING,
                    (j as f32) * CELL_SIZE + CELL_PADDING + self.y,
                    CELL_SIZE - CELL_PADDING,
                    CELL_SIZE - CELL_PADDING,
                    BLACK,
                );
            }
        }
    }
}
