use self::pairs::OverlapCalculator;

mod fetch;
mod pairs;

pub fn solution_1() -> String {
    let pairs = fetch::fetch_day_4();

    let result_values: Vec<bool> = pairs
        .into_iter()
        .map(|x| x.full_overlap())
        .filter(|y| *y)
        .collect();

    format!(
        "The amount of pairs with full overlap is {:?}",
        result_values.len()
    )
}

pub fn solution_2() -> String {
    let pairs = fetch::fetch_day_4();

    let result_values: Vec<bool> = pairs
        .into_iter()
        .map(|x| x.partial_overlap())
        .filter(|y| *y)
        .collect();

    format!(
        "The amount of pairs with partial overlap is {:?}",
        result_values.len()
    )
}
