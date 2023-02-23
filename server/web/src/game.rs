use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wordtwist::game::Game as GameData;

use crate::db::{
    game::{get_game_by_id, set_daily, try_get_daily},
    open_db_connection,
};

pub mod daily;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Game {
    id: String,
    data: GameData,
}

pub struct DailyGame(GameData);

impl Game {
    pub fn new(size: usize) -> Self {
        let mut conn = open_db_connection();
        let data = GameData::new(size);

        let uuid = crate::db::game::insert_game(&mut conn, &data).unwrap();
        Self {
            id: uuid.to_string(),
            data,
        }
    }

    pub fn from(uuid: Uuid, data: GameData) -> Self {
        Self {
            id: uuid.to_string(),
            data,
        }
    }
}

impl DailyGame {
    pub fn get() -> Self {
        let mut conn = open_db_connection();
        Self(try_get_daily(&mut conn).unwrap_or_else(|_| {
            let game = Game::new(4);
            set_daily(&mut conn, Uuid::parse_str(game.id.as_str()).unwrap())
                .expect("error adding daily game to db (DailyGame::get)");
            game.data
        }))
    }
}

pub async fn get_new_game(Path(size): Path<usize>) -> impl IntoResponse {
    if !(3..=7).contains(&size) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid game size. Games can be of size 3-7 inclusive.",
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_daily_game() {
        let g1 = DailyGame::get();
        let g2 = DailyGame::get();

        assert_eq!(g1.0, g2.0);
    }
}
