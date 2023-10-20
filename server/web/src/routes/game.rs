use std::net::SocketAddr;

use crate::{
    db::{
        game::{get_game_by_id, get_game_score, get_game_stats},
        open_db_connection,
    },
    game::{DailyGame, Game},
    ws::handle_socket_game,
};

use axum::{
    extract::{ConnectInfo, Path, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::SignedCookieJar;
use serde::Deserialize;
use uuid::Uuid;

use super::user::get_uid_from_cookie;

#[derive(Deserialize)]
struct GetGameStatsDTO {
    game_id: String,
    max_time: usize,
}

// TODO parse out user and pass to `handle_socket_game`
pub async fn get_new_game(
    Path(size): Path<usize>,
    jar: SignedCookieJar,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    if !(3..=7).contains(&size) {
        return Err::<(), _>((
            StatusCode::BAD_REQUEST,
            "Invalid game size. Games can be of size 3-7 inclusive.",
        ))
        .into_response();
    }
    let user = get_uid_from_cookie(jar);
    ws.on_upgrade(move |socket| handle_socket_game(socket, addr, Game::new(size), user))
        .into_response()
}

pub async fn get_existing_game_by_id(
    Path(id): Path<String>,
    jar: SignedCookieJar,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let conn = &mut open_db_connection();
    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err::<(), _>((StatusCode::BAD_REQUEST, "Cannot parse provided id"))
                .into_response()
        }
    };
    let game_data = match get_game_by_id(conn, id) {
        Ok(game) => game,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err::<(), _>((StatusCode::NOT_FOUND, "Game with specified ID not found"))
                .into_response()
        }
        Err(_) => {
            return Err::<(), _>((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching game from database",
            ))
            .into_response()
        }
    };
    let user = get_uid_from_cookie(jar);
    ws.on_upgrade(move |socket| handle_socket_game(socket, addr, Game::from(id, game_data), user))
        .into_response()
}

pub async fn get_daily_game(
    jar: SignedCookieJar,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user = get_uid_from_cookie(jar);
    let game = DailyGame::get().0;
    ws.on_upgrade(move |socket| handle_socket_game(socket, addr, game, user))
}

pub async fn get_score(jar: SignedCookieJar, Path(game_id): Path<String>) -> impl IntoResponse {
    let Some(uid) = get_uid_from_cookie(jar) else {
        return Err((StatusCode::UNAUTHORIZED, "You are not currently logged in"));
    };
    let Ok(game_id) = Uuid::parse_str(&game_id) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid game id provided"));
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

pub async fn get_stats(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let Ok(payload) = serde_json::from_value::<GetGameStatsDTO>(payload) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid body for getAverageScore"));
    };
    let Ok(game_id) = Uuid::parse_str(&payload.game_id) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid game id provided"));
    };
    let mut conn = open_db_connection();
    let res = match get_game_stats(&mut conn, game_id, payload.max_time) {
        Ok(res) => res,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err((
                StatusCode::NOT_FOUND,
                "No results found for provided game under provided time",
            ))
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting game stats from database",
            ));
        }
    };
    Ok((StatusCode::OK, Json(res)))
}
