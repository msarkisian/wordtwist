use chrono::Utc;
use rusqlite::{Connection, Result};
use uuid::Uuid;
use wordtwist::game::Game;

pub fn get_game_by_id(conn: &mut Connection, id: Uuid) -> Result<Game> {
    conn.query_row_and_then(
        "SELECT game_data from GAMES where id=?1",
        (id.to_string(),),
        |row| Ok(serde_json::from_str(&row.get::<usize, String>(0)?).unwrap()),
    )
}

pub fn insert_game(conn: &mut Connection, game: &Game) -> Result<Uuid> {
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
        "SELECT games.game_data FROM dates JOIN daily ON dates.daily_id = daily.id JOIN games ON daily.game_id = games.id WHERE date = ?1",
        (date,),
        |r| r.get::<usize, String>(0))
    .map(|d| serde_json::from_str(&d).unwrap())
}

pub fn set_daily(conn: &mut Connection, id: Uuid) -> Result<()> {
    let date = Utc::now().date_naive().to_string();
    let daily_id: Result<usize, rusqlite::Error> = conn.query_row(
        "INSERT INTO daily (game_id) VALUES (?1) RETURNING daily.id",
        (id.to_string(),),
        |r| r.get(0),
    );
    conn.execute(
        "INSERT INTO dates (date, daily_id) VALUES (?1, ?2)",
        (date, daily_id.unwrap()),
    )
    .expect("error adding to dates table (set_daily)");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::db::open_db_connection;

    use super::*;

    #[test]
    fn insert_and_get() {
        let mut connection = open_db_connection();
        let game = Game::new(5);

        let game_id = insert_game(&mut connection, &game).unwrap();
        let fetched_game = get_game_by_id(&mut connection, game_id).unwrap();

        assert_eq!(game, fetched_game);
    }
}
