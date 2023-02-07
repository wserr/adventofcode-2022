use std::collections::BTreeMap;

use self::commands::execute_commands;
use self::tree::{Tree, TreeQuerier, TreeValue};

mod commands;
mod fetch;
mod tree;

pub fn solution_1() -> String {
    let input = fetch::fetch_commands();
    let items = BTreeMap::from([("/".to_string(), TreeValue::Folder)]);
    let mut tree = Tree { items };
    execute_commands(&mut tree, &input);

    let mut sum = 0;
    for item in tree.items.keys() {
        println!("{:?}", item);
        if item.ends_with("/") {
            let size = tree.calculate_file_size_in_folder(item).unwrap();
            if size < 100_000 {
                sum += size;
            }
        }
    }
    format!("The total size is {:?}", sum)
}

pub fn solution_2() -> String {
    let input = fetch::fetch_commands();
    let items = BTreeMap::from([("/".to_string(), TreeValue::Folder)]);
    let mut tree = Tree { items };
    execute_commands(&mut tree, &input);

    let disk_size_to_clean_up =
        30_000_000 - (70_000_000 - tree.calculate_file_size_in_folder("/").unwrap());

    let mut currently_smallest_folder_size = 0;
    for item in tree.items.keys() {
        if item.ends_with("/") {
            let size = tree.calculate_file_size_in_folder(item).unwrap();

            if size > disk_size_to_clean_up
                && (size < currently_smallest_folder_size || currently_smallest_folder_size == 0)
            {
                currently_smallest_folder_size = size;
            }
        }
    }
    format!(
        "The size of the folder that should be deleted is is {:?}",
        currently_smallest_folder_size
    )
}
