mod db;
mod game;
mod routes;
mod ws;

use std::net::SocketAddr;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use db::open_db_connection;
use routes::{
    game::{get_daily_game, get_existing_game_by_id, get_new_game, get_score, get_stats},
    user::{create_new_user, get_login, login_user, logout_user},
};
use tower_http::services::{ServeDir, ServeFile};

const KEY_BYTES: &[u8] = include_bytes!("../cookie_key");

#[derive(Clone)]
struct AppState {
    key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() {
    let _ = open_db_connection();

    let state = AppState {
        key: Key::from(KEY_BYTES),
    };

    let app = Router::new()
        .nest_service("/", ServeFile::new("../client/dist/index.html"))
        .nest_service("/assets", ServeDir::new("../client/dist/assets"))
        .route("/game/:size", get(get_new_game))
        .route("/game/id/:id", get(get_existing_game_by_id))
        .route("/game/daily", get(get_daily_game))
        .route("/game/score/:id", get(get_score))
        .route("/game/stats", get(get_stats))
        .route("/user", post(create_new_user))
        .route(
            "/login",
            post(login_user).get(get_login).delete(logout_user),
        )
        .with_state(state);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}
