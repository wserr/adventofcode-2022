use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Crates
{
    pub stacks: Vec<VecDeque<String>> 
}

pub struct MoveOperation
{
    pub source_pos: usize,
    pub target_pos: usize,
    pub amount: usize
}

pub fn move_crates_multiple<'a>(stacks: &'a mut Vec<VecDeque<String>>, move_operations: &Vec<MoveOperation>) -> Result<&'a mut Vec<VecDeque<String>>, &'static str>
{
    for operation in move_operations.into_iter() {
        move_crates(stacks, operation);
    }
    Ok(stacks)
}

fn move_crates<'a>(stacks: &'a mut Vec<VecDeque<String>>, move_operation: &MoveOperation)
{
    let source_stack = stacks.get_mut(move_operation.source_pos - 1).unwrap();
    let mut items_to_push: VecDeque<String> = VecDeque::new();
    let mut index = 0;
    while index < move_operation.amount
    {
        items_to_push.push_back(source_stack.pop_back().unwrap());
        index += 1;
    }

    let target_stack = stacks.get_mut(move_operation.target_pos - 1).unwrap();
    while let Some(result) = items_to_push.pop_front()
    {
        target_stack.push_back(result);
    }
}

#[test]
fn test_move_crates_multiple_success() {
    let mut crates = Crates {
        stacks: Vec::from([ VecDeque::from([String::from("A"), String::from("B"), String::from("C")]), VecDeque::from([String::from("D")]), VecDeque::from([String::from("E"), String::from("F"), String::from("G")])])
    };

    let move_operations = Vec::from([
    MoveOperation {
        source_pos: 1,
        target_pos: 2,
        amount: 2
    },
    MoveOperation {
        source_pos: 2,
        target_pos: 3,
        amount: 1
    }]);

    let result = move_crates_multiple(&mut crates.stacks, &move_operations);

    assert_eq!(Ok(&mut Vec::from([VecDeque::from([String::from("A")]), VecDeque::from([String::from("D"), String::from("C")]), VecDeque::from([String::from("E"), String::from("F"), String::from("G"), String::from("B")])])), result);
}

#[test]
fn test_move_crates_success() {
    let mut crates = Crates {
        stacks: Vec::from([ VecDeque::from([String::from("A"), String::from("B"), String::from("C")]), VecDeque::from([String::from("D")])])
    };

    let move_operation = MoveOperation {
        source_pos: 1,
        target_pos: 2,
        amount: 1
    };

    move_crates(&mut crates.stacks, &move_operation);

    assert_eq!(Vec::from([ VecDeque::from([String::from("A"), String::from("B")]), VecDeque::from([String::from("D"), String::from("C")])]), crates.stacks)
}
