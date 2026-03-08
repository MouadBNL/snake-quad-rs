use macroquad::prelude::*;

use crate::grid::Grid;
mod grid;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn window_config() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    const BG: Color = color_u8!(2, 6, 24, 255);
    const BGM: Color = color_u8!(16, 24, 40, 255);
    let grid = Grid::new(0.0, 48.0, 32, 10);
    loop {
        clear_background(BG);

        draw_rectangle(0.0, 0.0, screen_width(), 48.0, BGM);
        grid.draw();


        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
