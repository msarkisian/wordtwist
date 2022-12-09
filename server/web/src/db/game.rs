use rusqlite::{Connection, Result};
use uuid::Uuid;
use wordtwist::game::GeneratedGame;

pub fn get_game_by_id(conn: &mut Connection, id: Uuid) -> Result<GeneratedGame> {
    conn.query_row_and_then(
        "SELECT game_data from GAMES where id=?1",
        (id.to_string(),),
        |row| Ok(serde_json::from_str(&row.get::<usize, String>(0)?).unwrap()),
    )
}

pub fn insert_game(conn: &mut Connection, game: &GeneratedGame) -> Result<Uuid> {
    let uuid = Uuid::new_v4();

    conn.execute(
        "INSERT INTO games (id, game_data) VALUES (?1, ?2)",
        (uuid.to_string(), serde_json::to_string(game).unwrap()),
    )?;
    Ok(uuid)
}

#[cfg(test)]
mod test {
    use crate::db::open_db_connection;

    use super::*;

    #[test]
    fn insert_and_get() {
        let mut connection = open_db_connection();
        let game = GeneratedGame::new(5);

        let game_id = insert_game(&mut connection, &game).unwrap();
        let fetched_game = get_game_by_id(&mut connection, game_id).unwrap();

        assert_eq!(game, fetched_game);
    }
}
