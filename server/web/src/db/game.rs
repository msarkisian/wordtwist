use chrono::Utc;
use rusqlite::{Connection, Result};
use uuid::Uuid;
use wordtwist::game::Game as GameData;

use crate::game::Game;

use super::user::UserID;

pub fn get_game_by_id(conn: &mut Connection, id: Uuid) -> Result<GameData> {
    conn.query_row_and_then(
        "SELECT game_data from GAMES where id=?1",
        (id.to_string(),),
        |row| Ok(serde_json::from_str(&row.get::<usize, String>(0)?).unwrap()),
    )
}

pub fn insert_game(conn: &mut Connection, game: &GameData) -> Result<Uuid> {
    let uuid = Uuid::new_v4();

    conn.execute(
        "INSERT INTO games (id, game_data) VALUES (?1, ?2)",
        (uuid.to_string(), serde_json::to_string(game).unwrap()),
    )?;
    Ok(uuid)
}

pub fn try_get_daily(conn: &mut Connection) -> Result<Game> {
    let date = Utc::now().date_naive().to_string();
    conn.query_row(
        "SELECT games.id, games.game_data FROM dates JOIN daily ON dates.daily_id = daily.id JOIN games ON daily.game_id = games.id WHERE date = ?1",
        (date,),
        |r| {
            Ok(Game {
                id: r.get(0)?,
                data: serde_json::from_str(&r.get::<usize, String>(1)?).unwrap()
            }
            )})
}

pub fn set_daily(conn: &mut Connection, id: Uuid) -> Result<()> {
    let date = Utc::now().date_naive().to_string();
    let daily_id: usize = conn.query_row(
        "INSERT INTO daily (game_id) VALUES (?1) RETURNING daily.id",
        (id.to_string(),),
        |r| r.get(0),
    )?;
    conn.execute(
        "INSERT INTO dates (date, daily_id) VALUES (?1, ?2)",
        (date, daily_id),
    )
    .expect("error adding to dates table (set_daily)");
    Ok(())
}

pub fn add_game_score(
    conn: &mut Connection,
    game_id: Uuid,
    user_id: UserID,
    score: usize,
    size: usize,
    time: usize,
) -> Result<()> {
    conn.execute(
        "INSERT INTO scores (game_id, user_id, score, size, time) VALUES (?1, ?2, ?3, ?4, ?5)",
        (game_id.to_string(), user_id.0, score, size, time),
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
    use crate::db::open_db_connection;

    use super::*;

    #[test]
    fn insert_and_get() {
        let mut connection = open_db_connection();
        let game = GameData::new(5);

        let game_id = insert_game(&mut connection, &game).unwrap();
        let fetched_game = get_game_by_id(&mut connection, game_id).unwrap();

        assert_eq!(game, fetched_game);
    }

    #[test]
    fn test_set_get_score() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE scores (
                id INTEGER PRIMARY KEY,
                game_id TEXT,
                user_id INTEGER,
                score, INTEGER,
                size INTEGER,
                time INTEGER
            );",
            (),
        )
        .unwrap();

        let uid = UserID(500);
        let game_uuid = Uuid::new_v4();

        add_game_score(&mut conn, game_uuid, uid, 9001, 5, 180).unwrap();
        assert_eq!(get_game_score(&mut conn, game_uuid, uid).unwrap(), 9001)
    }
}
