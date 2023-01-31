pub fn fetch_day_3() -> Vec<String>
{
    include_str!("../../inputs/3.txt").lines().map(|r| r.to_string()).collect()
}

pub fn fetch_day_3_2() -> Vec<Vec<String>>
{
    let mut result = Vec::new();

    let input : Vec<String> = include_str!("../../inputs/3.txt").lines().map(|r| r.to_string()).collect();
    let length = input.len();
    let mut input_iter = input.into_iter();

    let mut index = 0;

    while index < length / 3
    {
        let element = (0..3).map(|_x| input_iter.next().unwrap()).collect();
        result.push(element);
        index += 1;
    }
    result
}

#[test]
fn fetch_day_3_should_fetch_300_items() {
    assert_eq!(300, fetch_day_3().len());
}

#[test]
fn fetch_day_3_2_should_fetch_100_items() {
    assert_eq!(100, fetch_day_3_2().len());
}



