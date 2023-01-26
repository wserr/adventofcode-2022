use super::elf::Elf;
use super::elves::Elves;

pub fn fetch_day_1() -> Elves
{
    let input = include_str!("../../inputs/1/1.txt").lines();
    let mut result: Elves = Elves { elements: Vec::new() };

    let mut current_index: usize = 1;
    let mut current_element = Elf {
        index: current_index,
        calorie_count: Vec::new()
    };
    for line in input.into_iter() {
        match line {
            "" => {
                result.elements.push(current_element);
                current_index += 1;
                current_element = Elf {
                    index: current_index,
                    calorie_count: Vec::new()
                };
            },
            _ => {
                match line.parse::<usize>() {
                    Ok(result) => {
                        current_element.calorie_count.push(result);
                    },
                    Err(_) => {
                        panic!("Invalid input format.")
                    },
                }

             },
        }
    }
    match  result.elements.last() {
        Some(el) => {
            if el.index != current_element.index && current_element.calorie_count.len() > 0
            {
                result.elements.push(current_element);
            };
        },
        None => {
            if current_element.calorie_count.len() > 0
            {
                result.elements.push(current_element);
            };
        },
    }
    result
}

#[test]
fn test_day_1() {
    let result = fetch_day_1();
    println!("{:?} records were fetched.", result.elements.len());
}
