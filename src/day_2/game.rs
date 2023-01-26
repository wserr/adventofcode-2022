enum Move
{
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq)]
enum Player
{
    Player1,
    Player2
}

struct Game
{
    winner: Player,
    move_1: Move,
    move_2: Move
}

trait PointsCalculator
{
    fn calculate_points(&self) -> usize;
}

trait GamePlay
{
    fn calculate_score_for_player(&self, player: Player) -> usize;
    fn calculate_winner(&self) -> Player;
}

impl PointsCalculator for Move
{
    fn calculate_points(&self) -> usize {
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
    fn calculate_score_for_player(&self, player: Player) -> usize {
        let mut points: usize = 0;
        if self.calculate_winner() == player
        {
            points += 3;
        }
        points += &self.move_1.calculate_points() + &self.move_2.calculate_points();
        points
    }

    fn calculate_winner(&self) -> Player {

    }
}


