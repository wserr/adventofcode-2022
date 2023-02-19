use self::rucksack::{calculate_priority, find_common_items, find_common_items_in_group};

mod fetch;
mod rucksack;

pub fn solution_1() -> String {
    let rucksacks = fetch::fetch_day_3();
    let common_items: Vec<char> = rucksacks
        .into_iter()
        .map(|r| find_common_items(r))
        .flatten()
        .collect();
    let sum: u32 = common_items
        .into_iter()
        .map(|c| calculate_priority(&c).unwrap())
        .sum();

    format!("The sum of priority is {:?}", sum)
}

pub fn solution_2() -> String {
    let rucksacks = fetch::fetch_day_3_2();
    let common_items: Vec<char> = rucksacks
        .into_iter()
        .map(|r| find_common_items_in_group(r))
        .flatten()
        .collect();
    let sum: u32 = common_items
        .into_iter()
        .map(|c| calculate_priority(&c).unwrap())
        .sum();

    format!("The sum of priority is {:?}", sum)
}
