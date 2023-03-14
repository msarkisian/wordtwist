mod db;
mod game;
mod routes;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use axum_extra::{extract::cookie::Key, routing::SpaRouter};
use db::open_db_connection;
use routes::{
    game::{get_daily_game, get_existing_game_by_id, get_new_game},
    user::{create_new_user, login_user},
};

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
        key: Key::generate(),
    };

    let app = Router::new()
        .merge(SpaRouter::new("/assets", "../client/dist/assets").index_file("../index.html"))
        .route("/game/:size", get(get_new_game))
        .route("/game/id/:id", get(get_existing_game_by_id))
        .route("/game/daily", get(get_daily_game))
        .route("/user", post(create_new_user))
        .route("/login", post(login_user))
        .with_state(state);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
