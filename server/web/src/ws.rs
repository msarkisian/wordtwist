use std::{borrow::Cow, net::SocketAddr, time::Duration};

use axum::extract::ws::{close_code::NORMAL, Message, WebSocket};
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
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum SocketResponse<'a> {
    GuessResponse { word: &'a str, valid: bool },
    GameOver { results: GameResults },
    Setup { time: u64, game: GameSetupDTO<'a> },
}

#[derive(Serialize)]
struct GameSetupDTO<'a> {
    grid: &'a Vec<Vec<char>>,
    id: &'a str,
}

pub async fn handle_socket_game(
    mut socket: WebSocket,
    _: SocketAddr,
    game: Game,
    time: u64,
    user: Option<UserID>,
) {
    // ignoring potential errors here, since if the client fails to establish the socket
    // there isn't anything we can do here anyway
    let _ = socket
        .send(axum::extract::ws::Message::Text(
            serde_json::to_string(&SocketResponse::Setup {
                game: GameSetupDTO {
                    grid: game.data.grid(),
                    id: &game.id,
                },
                time,
            })
            .unwrap(),
        ))
        .await
        .is_ok();

    tokio::spawn(async move {
        let mut submitted_words = Vec::with_capacity(game.data.valid_words().len());

        let timeout = time::sleep(Duration::from_secs(time));
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                _ = &mut timeout => {
                    handle_end_game(socket, game, user, time, submitted_words).await;
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
            // TODO handle already existing score for user for game somehow
            eprintln!("failed to add game {game_id:?} to database (for user {user:?}")
        }
    }
    let _ = socket
        .send(Message::Text(
            serde_json::to_string(&SocketResponse::GameOver { results }).unwrap(),
        ))
        .await
        .is_err();
    let _ = socket
        .send(Message::Close(Some({
            axum::extract::ws::CloseFrame {
                code: NORMAL,
                reason: Cow::from("game over"),
            }
        })))
        .await;
}
