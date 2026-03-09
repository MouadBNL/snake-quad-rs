use crate::scene::{Scene, SceneTransition};
use macroquad::prelude::*;
use crate::constants::colors::*;

#[derive(Debug, PartialEq)]
pub struct MenuScene {
    best_score: u64,
}

impl MenuScene {
    pub fn new() -> Self {
        MenuScene { best_score: 0 }
    }
}

impl Scene for MenuScene {
    fn draw(&self) {
        clear_background(GRAY_800);

        let text = "Press Space to start";
        let font_size: f32 = 64.0;
        let dims = measure_text(text, None, font_size as u16, 1.0);
        let x = screen_width() / 2.0 - dims.width / 2.0;
        let y = screen_height() / 2.0 + dims.height / 2.0; 
        draw_text(text, x, y, font_size, WHITE);
    }

    fn update(&mut self) -> SceneTransition {
        if is_key_pressed(KeyCode::Space) {
            return SceneTransition::GoToGame;
        }
        SceneTransition::None
    }
}
