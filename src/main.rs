use std::hash::{BuildHasher, Hasher};

use macroquad::{prelude::*, rand::srand};
use game::Game;

mod constants;
mod snake;
mod fruit;
mod scene;
mod game;

fn window_config() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: game::GAME_WINDOW_WIDTH,
        window_height: game::GAME_WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn rand_seeder() -> u64 {
    std::collections::hash_map::RandomState::new().build_hasher().finish()
}

#[macroquad::main(window_config)]
async fn main() {
    srand(rand_seeder());
    let mut game = Game::new();
    game.gameloop().await;
}
