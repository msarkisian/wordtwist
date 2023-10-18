use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::IntoResponse,
    Json,
};

use crate::game::Game;

pub async fn handle_socket_game(mut socket: WebSocket, who: SocketAddr, game: Game) {
    // TODO let client pass us their gametime
    const GAME_TIME: usize = 120;
    if socket
        .send(axum::extract::ws::Message::Text(
            serde_json::to_string(&game).unwrap(),
        ))
        .await
        .is_ok()
    {
        println!("socket opened at {who:?}");
        println!("{:?}", socket);
    }
}
