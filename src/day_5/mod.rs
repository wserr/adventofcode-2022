mod crates;
mod fetch;

pub fn solution_1 () -> String
{
    let mut crates = fetch::fetch_crates();
    let moves = fetch::fetch_moves();

    crates::move_crates_multiple(&mut crates.stacks, &moves).unwrap();

    let result: Vec<String> = crates.stacks.into_iter().map(|mut x| x.pop_back().unwrap()).collect();

    format!("The top of the stack is {:?}", result.join(""))

}
