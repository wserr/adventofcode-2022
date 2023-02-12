use self::instructions::calculate_cycles;

mod fetch;
mod instructions;
mod display;

pub fn solution_1() -> String {
    let indeces_to_evaluate: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let input = fetch::fetch_instructions();
    let result = calculate_cycles(input);

    let sum = indeces_to_evaluate.into_iter().fold(0, |mut acc, e| {
        let e_usize: isize = e.try_into().unwrap();
        acc += result.0[&e] * e_usize;
        acc
    });

    format!("Value is {:?}", sum)
}

pub fn solution_2() -> String {
    let input = fetch::fetch_instructions();
    let result = calculate_cycles(input);

    format!("{}", result)
}
