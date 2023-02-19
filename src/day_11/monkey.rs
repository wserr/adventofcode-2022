use std::collections::HashMap;

#[derive(Clone)]
pub struct Monkey {
    pub index: usize,
    pub items: Vec<u128>,
    pub operation_operator: Operator,
    pub operation_factor: OperationFactor,
    pub test_division_factor: u128,
    pub true_condition_monkey_number: usize,
    pub false_condition_monkey_number: usize,
    pub amount_of_items_thrown: usize,
}

#[derive(Clone)]
pub enum OperationFactor {
    Number(u128),
    Old,
}

#[derive(Clone)]
pub enum Operator {
    Multiply,
    Addition,
}

pub fn calculate_round(monkeys: &Vec<Monkey>) -> Vec<Monkey> {
    let mut i = 0;
    let mut new_monkey_state = monkeys.clone();

    while i < new_monkey_state.len() {
        let monkey = new_monkey_state.get_mut(i).unwrap().clone();
        let operations = calculate_monkey_move(&monkey);
        new_monkey_state = new_monkey_state
            .into_iter()
            .map(|mut m| match operations.get(&m.index) {
                Some(values) => {
                    for value in values.into_iter() {
                        m.items.push(*value);
                    }
                    m
                }
                None => {
                    if monkey.index == m.index {
                        m.amount_of_items_thrown += m.items.len();
                        m.items = Vec::new();
                    }
                    m
                }
            })
            .collect();
        i += 1;
    }
    new_monkey_state
}

pub fn calculate_monkey_move(monkey: &Monkey) -> HashMap<usize, Vec<u128>> {
    let mut result: HashMap<usize, Vec<u128>> = HashMap::new();

    let calc_second_operator =
        |old: u128, operation_factor: &OperationFactor| match operation_factor {
            &OperationFactor::Old => old,
            &OperationFactor::Number(result) => result,
        };

    let calc_new_worry_level = |x: u128, y: u128, operator: &Operator| match operator {
        &Operator::Addition => x + y,
        &Operator::Multiply => x * y,
    };

    let insert_into_hashmap =
        |h: &mut HashMap<usize, Vec<u128>>, id: usize, worry_level: u128| {
            h.entry(id)
                .and_modify(|x| x.push(worry_level))
                .or_insert(Vec::from([worry_level]));
        };

    for item in monkey.items.iter() {
        let mut new_worry_level = calc_new_worry_level(
            *item,
            calc_second_operator(*item, &monkey.operation_factor),
            &monkey.operation_operator,
        );

        // In rust, division is already floored for all integer types
        new_worry_level = new_worry_level / 3;

        match new_worry_level % monkey.test_division_factor {
            0 => insert_into_hashmap(
                &mut result,
                monkey.true_condition_monkey_number,
                new_worry_level,
            ),
            _ => insert_into_hashmap(
                &mut result,
                monkey.false_condition_monkey_number,
                new_worry_level,
            ),
        };
    }
    result
}

#[test]
fn test_calculate_monkey_move() {
    let monkey = Monkey {
        index: 1,
        items: Vec::from([1, 2, 3]),
        operation_operator: Operator::Addition,
        operation_factor: OperationFactor::Old,
        test_division_factor: 4,
        true_condition_monkey_number: 1,
        false_condition_monkey_number: 2,
        amount_of_items_thrown: 0,
    };

    let result = calculate_monkey_move(&monkey);

    assert_eq!(3, result.len());
}
