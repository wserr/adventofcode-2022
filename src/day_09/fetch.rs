use regex::Regex;

use super::bridge::{Direction, Instruction};

pub fn fetch_directions() -> Vec<Instruction> {
    let input = include_str!("../../inputs/09.txt").lines();

    let regex = Regex::new(r"([UDLR]) (\d+)").unwrap();
    let mut result = Vec::new();

    for line in input.into_iter() {
        for matches in regex.captures_iter(line) {
            result.push(Instruction {
                direction: parse_letter_to_direction(&matches[1]).unwrap(),
                quantity: matches[2].parse::<isize>().unwrap(),
            })
        }
    }
    result
}

fn parse_letter_to_direction(letter: &str) -> Option<Direction> {
    match letter {
        "U" => Some(Direction::Up),
        "D" => Some(Direction::Down),
        "L" => Some(Direction::Left),
        "R" => Some(Direction::Right),
        _ => None,
    }
}
