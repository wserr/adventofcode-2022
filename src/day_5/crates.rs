use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Crates
{
    pub stacks: Vec<VecDeque<String>> 
}


#[derive(Debug)]
pub struct MoveOperation
{
    pub source_pos: usize,
    pub target_pos: usize,
    pub amount: usize
}

pub fn move_crates_multiple<'a>(stacks: &'a mut Vec<VecDeque<String>>, move_operations: &Vec<MoveOperation>, multiple_crates_at_once: bool) -> Result<&'a mut Vec<VecDeque<String>>, &'static str>
{
    for operation in move_operations.into_iter() {
        move_crates(stacks, operation, multiple_crates_at_once);
    }
    Ok(stacks)
}

fn move_crates<'a>(stacks: &'a mut Vec<VecDeque<String>>, move_operation: &MoveOperation, multiple_crates_at_once: bool)
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
    while let Some(result) = if multiple_crates_at_once { items_to_push.pop_back() } else { items_to_push.pop_front() }
    {
        target_stack.push_back(result);
    }
}

#[test]
fn test_move_crates_multiple_at_once() {
    let mut crates = Crates {
        stacks: Vec::from([ VecDeque::from([str("A"), str("B"), str("C")]), VecDeque::from([str("D")]), VecDeque::from([str("E"), str("F"), str("G")])])
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

    let result = move_crates_multiple(&mut crates.stacks, &move_operations, true);

    assert_eq!(Ok(&mut Vec::from([VecDeque::from([str("A")]), VecDeque::from([str("D"), str("B")]), VecDeque::from([str("E"), str("F"), str("G"), str("C")])])), result);

}

#[test]
fn test_move_crates_multiple() {
    let mut crates = Crates {
        stacks: Vec::from([ VecDeque::from([str("A"), str("B"), str("C")]), VecDeque::from([str("D")]), VecDeque::from([str("E"), str("F"), str("G")])])
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
    },
    MoveOperation {
        source_pos: 3,
        target_pos: 1,
        amount: 3
    },
    MoveOperation {
        source_pos: 1,
        target_pos: 2,
        amount: 4
    }]);

    let result = move_crates_multiple(&mut crates.stacks, &move_operations, false);

    assert_eq!(Ok(&mut Vec::from([VecDeque::from([]), VecDeque::from([str("D"), str("C"), str("F"), str("G"), str("B"), str("A")]), VecDeque::from([str("E")])])), result);
}

#[test]
fn test_move_crates_success() {
    let mut crates = Crates {
        stacks: Vec::from([ VecDeque::from([str("A"), str("B"), str("C")]), VecDeque::from([str("D")])])
    };

    let move_operation = MoveOperation {
        source_pos: 1,
        target_pos: 2,
        amount: 1
    };

    move_crates(&mut crates.stacks, &move_operation, false);

    assert_eq!(Vec::from([ VecDeque::from([str("A"), str("B")]), VecDeque::from([str("D"), str("C")])]), crates.stacks)
}

fn str(input: &'static str) -> String
{
    String::from(input)
}
