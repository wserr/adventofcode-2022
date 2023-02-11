use self::bridge::calculate_amount_of_tail_positions;

mod bridge;
mod fetch;


pub fn solution_1() -> String {
    let input = fetch::fetch_directions();
    let result = calculate_amount_of_tail_positions(&input);
    format!("The amount of positions is {:?}", result)
}
