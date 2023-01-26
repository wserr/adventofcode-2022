use self::{elves::MostWantedElfCalculator, elf::TotalCalorieCountCalculator};

pub mod elf;
pub mod elves;
pub mod fetch;

pub fn solution_1 () -> String
{
    let elves = fetch::fetch_day_1();
    let result = elves.calculate_most_wanted_elves(1).unwrap();

    match result.first() {
        Some(res) => format!("Elf {:?} has the most calories, more specifically, {:?}.", res.index, res.calculate_total_calorie_count()),
        None => "Invalid solution".to_owned()
    }

}

pub fn solution_2 () -> String
{
    let elves = fetch::fetch_day_1();
    let result = elves.calculate_most_wanted_elves(3).unwrap();

    let sum: usize = result.iter().map(|x| x.calculate_total_calorie_count()).sum();

    format!("The calorie amount of the top 3 elves is {:?}", sum)

}
