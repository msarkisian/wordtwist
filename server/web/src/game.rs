use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wordtwist::game::Game as GameData;

use crate::db::{
    game::{set_daily, try_get_daily},
    open_db_connection,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Game {
    id: String,
    data: GameData,
}

pub struct DailyGame(pub GameData);

impl Game {
    pub fn new(size: usize) -> Self {
        let mut conn = open_db_connection();
        let data = GameData::new(size);

        let uuid = crate::db::game::insert_game(&mut conn, &data).unwrap();
        Self {
            id: uuid.to_string(),
            data,
        }
    }

    pub fn from(uuid: Uuid, data: GameData) -> Self {
        Self {
            id: uuid.to_string(),
            data,
        }
    }
}

impl DailyGame {
    pub fn get() -> Self {
        let mut conn = open_db_connection();
        Self(try_get_daily(&mut conn).unwrap_or_else(|_| {
            let game = Game::new(4);
            set_daily(&mut conn, Uuid::parse_str(game.id.as_str()).unwrap())
                .expect("error adding daily game to db (DailyGame::get)");
            game.data
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_daily_game() {
        let g1 = DailyGame::get();
        let g2 = DailyGame::get();

        assert_eq!(g1.0, g2.0);
    }
}
