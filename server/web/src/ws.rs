use std::{net::SocketAddr, time::Duration};

use axum::extract::ws::{Message, WebSocket};
use tokio::sync::oneshot;

use crate::game::Game;

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

    let mut timeout = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(GAME_TIME)).await;
    });
    let (tx_done, mut rx_done) = oneshot::channel::<()>();

    let mut process_words = tokio::spawn(async move {
        let mut submitted_words = Vec::with_capacity(game.data.valid_words().len());
        loop {
            tokio::select! {
                _ = &mut rx_done => {
                    println!("timeout");
                    let _ = socket.send(Message::Text("!GAME OVER".to_string())).await.is_err();
                    break;
                }
                Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(word) = msg {
                    if game.data.valid_words().contains(&word) && !submitted_words.contains(&word) {
                        submitted_words.push(word);
                        if socket
                            .send(Message::Text("true".to_string()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    } else {
                        if socket
                            .send(Message::Text("false".to_string()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                }

                }
            }
        }
    });

    tokio::select! {
        _ = (&mut timeout) => {
            // stop the process word task, signal the client
            tx_done.send(()).unwrap();
        }
        _ = (&mut process_words) => {
        // if client hangs up, cancel timeout
        timeout.abort();
        }
    };
}
