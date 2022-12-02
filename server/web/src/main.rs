mod game;
pub mod user;

use axum::{routing::get, Router};
use game::get_new_game;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/game", get(get_new_game));

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
