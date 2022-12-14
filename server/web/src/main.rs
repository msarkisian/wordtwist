mod db;
mod game;

use axum::{routing::get, Router};
use axum_extra::routing::SpaRouter;
use db::open_db_connection;
use game::{get_existing_game_by_id, get_new_game};

#[tokio::main]
async fn main() {
    let _ = open_db_connection();

    let app = Router::new()
        .merge(SpaRouter::new("/assets", "../client/dist/assets").index_file("../index.html"))
        .route("/game/:size", get(get_new_game))
        .route("/game/id/:id", get(get_existing_game_by_id));

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
