#[derive(PartialEq, Debug)]
pub struct Elf {
    pub index: usize,
    pub calorie_count: Vec<usize>,
}

impl TotalCalorieCountCalculator for Elf {
    fn calculate_total_calorie_count(&self) -> usize {
        let mut result: usize = 0;
        for number in &self.calorie_count {
            result += number;
        }
        result
    }
}

pub trait TotalCalorieCountCalculator {
    fn calculate_total_calorie_count(&self) -> usize;
}

#[test]
fn should_calculate_total_calorie_count() {
    let mut calorie_count: Vec<usize> = Vec::new();
    calorie_count.push(5);
    calorie_count.push(6);
    let food_item = Elf {
        index: 1,
        calorie_count,
    };
    assert_eq!(11, food_item.calculate_total_calorie_count());
}

#[test]
fn should_calculate_total_calorie_count_no_calories() {
    let calorie_count: Vec<usize> = Vec::new();
    let food_item = Elf {
        index: 1,
        calorie_count,
    };
    assert_eq!(0, food_item.calculate_total_calorie_count());
}
