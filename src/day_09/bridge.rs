use crate::day_09::map::{Coordinate, Map};
use std::collections::HashMap;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Instruction {
    pub direction: Direction,
    pub quantity: isize,
}

pub fn calculate_tail_positions(
    instructions: &Vec<Instruction>,
    amount_of_tail_pieces: usize,
) -> Map {
    let mut map: Map = Map(HashMap::new());
    let mut head = Coordinate { x: 0, y: 0 };

    let mut tails: Vec<Coordinate> = Vec::new();

    let mut index = amount_of_tail_pieces;

    while index > 0 {
        tails.push(Coordinate { x: 0, y: 0 });
        index -= 1;
    }

    for instr in instructions {
        let mut amount = instr.quantity;
        while amount > 0 {
            head = calculate_head_movement(&head, &instr.direction);
            tails = tails.into_iter().fold(Vec::new(), |mut acc, x| {
                if acc.len() == 0 {
                    acc.push(calculate_tail_movement(&head, &x));
                } else {
                    let prev_element = acc.iter().last().unwrap();
                    acc.push(calculate_tail_movement(prev_element, &x));
                }
                acc
            });
            map.0
                .entry(tails.last().unwrap().clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
            amount -= 1;
        }
    }
    println!("{}", map);
    map
}

pub fn calculate_amount_of_tail_positions(
    instructions: &Vec<Instruction>,
    amount_of_tail_pieces: usize,
) -> usize {
    calculate_tail_positions(instructions, amount_of_tail_pieces)
        .0
        .len()
}

fn calculate_head_movement(current_head: &Coordinate, direction: &Direction) -> Coordinate {
    let mut modifier_x: isize = 0;
    let mut modifier_y: isize = 0;

    match direction {
        Direction::Up => modifier_y -= 1,
        Direction::Down => modifier_y += 1,
        Direction::Left => modifier_x -= 1,
        Direction::Right => modifier_x += 1,
    };

    Coordinate {
        x: current_head.x + modifier_x,
        y: current_head.y + modifier_y,
    }
}

pub fn calculate_tail_movement(current_head: &Coordinate, current_tail: &Coordinate) -> Coordinate {
    if are_adjacent(current_head, current_tail) {
        return Coordinate {
            x: current_tail.x,
            y: current_tail.y,
        };
    }

    let diff_x = current_head.x - current_tail.x;
    let diff_y = current_head.y - current_tail.y;

    let mut modifier_x: isize = 0;
    let mut modifier_y: isize = 0;

    if diff_x == 0 && diff_y.abs() == 2 {
        // Horizontal movement
        modifier_y = (current_head.y - current_tail.y) / 2;
    } else if diff_y == 0 && diff_x.abs() == 2 {
        // Vertical movement
        modifier_x = (current_head.x - current_tail.x) / 2;
    } else {
        let calc_diag_mov = |x: isize| match x.abs() {
            2 => x / 2,
            _ => x,
        };
        // Diagonal movement
        modifier_y = calc_diag_mov(diff_y);
        modifier_x = calc_diag_mov(diff_x);
    }

    Coordinate {
        x: current_tail.x + modifier_x,
        y: current_tail.y + modifier_y,
    }
}

fn are_adjacent(current_head: &Coordinate, current_tail: &Coordinate) -> bool {
    let diff_x = current_head.x - current_tail.x;
    let diff_y = current_head.y - current_tail.y;

    diff_x.abs() <= 1 && diff_y.abs() <= 1
}

#[test]
fn are_adjacent_should_be_true() {
    let head = Coordinate { x: 1, y: 1 };

    let tail = Coordinate { x: 2, y: 1 };
    assert_eq!(true, are_adjacent(&head, &tail));
}

#[test]
fn are_adjacent_should_be_false() {
    let head = Coordinate { x: 1, y: 1 };

    let tail = Coordinate { x: 3, y: 1 };
    assert_eq!(false, are_adjacent(&head, &tail));
}

#[test]
fn calculate_head_movement_should_calc_vert() {
    let head = Coordinate { x: 1, y: 1 };

    assert_eq!(
        Coordinate { x: 1, y: 0 },
        calculate_head_movement(&head, &Direction::Up)
    );
}

#[test]
fn calculate_head_movement_should_calc_hor() {
    let head = Coordinate { x: 1, y: 1 };
    assert_eq!(
        Coordinate { x: 2, y: 1 },
        calculate_head_movement(&head, &Direction::Right)
    );
}

#[test]
fn calculate_head_movement_should_calc() {
    let head = Coordinate { x: 1, y: 1 };

    assert_eq!(
        Coordinate { x: 1, y: 0 },
        calculate_head_movement(&head, &Direction::Up)
    );
}

#[test]
fn calculate_tail_movement_should_calc_vert() {
    let head = Coordinate { x: 1, y: 1 };
    let tail = Coordinate { x: 1, y: 3 };

    assert_eq!(
        Coordinate { x: 1, y: 2 },
        calculate_tail_movement(&head, &tail)
    );
}
