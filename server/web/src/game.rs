use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use wordtwist::game::GeneratedGame;

use crate::db::open_db_connection;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Game {
    id: String,
    data: GeneratedGame,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let mut conn = open_db_connection();
        let game = GeneratedGame::new(size);

        let uuid = crate::db::game::insert_game(&mut conn, &game).unwrap();
        Self {
            id: uuid.to_string(),
            data: game,
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
