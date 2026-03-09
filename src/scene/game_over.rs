use crate::constants::colors::*;
use crate::scene::{Scene, SceneTransition};
use macroquad::prelude::*;

#[derive(Debug, PartialEq)]
pub struct GameOverScene {
    score: u64,
}

impl GameOverScene {
    pub fn new(score: u64) -> Self {
        GameOverScene { score }
    }
}

impl Scene for GameOverScene {
    fn draw(&self) {
        clear_background(GRAY_900);

        let text = "Game Over";
        let font_size: f32 = 64.0;
        let dims = measure_text(text, None, font_size as u16, 1.0);
        let x = screen_width() / 2.0 - dims.width / 2.0;
        let y = screen_height() / 2.0 + dims.height / 2.0 - 30.0;
        draw_text(text, x, y, font_size, WHITE);

        let text = format!("Score: {}, press escape to go to the menu", &self.score);
        let font_size: f32 = 24.0;
        let dims = measure_text(text.as_str(), None, font_size as u16, 1.0);
        let x = screen_width() / 2.0 - dims.width / 2.0;
        let y = screen_height() / 2.0 + dims.height / 2.0 + 30.0;
        draw_text(text.as_str(), x, y, font_size, WHITE);
    }

    fn update(&mut self) -> SceneTransition {
        if is_key_pressed(KeyCode::Escape) {
            return SceneTransition::GoToMenu;
        }
        SceneTransition::None
    }
}
