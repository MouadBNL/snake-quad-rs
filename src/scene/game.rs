use crate::constants::colors::*;
use crate::{
    fruit::Fruit,
    scene::{Scene, SceneTransition},
    snake::{self, Snake},
};
use macroquad::prelude::*;


const CELL_SIZE: f32 = 32.0;
const CELL_PADDING: f32 = 1.0;
const HEADER_HEIGHT: f32 = 48.0;
const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 16;

#[derive(Debug, PartialEq)]
pub struct GameScene {
    snake: Snake,
    fruit: Fruit,
    score: u64,

    game_speed: u16,
    game_cnt: u16,
}

enum GridCellType {
    Emty,
    Snake,
    Fruit,
}

impl GameScene {
    pub fn new() -> Self {
        GameScene {
            snake: Snake::new(),
            fruit: Fruit::new(),
            score: 0,
            game_speed: 10,
            game_cnt: 0,
        }
    }

    fn draw_header(&self) {
        draw_rectangle(0.0, 0.0, screen_width(), HEADER_HEIGHT, GRAY_950);
        draw_text("Snake Game", 8.0, 40.0, 28.0, GREEN_600);
        draw_text(
            format!("Score: {}", self.score).as_str(),
            screen_width() - 108.0,
            40.0,
            28.0,
            GREEN_600,
        );
        draw_line(0.0, HEADER_HEIGHT, screen_width(), HEADER_HEIGHT, 1.0, GRAY_700);
    }

    fn draw_grid(&self) {
        draw_rectangle(
            0.0,
            HEADER_HEIGHT,
            (GRID_WIDTH as f32) * CELL_SIZE,
            (GRID_HEIGHT as f32) * CELL_SIZE,
            GRAY_700,
        );

        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                let cell_type = if self.snake.cells.contains(&(i, j)) {
                    GridCellType::Snake
                } else if self.fruit.cell() == (i, j) {
                    GridCellType::Fruit
                } else {
                    GridCellType::Emty
                };

                draw_rectangle(
                    (i as f32) * CELL_SIZE + CELL_PADDING,
                    (j as f32) * CELL_SIZE + CELL_PADDING + HEADER_HEIGHT,
                    CELL_SIZE - CELL_PADDING * 2.0,
                    CELL_SIZE - CELL_PADDING * 2.0,
                    match cell_type {
                        GridCellType::Emty => GRAY_900,
                        GridCellType::Snake => GREEN_600,
                        GridCellType::Fruit => AMBER_700,
                    },
                );
            }
        }
    }
}

impl Scene for GameScene {
    fn draw(&self) {
        clear_background(GRAY_950);
        self.draw_header();
        self.draw_grid();
    }

    fn update(&mut self) -> SceneTransition {
        if is_key_pressed(KeyCode::Escape) {
            return SceneTransition::GoToGameOver { score: 0 };
        }

        if is_key_pressed(KeyCode::Down) {
            self.snake.change_direction(snake::SnakeDirection::Down);
        }

        if is_key_pressed(KeyCode::Up) {
            self.snake.change_direction(snake::SnakeDirection::Up);
        }

        if is_key_pressed(KeyCode::Right) {
            self.snake.change_direction(snake::SnakeDirection::Right);
        }

        if is_key_pressed(KeyCode::Left) {
            self.snake.change_direction(snake::SnakeDirection::Left);
        }

        
        if self.game_cnt == self.game_speed {
            self.game_cnt= 0;
            self.snake.update();
            if let Some(cell) = self.snake.get_head() && cell == self.fruit.cell() {
                self.score += 1;
                self.snake.has_eaten = true;
                self.fruit.respawn();
            }


            if self.snake.is_colliding_with_self() {
                return SceneTransition::GoToGameOver { score: self.score };
            }
        } else {
            self.game_cnt += 1;
        }

        SceneTransition::None
    }
}
