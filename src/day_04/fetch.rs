use super::pairs::Pair;
use regex::Regex;

pub fn fetch_day_4() -> Vec<Pair> {
    include_str!("../../inputs/4.txt")
        .lines()
        .map(|r| calculate_pair_from_string(r))
        .collect()
}

fn calculate_pair_from_string(input: &str) -> Pair {
    // Pair: 2-4,6-8
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    if !re.is_match(input) {
        panic!("{:?} has invalid format", input);
    } else {
        let mut pair = Pair {
            range_1: Vec::new(),
            range_2: Vec::new(),
        };

        for item in re.captures_iter(input) {
            pair = Pair {
                range_1: (item[1].parse::<u8>().unwrap()..item[2].parse::<u8>().unwrap() + 1)
                    .collect(),
                range_2: (item[3].parse::<u8>().unwrap()..item[4].parse::<u8>().unwrap() + 1)
                    .collect(),
            };
        }
        pair
    }
}

#[test]
fn test_calculate_pair_from_string() {
    let result = calculate_pair_from_string("1-2,5-7");
    assert_eq!(
        Pair {
            range_1: Vec::from([1, 2]),
            range_2: Vec::from([5, 6, 7])
        },
        result
    )
}

#[test]
fn should_fetch_all_pairs() {
    assert_eq!(1000, fetch_day_4().len());
}
