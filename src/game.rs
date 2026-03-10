use macroquad::window::next_frame;

use crate::scene::{
    Scene, SceneTransition, game::GameScene, game_over::GameOverScene, menu::MenuScene,
};

pub const GAME_WINDOW_WIDTH: i32 = 1280;
pub const GAME_WINDOW_HEIGHT: i32 = 720;

pub enum GameState {
    Menu(MenuScene),
    Game(GameScene),
    Over(GameOverScene),
}

impl GameState {
    fn scene(&mut self) -> &mut dyn Scene {
        match self {
            GameState::Menu(s) => s,
            GameState::Game(s) => s,
            GameState::Over(s) => s,
        }
    }
}

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::Menu(MenuScene::new()),
        }
    }

    pub async fn gameloop(&mut self) {
        loop {
            let transition = self.state.scene().update();
            self.state.scene().draw();

            match transition {
                SceneTransition::None => {}
                SceneTransition::GoToMenu => self.state = GameState::Menu(MenuScene::new()),
                SceneTransition::GoToGame => self.state = GameState::Game(GameScene::new()),
                SceneTransition::GoToGameOver { score } => {
                    self.state = GameState::Over(GameOverScene::new(score))
                }
            };
            next_frame().await
        }
    }
}
