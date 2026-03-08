use macroquad::prelude::*;
use crate::{constants::colors::*, snake::Snake};

const CELL_SIZE: f32 = 32.0;
const CELL_PADDING: f32 = 1.0;

#[derive(Debug)]
pub struct Grid {
    x: f32,
    y: f32,
    w: usize,
    h: usize,
}


enum GridCellType {
    Emty,
    Snake,
}

impl Grid {
    pub fn new(x: f32, y: f32, w: usize, h: usize) -> Self {
        Grid { x, y, w, h }
    }
    pub fn draw(&self, snake: &Snake) {
        draw_rectangle(
            self.x,
            self.y,
            (self.w as f32) * CELL_SIZE,
            (self.h as f32) * CELL_SIZE,
            GRAY_700,
        );

        for i in 0..self.w {
            for j in 0..self.h {
                let cell_type = if snake.cells.contains(&(i, j)) {
                    GridCellType::Snake
                } else {
                    GridCellType::Emty
                };

                draw_rectangle(
                    (i as f32) * CELL_SIZE + CELL_PADDING,
                    (j as f32) * CELL_SIZE + CELL_PADDING + self.y,
                    CELL_SIZE - CELL_PADDING * 2.0,
                    CELL_SIZE - CELL_PADDING * 2.0,
                    match cell_type {
                        GridCellType::Emty => GRAY_900,
                        GridCellType::Snake => GREEN_600,
                    },
                );
            }
        }
    }
}
