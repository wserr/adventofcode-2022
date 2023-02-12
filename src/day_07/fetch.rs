use super::commands::Command;
use regex::Regex;

const CD_COMMAND: &'static str = "$ cd ";
const LS_COMMAND: &'static str = "$ ls ";

// TO DO
const FILE_REGEX: &'static str = r"(\d+) (.*)";
const DIR_REGEX: &'static str = r"dir (.+)";

pub fn fetch_commands() -> Vec<Command> {
    let mut result = Vec::new();
    let lines = include_str!("../../inputs/7.txt").lines();
    let file_reg = Regex::new(FILE_REGEX).unwrap();
    let dir_reg = Regex::new(DIR_REGEX).unwrap();

    for line in lines {
        if line.starts_with(CD_COMMAND) {
            let dir_name = line.replace(CD_COMMAND, "");
            if dir_name == ".." {
                result.push(Command::MoveOut);
            } else if dir_name == "/" {
                result.push(Command::MoveHome);
            } else {
                result.push(Command::MoveIn(dir_name));
            }
        } else {
            for capture in file_reg.captures_iter(line) {
                result.push(Command::CreateFile(
                    capture[2].parse().unwrap(),
                    capture[1].parse::<usize>().unwrap(),
                ));
            }
            for capture in dir_reg.captures_iter(line) {
                result.push(Command::CreateDirectory(capture[1].parse().unwrap()));
            }
        }
    }
    result
}

#[test]
fn should_fetch_test_commands() {
    let result = fetch_commands();

    assert_eq!(783, result.len());
}
