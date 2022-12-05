use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use wordtwist::game::GeneratedGame;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    data: GeneratedGame,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            data: GeneratedGame::new(size),
        }
    }
}

pub async fn get_new_game() -> impl IntoResponse {
    Json(Game::new(5))
}
