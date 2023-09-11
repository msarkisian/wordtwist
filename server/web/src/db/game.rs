use chrono::Utc;
use rusqlite::{Connection, Result};
use serde::Serialize;
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
        "INSERT INTO games (id, game_data, size) VALUES (?1, ?2, ?3)",
        (
            uuid.to_string(),
            serde_json::to_string(game).unwrap(),
            game.size(),
        ),
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
    time: usize,
) -> Result<()> {
    conn.execute(
        "INSERT INTO scores (game_id, user_id, score, time) VALUES (?1, ?2, ?3, ?4)",
        (game_id.to_string(), user_id.0, score, time),
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

#[derive(Serialize)]
pub struct GameStats {
    count: usize,
    max_score: usize,
    average_score: f64,
}

pub fn get_game_stats(conn: &mut Connection, game_id: Uuid, max_time: usize) -> Result<GameStats> {
    conn.query_row(
        "SELECT COUNT(score), MAX(score), AVG(score) FROM scores WHERE game_id=?1 AND time=?2",
        (game_id.to_string(), max_time),
        |r| {
            Ok(GameStats {
                count: r.get(0)?,
                max_score: r.get(1)?,
                average_score: r.get(2)?,
            })
        },
    )
}

#[cfg(test)]
mod test {
    use crate::db::open_db_connection;

    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
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
        conn
    }

    #[test]
    fn insert_and_get() {
        let mut connection = open_db_connection();
        let game = GameData::new(4);

        let game_id = insert_game(&mut connection, &game).unwrap();
        let fetched_game = get_game_by_id(&mut connection, game_id).unwrap();

        assert_eq!(game, fetched_game);
    }

    #[test]
    fn test_set_get_score() {
        let mut conn = setup_test_db();
        let uid = UserID(500);
        let game_uuid = Uuid::new_v4();

        add_game_score(&mut conn, game_uuid, uid, 9001, 5).unwrap();
        assert_eq!(get_game_score(&mut conn, game_uuid, uid).unwrap(), 9001)
    }

    #[test]
    fn score_stats() {
        let mut conn = setup_test_db();
        let game_uuid = Uuid::new_v4();

        add_game_score(&mut conn, game_uuid, UserID(2), 60, 30).unwrap();
        add_game_score(&mut conn, game_uuid, UserID(3), 30, 30).unwrap();
        add_game_score(&mut conn, game_uuid, UserID(5), 90, 30).unwrap();
        add_game_score(&mut conn, game_uuid, UserID(5), 10000, 60).unwrap();

        let stats = get_game_stats(&mut conn, game_uuid, 30).unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.average_score, 60.0);
        assert_eq!(stats.max_score, 90);
    }
}
