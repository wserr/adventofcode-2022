use self::trees::{calculate_amount_of_visible_trees, calculate_max_scenic_score};

mod fetch;
mod trees;

pub fn solution_1() -> String {
    let input = fetch::fetch();

    let result = calculate_amount_of_visible_trees(&input);

    format!("Amount of visible trees is {:?}", result)
}

pub fn solution_2() -> String {
    let input = fetch::fetch();

    let result = calculate_max_scenic_score(&input);

    format!("Maximum scenic score is {:?}", result)
}
