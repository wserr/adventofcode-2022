#[derive(PartialEq)]
pub enum Move
{
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Debug)]
pub enum Player
{
    Player1,
    Player2
}

pub struct Game
{
    // Move of player 1
    pub move_1: Move,

    // Move of player 2
    pub move_2: Move
}

trait PointsCalculator
{
    fn calculate_points(&self) -> i8;
}

pub trait GamePlay
{
    fn calculate_score_for_player(&self, player: Player) -> i16;
    fn calculate_winner(&self) -> Option<&Player>;
}

impl PointsCalculator for Move
{
    fn calculate_points(&self) -> i8 {
        match &self
        {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

impl GamePlay for Game
{
    fn calculate_score_for_player(&self, player: Player) -> i16 {
        let mut points: i16 = 0;
        if self.calculate_winner() == Some(&player)
        {
            points += 6;
        }
        else if self.calculate_winner() == None
        {
            points += 3;
        }
        //points += i16::from(&self.move_1.calculate_points() + &self.move_2.calculate_points());
        points += match &player {
            &Player::Player1 => i16::from(self.move_1.calculate_points()),
            &Player::Player2 => i16::from(self.move_2.calculate_points())
        };

        points
    }

    fn calculate_winner(&self) -> Option<&Player> {
        // If moves are equal - no winner
        // X: Rock
        // Y: Paper
        // Z: Scissors
        // X - Y -> 2
        // Y - X -> 1
        // Z - X -> 2
        // X - Z -> 1
        // Y - Z -> 2
        // Z - Y -> 1
        let sum: i8 = self.move_1.calculate_points() - self.move_2.calculate_points();
        match sum
        {
            0 => None,
            -1 | -2 => Some(&Player::Player2),
            _ => Some(&Player::Player1)
        }
    }
}

#[test]
fn test_game_player_1_should_win() {
    let game = Game {
        move_1: Move::Scissors,
        move_2: Move::Paper,
    };
    assert_eq!(Some(&Player::Player1), game.calculate_winner());
    assert_eq!(9, game.calculate_score_for_player(Player::Player1));
    assert_eq!(2, game.calculate_score_for_player(Player::Player2));
}

#[test]
fn test_game_player_2_should_win() {
    let game = Game {
        move_1: Move::Rock,
        move_2: Move::Paper,
    };
    assert_eq!(Some(&Player::Player2), game.calculate_winner());
    assert_eq!(1, game.calculate_score_for_player(Player::Player1));
    assert_eq!(8, game.calculate_score_for_player(Player::Player2));
}


#[test]
fn test_game_no_one_should_win() {
    let game = Game {
        move_1: Move::Rock,
        move_2: Move::Rock,
    };
    assert_eq!(None, game.calculate_winner());
    assert_eq!(4, game.calculate_score_for_player(Player::Player1));
    assert_eq!(4, game.calculate_score_for_player(Player::Player2));
}
