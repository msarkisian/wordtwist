use std::{net::SocketAddr, time::Duration};

use axum::extract::ws::{Message, WebSocket};
use serde::Serialize;
use serde_json;
use tokio::time;
use wordtwist::game::GameResults;

use crate::game::Game;

#[derive(Serialize)]
#[serde(tag = "type")]
enum SocketResponse<'a> {
    GuessResponse { word: &'a str, valid: bool },
    GameOver { results: GameResults },
}

pub async fn handle_socket_game(mut socket: WebSocket, _: SocketAddr, game: Game) {
    // TODO let client pass us their gametime
    const GAME_TIME: u64 = 120;
    // ignoring potential errors here, since if the client fails to establish the socket
    // there isn't anything we can do here anyway
    let _ = socket
        .send(axum::extract::ws::Message::Text(
            serde_json::to_string(&game).unwrap(),
        ))
        .await
        .is_ok();

    tokio::spawn(async move {
        let mut submitted_words = Vec::with_capacity(game.data.valid_words().len());

        let timeout = time::sleep(Duration::from_secs(GAME_TIME));
        tokio::pin!(timeout);

        while !timeout.is_elapsed() {
            tokio::select! {
                _ = &mut timeout => {
                    let _ = socket.send(Message::Text(serde_json::to_string(&SocketResponse::GameOver { results: game.data.score(submitted_words) }).unwrap())).await.is_err();
                    break;
                }
                s = socket.recv() => {
                    if let Some(Ok(Message::Text(word))) = s {
                        if game.data.valid_words().contains(&word) && !submitted_words.contains(&word) {
                            submitted_words.push(word);
                            let _ = socket.send(Message::Text(
                                serde_json::to_string(&SocketResponse::GuessResponse {
                                     word: submitted_words.last().unwrap(), valid: true
                                    }).unwrap()
                            )).await;
                        } else {
                            let _ = socket.send(Message::Text(
                                serde_json::to_string(&SocketResponse::GuessResponse {
                                word: &word, valid: false
                                }).unwrap()
                            )).await;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    });
}
