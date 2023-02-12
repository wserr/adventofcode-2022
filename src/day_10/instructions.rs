use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add(isize),
}

#[derive(Debug)]
pub struct Cycles(pub BTreeMap<usize, isize>);
pub struct NewInstruction(usize, isize);

pub fn calculate_cycles(instructions: Vec<Instruction>) -> Cycles {
    let cycles: Cycles = instructions
        .into_iter()
        .fold(Cycles(BTreeMap::new()), |mut acc, i| {
            if acc.0.len() == 0 {
                acc.0.insert(1, 1);
            }
            let last_cycle_key = acc.0.keys().max().unwrap();
            let last_cycle_value = acc.0[last_cycle_key];
            let next_cycle = calculate_cycle_for_instruction(*last_cycle_key, last_cycle_value, &i);
            acc.0.insert(next_cycle.0, next_cycle.1);
            acc
        });
    // add missing gaps
    cycles
        .0
        .into_iter()
        .fold(Cycles(BTreeMap::new()), |mut acc, i| {
            if acc.0.len() > 0 {
                let last_cycle_key = acc.0.keys().max().unwrap();
                let last_cycle_value = acc.0[last_cycle_key];
                let mut amount_of_cycles_to_add = i.0 - *last_cycle_key - 1;
                let mut next_cycle_key = *last_cycle_key + 1;
                let next_cycle_value = last_cycle_value;

                while amount_of_cycles_to_add > 0 {
                    acc.0.insert(next_cycle_key, next_cycle_value);
                    next_cycle_key += 1;
                    amount_of_cycles_to_add -= 1;
                }
            }
            acc.0.insert(i.0, i.1);
            acc
        })
}

fn calculate_cycle_for_instruction(
    current_cycle: usize,
    current_value: isize,
    instruction: &Instruction,
) -> NewInstruction {
    match instruction {
        &Instruction::Noop => NewInstruction(current_cycle + 1, current_value),
        &Instruction::Add(value) => NewInstruction(current_cycle + 2, current_value + value),
    }
}
