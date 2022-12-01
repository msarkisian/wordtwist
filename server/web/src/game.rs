use std::collections::HashMap;

use wordtwist::game::GeneratedGame;

use crate::user::User;

pub struct Game<const N: usize> {
    data: GeneratedGame<N>,
    players: HashMap<User, usize>,
}

pub struct DailyGame<const N: usize> {
    game: Game<N>,
}
