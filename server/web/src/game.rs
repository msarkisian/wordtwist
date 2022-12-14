use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wordtwist::game::GeneratedGame;

use crate::db::{game::get_game_by_id, open_db_connection};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
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

    pub fn from(uuid: Uuid, data: GeneratedGame) -> Self {
        Self {
            id: uuid.to_string(),
            data,
        }
    }
}

pub async fn get_new_game(Path(size): Path<usize>) -> impl IntoResponse {
    if !(3..=7).contains(&size) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid game size. Games can be of size 3-8 inclusive.",
        ));
    }
    Ok((StatusCode::OK, Json(Game::new(size))))
}

pub async fn get_existing_game_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let conn = &mut open_db_connection();
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Cannot parse provided id")),
    };
    let game_data = match get_game_by_id(conn, id) {
        Ok(game) => game,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err((StatusCode::NOT_FOUND, "Game with specified ID not found"))
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching game from database",
            ))
        }
    };

    Ok((StatusCode::OK, Json(Game::from(id, game_data))))
}
