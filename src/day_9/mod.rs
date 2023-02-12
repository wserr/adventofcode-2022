use self::bridge::calculate_amount_of_tail_positions;

mod bridge;
mod fetch;
mod map;

pub fn solution_1() -> String {
    let input = fetch::fetch_directions();
    let result = calculate_amount_of_tail_positions(&input, 1);
    format!("The amount of positions is {:?}", result)
}

pub fn solution_2() -> String {
    let input = fetch::fetch_directions();
    let result = calculate_amount_of_tail_positions(&input, 9);
    format!("The amount of positions is {:?}", result)
}
