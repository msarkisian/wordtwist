use rusqlite::{Connection, Result};
use uuid::Uuid;

use super::user::UserID;

pub fn add_game_score(
    conn: &mut Connection,
    game_id: Uuid,
    user_id: UserID,
    score: usize,
) -> Result<()> {
    conn.execute(
        "INSERT INTO scores (game_id, user_id, score) VALUES (?1, ?2, ?3)",
        (game_id.to_string(), user_id.0, score),
    )?;
    Ok(())
}

pub fn get_game_score(conn: &mut Connection, game_id: Uuid, user_id: UserID) -> Result<usize> {
    conn.query_row(
        "SELECT score FROM scores WHERE game_id=?1 AND user_id=?2",
        (game_id.to_string(), user_id.0),
        |r| r.get(0),
    )
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;
    use uuid::Uuid;

    use crate::db::{score::get_game_score, user::UserID};

    use super::add_game_score;

    #[test]
    fn test_set_get_score() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE scores (
                id INTEGER PRIMARY KEY,
                game_id TEXT,
                user_id INTEGER,
                score, INTEGER
            );",
            (),
        )
        .unwrap();

        let uid = UserID(500);
        let game_uuid = Uuid::new_v4();

        add_game_score(&mut conn, game_uuid, uid, 9001).unwrap();
        assert_eq!(get_game_score(&mut conn, game_uuid, uid).unwrap(), 9001)
    }
}
