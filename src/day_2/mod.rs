use self::game::{GamePlay, Player};

mod game;
mod fetch;



pub fn solution_1 () -> String
{
    let games = fetch::fetch_day_2();
    let sum: i16 = games.into_iter().map(|g| g.calculate_score_for_player(Player::Player2)).sum();

    format!("Player 2 scored {:?} points", sum)
}
