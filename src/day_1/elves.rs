use std::collections::BTreeMap;
use super::elf::{Elf, TotalCalorieCountCalculator};

pub struct Elves {
    pub elements: Vec<Elf>
}

pub trait MostWantedElfCalculator {
    fn calculate_most_wanted_elf(&self) -> Option<&Elf>; 
}

impl MostWantedElfCalculator for Elves {
    fn calculate_most_wanted_elf(&self) -> Option<&Elf> {
        let mut elf_map: BTreeMap<usize, Vec<&Elf>> = BTreeMap::new();

        for elf in &self.elements  {
            let calorie_count = elf.calculate_total_calorie_count();

            match elf_map.get_mut(&calorie_count) {
                Some(names) => { names.push(elf); },
                None => { elf_map.insert(calorie_count, vec![elf]); },
            };
        };

        if elf_map.len() == 0 {
            return None;
        }

        let max_calorie_count = elf_map.iter().map(|e| e.0).max().unwrap();
        let elves_with_max_calorie_count = elf_map.get(max_calorie_count).unwrap();

        match elves_with_max_calorie_count.len() {
            1 => Some(elves_with_max_calorie_count.first().unwrap()),
            _ => None,
        }
    }
}

#[test]
fn calculate_most_wanted_elf_should_return_none() {
    let elves = Elves {
        elements: Vec::new()
    };
    assert_eq!(None, elves.calculate_most_wanted_elf());
}

#[test]
fn calculate_most_wanted_elf_should_return_some() {
    let elves = Elves {
        elements: vec![Elf {
            index: 1,
            calorie_count: vec![1, 2, 3]
        }]
    };
    assert_eq!(Some(&Elf { index: 1, calorie_count: vec![1, 2, 3]}), elves.calculate_most_wanted_elf());
}

#[test]
fn calculate_most_wanted_elf_should_return_some_2() {
    let elves = Elves {
        elements: vec![Elf {
            index: 1,
            calorie_count: vec![1, 2, 3]
        },Elf {
            index: 2,
            calorie_count: vec![1, 5, 3]
        }]
    };
    assert_eq!(Some(&Elf { index: 2, calorie_count: vec![1, 5, 3]}), elves.calculate_most_wanted_elf());
}
