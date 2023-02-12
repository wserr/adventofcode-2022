use regex::Regex;

use super::instructions::Instruction;

const NOOP_REGEX_STRING: &'static str = r"noop";
const ADD_REGEX_STRING: &'static str = r"add([a-z]) (-?\d+)";

pub fn fetch_instructions() -> Vec<Instruction> {
    let input = include_str!("../../inputs/10.txt").lines();
    let mut result = Vec::new();
    let noop_regex = Regex::new(NOOP_REGEX_STRING).unwrap();
    let add_regex = Regex::new(ADD_REGEX_STRING).unwrap();

    for line in input {
        if noop_regex.is_match(line) {
            result.push(Instruction::Noop);
        } else {
            for ma in add_regex.captures_iter(line) {
                result.push(Instruction::Add(ma[2].parse::<isize>().unwrap()));
            }
        }
    }
    result
}
