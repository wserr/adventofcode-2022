use self::monkey::calculate_round;

mod fetch;
mod monkey;

pub fn solution_1() -> String {
    let result = fetch::fetch_monkeys();

    let mut amount_of_rounds = 20;
    let mut new_round = result;

    while amount_of_rounds > 0 {
        new_round = calculate_round(&new_round);
        amount_of_rounds -= 1;
    }

    let mut count_map: Vec<usize> = new_round.iter().map(|m| m.amount_of_items_thrown).collect();

    let max_1 = count_map.clone().into_iter().max().unwrap();

    count_map = count_map.into_iter().filter(|c| *c != max_1).collect();

    let max_2 = count_map.clone().into_iter().max().unwrap();

    format!("The monkey business amount is equal to {:?}", max_1 * max_2)
}
