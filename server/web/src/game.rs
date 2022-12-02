use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use wordtwist::game::GeneratedGame;

#[derive(Debug, Deserialize, Serialize)]
pub struct Game<const N: usize> {
    data: GeneratedGame<N>,
}

impl<const N: usize> Game<N> {
    pub fn new() -> Self {
        Self {
            data: GeneratedGame::<N>::new(),
        }
    }
}

pub async fn get_new_game() -> impl IntoResponse {
    Json(Game::<5>::new())
}
