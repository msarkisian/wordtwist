use std::{net::SocketAddr, time::Duration};

use axum::extract::ws::{Message, WebSocket};
use serde::Serialize;
use serde_json;
use tokio::time;
use uuid::Uuid;
use wordtwist::game::GameResults;

use crate::{
    db::{game::add_game_score, open_db_connection, user::UserID},
    game::Game,
};

#[derive(Serialize)]
#[serde(tag = "type")]
enum SocketResponse<'a> {
    GuessResponse { word: &'a str, valid: bool },
    GameOver { results: GameResults },
}

pub async fn handle_socket_game(
    mut socket: WebSocket,
    _: SocketAddr,
    game: Game,
    user: Option<UserID>,
) {
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
                    handle_end_game(socket, game, user, GAME_TIME, submitted_words).await;
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

async fn handle_end_game(
    mut socket: WebSocket,
    game: Game,
    user: Option<UserID>,
    time: u64,
    submitted_words: Vec<String>,
) {
    let game_id = Uuid::parse_str(&game.id).unwrap();
    let results = game.data.score(submitted_words);
    if user.is_some() {
        let conn = &mut open_db_connection();
        if add_game_score(conn, game_id, user.unwrap(), results.score, time as usize).is_err() {
            eprintln!("failed to add game {game_id:?} to database (for user {user:?}")
        }
    }
    let _ = socket
        .send(Message::Text(
            serde_json::to_string(&SocketResponse::GameOver { results }).unwrap(),
        ))
        .await
        .is_err();
}
