pub fn fetch() -> Vec<Vec<usize>> {
    let input = include_str!("../../inputs/8.txt");
    let mut result: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        let mut new_line: Vec<usize> = Vec::new();
        for char in line.chars() {
            new_line.push(char.to_string().parse::<usize>().unwrap());
        }
        result.push(new_line);
    }
    result
}
