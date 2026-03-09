use macroquad::prelude::*;
use constants::colors::*;

use crate::{grid::Grid, snake::Snake};
mod constants;
mod grid;
mod snake;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn window_config() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let game_speed = 20;
    let mut cnt = 0;
    let grid = Grid::new(0.0, 48.0, 32, 16);
    let mut snake = Snake::new();
    loop {
        if is_key_pressed(KeyCode::Down) {
            snake.change_direction(snake::SnakeDirection::Down);
        }

        if is_key_pressed(KeyCode::Up) {
            snake.change_direction(snake::SnakeDirection::Up);
        }

        if is_key_pressed(KeyCode::Right) {
            snake.change_direction(snake::SnakeDirection::Right);
        }

        if is_key_pressed(KeyCode::Left) {
            snake.change_direction(snake::SnakeDirection::Left);
        }

        if cnt == game_speed {
            cnt = 0;
            snake.update();
        } else {
            cnt+=1;
        }
        clear_background(GRAY_950);

        draw_rectangle(0.0, 0.0, screen_width(), 48.0, GRAY_950);
        draw_text("Snake Game", 8.0, 40.0, 28.0, GREEN_600);
        draw_line(0.0, 48.0, screen_width(), 48.0, 1.0, GRAY_700);
        grid.draw(&snake);

        next_frame().await
    }
}
