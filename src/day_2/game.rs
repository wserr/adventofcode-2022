#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum Outcome
{
    Win,
    Lose,
    Draw
}

#[derive(Debug, PartialEq)]
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

pub trait GamePlay
{
    fn calculate_score_for_player(&self, player: Player) -> i16;
    fn calculate_winner(&self) -> Option<&Player>;
}

pub trait GameTransformer
{
    // Views the second move of the input game as the desired outcome
    // Based on the desired outcome, the second move will be converted and the result will be
    // transformed into a new game
    fn transform_game(&self) -> Game;
}

impl GameTransformer for Game
{
    fn transform_game(&self) -> Game {

        let outcome = match self.move_2 {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win
        };
        let move_2 = calculate_move_for_desired_outcome(&self.move_1, outcome);

        Game { move_1: self.move_1.clone(), move_2 }

    }

}

#[test]
fn transform_game_should_transform_game() {
    let input = Game {
        move_1: Move::Scissors,
        move_2: Move::Scissors
    };
    assert_eq!(Game { move_1: Move::Scissors, move_2: Move::Rock} ,input.transform_game());
}

fn calculate_move_for_desired_outcome(opponent_move: &Move, outcome: Outcome) -> Move
{
    match outcome {
        Outcome::Draw => opponent_move.clone(),
        Outcome::Win => match opponent_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock }
        Outcome::Lose => match opponent_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper }
    }
}

#[test]
fn calculate_move_for_desired_outcome_should_return_scissors() {
    assert_eq!(Move::Scissors, calculate_move_for_desired_outcome(&Move::Rock, Outcome::Lose));
    assert_eq!(Move::Scissors, calculate_move_for_desired_outcome(&Move::Scissors, Outcome::Draw));
    assert_eq!(Move::Scissors, calculate_move_for_desired_outcome(&Move::Paper, Outcome::Win));
}

#[test]
fn calculate_move_for_desired_outcome_should_return_rock() {
    assert_eq!(Move::Rock, calculate_move_for_desired_outcome(&Move::Paper, Outcome::Lose));
    assert_eq!(Move::Rock, calculate_move_for_desired_outcome(&Move::Rock, Outcome::Draw));
    assert_eq!(Move::Rock, calculate_move_for_desired_outcome(&Move::Scissors, Outcome::Win));
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
        points += match &player {
            &Player::Player1 => i16::from(self.move_1.calculate_points()),
            &Player::Player2 => i16::from(self.move_2.calculate_points())
        };

        points
    }

    fn calculate_winner(&self) -> Option<&Player> {
        // If moves are equal - no winner
        // 1: Rock
        // 2: Paper
        // 3: Scissors
        // 1 - 2 -> 2 (-1)
        // 2 - 1 -> 1 (1)
        // 3 - 1 -> 2 (2)
        // 1 - 3 -> 1 (-2)
        // 2 - 3 -> 2 (-1)
        // 3 - 2 -> 1 (1)
        let sum: i8 = self.move_1.calculate_points() - self.move_2.calculate_points();
        match sum
        {
            0 => None,
            -1 | 2 => Some(&Player::Player2),
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
