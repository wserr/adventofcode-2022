mod fetch;
mod marker;

pub fn solution_1() -> String {
    let input = fetch::fetch_input();
    let result = marker::calculate_marker(input, 4);

    format!("The first marker is at position {:?}", result)
}

pub fn solution_2() -> String {
    let input = fetch::fetch_input();
    let result = marker::calculate_marker(input, 14);

    format!("The first marker is at position {:?}", result)
}
