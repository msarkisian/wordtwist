use crate::{
    db::{
        game::{add_game_score, get_game_by_id, get_game_score},
        open_db_connection,
    },
    game::{DailyGame, Game},
};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde::Deserialize;
use uuid::Uuid;

use super::user::get_uid_from_cookie;

#[derive(Deserialize)]
struct PostScoreDTO {
    score: usize,
    time: usize,
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

pub async fn get_daily_game() -> impl IntoResponse {
    (StatusCode::OK, Json(DailyGame::get().0))
}

pub async fn get_score(jar: SignedCookieJar, Path(game_id): Path<String>) -> impl IntoResponse {
    let Some(uid) = get_uid_from_cookie(jar) else {
        return Err((StatusCode::UNAUTHORIZED, "You are not currently logged in"));
    };
    let Ok(game_id) = Uuid::parse_str(&game_id) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid game id provided"))
    };
    let conn = &mut open_db_connection();
    let score = match get_game_score(conn, game_id, uid) {
        Ok(s) => s,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err((
                StatusCode::NOT_FOUND,
                "No score found for specified user and game",
            ))
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting score from database",
            ))
        }
    };
    Ok((StatusCode::OK, score.to_string()))
}

pub async fn post_score(
    jar: SignedCookieJar,
    Path(game_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let Ok(game_id) = Uuid::parse_str(&game_id) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid game id provided"))
    };
    let Ok(payload) = serde_json::from_value::<PostScoreDTO>(payload) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid body for PostScore"))
    };
    let Some(uid) = get_uid_from_cookie(jar) else {
        return Err((StatusCode::UNAUTHORIZED, "You are not currently logged in"));
    };

    let conn = &mut open_db_connection();
    let _ = match add_game_score(conn, game_id, uid, payload.score, payload.time) {
        Ok(_) => (),
        Err(rusqlite::Error::SqliteFailure(e, _))
            if e.code == rusqlite::ErrorCode::ConstraintViolation =>
        {
            return Err((
                StatusCode::CONFLICT,
                "Score for this user and game already exist in database",
            ))
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error adding score to database",
            ))
        }
    };
    Ok(StatusCode::CREATED)
}
