pub mod menu;
pub mod game;
pub mod game_over;

pub trait Scene {
    fn draw(&self);
    fn update(&mut self) -> SceneTransition;
}

pub enum SceneTransition {
    None,
    GoToMenu,
    GoToGame,
    GoToGameOver { score: u64 },
}
