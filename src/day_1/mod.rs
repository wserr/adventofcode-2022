use self::{elves::MostWantedElfCalculator, elf::TotalCalorieCountCalculator};

pub mod elf;
pub mod elves;
pub mod fetch;

pub fn solution_1 () -> String
{
    let elves = fetch::fetch_day_1();
    let result = elves.calculate_most_wanted_elf().unwrap();
    format!("Elf {:?} has the most calories.", result.calculate_total_calorie_count())
}
