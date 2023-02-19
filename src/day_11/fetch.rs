use super::monkey::{Monkey, OperationFactor, Operator};
use regex::Regex;

const MONKEY_MATCH_REGEX: &'static str = r"Monkey (\d+):\n  Starting items: (\d+(, \d+)*)\n  Operation: new = old ([\*|\+\-]) (\d+|old)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)";

pub fn fetch_monkeys() -> Vec<Monkey> {
    let mut result = Vec::new();
    let input = include_str!("../../inputs/11.txt");

    let monkey_regex = Regex::new(MONKEY_MATCH_REGEX).unwrap();

    for monkey in monkey_regex.captures_iter(input) {
        result.push(Monkey {
            index: monkey[1].parse::<usize>().unwrap(),
            items: monkey[2]
                .split(", ")
                .map(|r| r.parse::<u128>().unwrap())
                .collect(),
            operation_operator: match monkey[4].parse::<char>().unwrap()
            {
                '*' => Operator::Multiply,
                '+' => Operator::Addition,
                '-' => Operator::Addition,
                _ => panic!("Invalid char")
            },
            operation_factor: match &monkey[5] {
                "old" => OperationFactor::Old,
                _ => OperationFactor::Number(monkey[5].parse::<u128>().unwrap()),
            },
            test_division_factor: monkey[6].parse::<u128>().unwrap(),
            true_condition_monkey_number: monkey[7].parse::<usize>().unwrap(),
            false_condition_monkey_number: monkey[8].parse::<usize>().unwrap(),
            amount_of_items_thrown: 0
        })
    }
    result
}
