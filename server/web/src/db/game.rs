use rusqlite::{Connection, Result};
use uuid::Uuid;
use wordtwist::game::GeneratedGame;

pub fn insert_game(conn: &mut Connection, game: &GeneratedGame) -> Result<Uuid> {
    let uuid = Uuid::new_v4();

    conn.execute(
        "INSERT INTO games (id, game_data) VALUES (?1, ?2)",
        (uuid.to_string(), serde_json::to_string(game).unwrap()),
    )?;
    Ok(uuid)
}
