use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use wordtwist::game::GeneratedGame;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

pub async fn get_new_game(Path(size): Path<usize>) -> impl IntoResponse {
    if !(3..=8).contains(&size) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid game size. Games can be of size 3-8 inclusive.",
        ));
    }
    Ok((StatusCode::OK, Json(Game::new(size))))
}
