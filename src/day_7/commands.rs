use super::tree::{Tree, TreeCommander, TreeValue};
use crate::day_7::tree::TreeQuerier;
use std::collections::BTreeMap;

pub enum Command {
    MoveIn(String),
    MoveOut,
    MoveHome,
    CreateFiles(Vec<(String, usize)>),
}

pub fn execute_commands(tree: &mut Tree, commands: &Vec<Command>) {
    let mut current_path: String = "/".to_string();
    for command in commands {
        match command {
            Command::MoveIn(value) => {
                let mut new_path = current_path.clone();
                new_path.push_str(value);
                new_path.push_str("/");
                current_path = tree.get_or_create_folder(new_path).unwrap();
            }
            Command::MoveOut => {
                let mut path: Vec<&str> = current_path.split("/").collect();
                path.pop();
                path.pop();
                current_path = path.join("/");
            }
            Command::MoveHome => {
                current_path = "/".to_string();
            }
            Command::CreateFiles(items) => {
                for file in items {
                    tree.create_file(current_path.as_str(), file.0.as_str(), file.1)
                        .unwrap();
                }
            }
        };
    }
}

#[test]
fn should_create_directory() {
    let commands = Vec::from([Command::MoveIn(String::from("test"))]);
    let items = BTreeMap::from([("/".to_string(), TreeValue::Folder)]);

    let mut tree = Tree { items };

    execute_commands(&mut tree, &commands);
    assert_eq!(2, tree.items.len());
    assert_eq!(0, tree.calculate_file_size_in_folder("/").unwrap());
}

#[test]
fn should_create_files_in_subfolder() {
    let commands = Vec::from([
        Command::MoveIn(String::from("test")),
        Command::MoveIn(String::from("test_2")),
        Command::CreateFiles(Vec::from([("Test.txt".to_string(), 25)])),
    ]);
    let items = BTreeMap::from([("/".to_string(), TreeValue::Folder)]);

    let mut tree = Tree { items };

    execute_commands(&mut tree, &commands);
    assert_eq!(4, tree.items.len());
    assert_eq!(25, tree.calculate_file_size_in_folder("/").unwrap());
    assert_eq!(
        25,
        tree.calculate_file_size_in_folder("/test/test_2/").unwrap()
    );
}
