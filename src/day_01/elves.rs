use super::elf::{Elf, TotalCalorieCountCalculator};
use std::collections::BTreeMap;

pub struct Elves {
    pub elements: Vec<Elf>,
}

pub trait MostWantedElfCalculator {
    fn calculate_most_wanted_elves(&self, amount: usize) -> Option<Vec<&Elf>>;
}

impl MostWantedElfCalculator for Elves {
    fn calculate_most_wanted_elves(&self, amount: usize) -> Option<Vec<&Elf>> {
        let mut elf_map: BTreeMap<usize, Vec<&Elf>> = BTreeMap::new();

        for elf in &self.elements {
            let calorie_count = elf.calculate_total_calorie_count();

            match elf_map.get_mut(&calorie_count) {
                Some(names) => {
                    names.push(elf);
                }
                None => {
                    elf_map.insert(calorie_count, vec![elf]);
                }
            };
        }

        if elf_map.len() == 0 {
            return None;
        }

        let mut elves: Vec<&Elf> = Vec::new();
        let mut index: usize = 1;

        for elf_entry in elf_map.iter().rev() {
            for elf in elf_entry.1.to_owned() {
                elves.push(elf);
                index += 1;
                if index > amount {
                    break;
                }
            }

            if index > amount {
                break;
            }
        }
        Some(elves)
    }
}

#[test]
fn calculate_most_wanted_elf_should_return_none() {
    let elves = Elves {
        elements: Vec::new(),
    };
    assert_eq!(None, elves.calculate_most_wanted_elves(1));
}

#[test]
fn calculate_most_wanted_elf_should_return_some() {
    let elves = Elves {
        elements: vec![Elf {
            index: 1,
            calorie_count: vec![1, 2, 3],
        }],
    };
    assert_eq!(
        Some(vec![&Elf {
            index: 1,
            calorie_count: vec![1, 2, 3]
        }]),
        elves.calculate_most_wanted_elves(1)
    );
}

#[test]
fn calculate_most_wanted_elf_should_return_some_2() {
    let elves = Elves {
        elements: vec![
            Elf {
                index: 1,
                calorie_count: vec![1, 2, 3],
            },
            Elf {
                index: 2,
                calorie_count: vec![1, 5, 3],
            },
        ],
    };
    assert_eq!(
        Some(vec![&Elf {
            index: 2,
            calorie_count: vec![1, 5, 3]
        }]),
        elves.calculate_most_wanted_elves(1)
    );
}

#[test]
fn calculate_most_wanted_elf_should_return_some_3() {
    let elves = Elves {
        elements: vec![
            Elf {
                index: 1,
                calorie_count: vec![1, 2, 3],
            },
            Elf {
                index: 2,
                calorie_count: vec![1, 5, 3],
            },
            Elf {
                index: 3,
                calorie_count: vec![1, 1, 1],
            },
        ],
    };
    let result = elves.calculate_most_wanted_elves(2);
    assert_eq!(
        Some(vec![
            &Elf {
                index: 2,
                calorie_count: vec![1, 5, 3]
            },
            &Elf {
                index: 1,
                calorie_count: vec![1, 2, 3]
            }
        ]),
        result
    );
}
