use std::collections::VecDeque;

use super::crates::{Crates, MoveOperation};
use regex::Regex;

fn fetch_input() -> Vec<String> {
    include_str!("../../inputs/5.txt")
        .lines()
        .map(|r| r.to_string())
        .collect()
}

pub fn fetch_moves() -> Vec<MoveOperation> {
    let mut input = fetch_input();
    let input_drained: Vec<String> = input.drain(find_whitespace_index(&input) + 1..).collect();

    input_drained
        .into_iter()
        .map(|x| fetch_move_operation_from_str(&x))
        .collect()
}

fn fetch_move_operation_from_str(input: &str) -> MoveOperation {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let matching_element = re.captures_iter(input).last().unwrap();
    MoveOperation {
        amount: matching_element[1].parse().unwrap(),
        source_pos: matching_element[2].parse().unwrap(),
        target_pos: matching_element[3].parse().unwrap(),
    }
}

pub fn fetch_crates() -> Crates {
    let mut input = fetch_input();
    let mut input_drained: Vec<String> = input.drain(0..find_whitespace_index(&input)).collect();

    let first_row = input_drained.pop().unwrap();
    let amount = fetch_amount_of_groups_from_first_row(&first_row);

    let mut crates: Crates = Crates { stacks: Vec::new() };

    initialize_stacks(&mut crates.stacks, amount);

    while let Some(output) = input_drained.pop() {
        append_stacks(&mut crates.stacks, &output);
    }
    crates
}

fn append_stacks(stacks: &mut Vec<VecDeque<String>>, input: &str) {
    let re = Regex::new(r" ?\[([A-Z])\]|(   ) ?").unwrap();

    let mut index = 0;
    for item in re.captures_iter(input) {
        if None != item.get(1) {
            stacks[index].push_back(item[1].to_string());
        }
        index += 1;
    }
}

fn initialize_stacks(stacks: &mut Vec<VecDeque<String>>, amount: u8) {
    let mut already_created = 0;

    while already_created < amount {
        stacks.push(VecDeque::new());
        already_created += 1;
    }
}

fn fetch_amount_of_groups_from_first_row(input: &str) -> u8 {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut amount: u8 = 0;
    for _item in re.captures_iter(input) {
        amount += 1;
    }
    amount
}

#[test]
fn test_fetch_amount_of_groups_from_first_row() {
    let test = " 1   2   3   4   5   6   7   8   9 ";
    assert_eq!(9, fetch_amount_of_groups_from_first_row(test));
}

fn find_whitespace_index(input: &Vec<String>) -> usize {
    let mut index = 0;
    for line in input.iter() {
        if line == "" {
            break;
        }
        index += 1;
    }
    index
}

#[test]
fn test_find_whitespace_index_should_return_correct_index() {
    let lines = Vec::from(["test".to_string(), "".to_string(), "test12".to_string()]);
    assert_eq!(1, find_whitespace_index(&lines));
}
