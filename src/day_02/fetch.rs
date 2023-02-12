use regex::Regex;

use super::game::{Move, Game};

pub fn fetch_day_2() -> Vec<Game> {
    let input = include_str!("../../inputs/2.txt").lines();
    let mut result: Vec<Game> = Vec::new();

    let re = Regex::new(r"(A|B|C) (X|Y|Z)").unwrap();
    let mut line_number: usize = 1;
    for line in input.into_iter() {
        if ! re.is_match(line)
        {
            panic!("{:?} has invalid format (line number: {:?}", line, line_number);
        }
        else {
            let mut move_1: Move = Move::Rock;
            let mut move_2: Move = Move::Rock;

            for item in re.captures_iter(line)  {
                move_1 = match &item[1]
                {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    _ => Move::Scissors
                };
                move_2 = match &item[2]
                {
                    "X" => Move::Rock,
                    "Y" => Move::Paper,
                    _ => Move::Scissors
                };
            }

            result.push(Game { move_1, move_2 });
        }
        line_number += 1;
    }
    result
}

#[test]
fn should_fetch_day_2() {
    let result = fetch_day_2();
    assert_eq!(2500, result.len());
}
