use std::{collections::HashMap, fmt::Display};

pub struct Map(HashMap<Coordinate, usize>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.0.iter().map(|x| x.0.x).min().unwrap();
        let min_y = self.0.iter().map(|y| y.0.y).min().unwrap();

        let max_x = self.0.iter().map(|x| x.0.x).max().unwrap();
        let max_y = self.0.iter().map(|y| y.0.y).max().unwrap();

        let x_modifier = -min_x;
        let y_modifier = -min_y;

        let x_size: usize = (max_x + x_modifier + 1).try_into().unwrap();
        let y_size: usize = (max_y + y_modifier + 1).try_into().unwrap();

        let mut a: Vec<Vec<char>> = vec![vec!['.'; x_size]; y_size];

        for item in self.0.iter() {
            let c = item.0;
            let x: usize = (c.x + x_modifier).try_into().unwrap();
            let y: usize = (c.y + y_modifier).try_into().unwrap();
            a[y][x] = '#';
        }

        for line in a {
            for item in line.iter() {
                write!(f, "{}", item)?
            }
            writeln!(f, "")?
        }

        write!(f, "")
    }
}

pub fn calculate_tail_positions(instructions: &Vec<Instruction>) -> Map {
    let mut map: Map = Map(HashMap::new());
    let mut head = Coordinate { x: 0, y: 0 };

    let mut tail = Coordinate { x: 0, y: 0 };

    map.0.insert(tail.clone(), 1);

    for instr in instructions {
        let mut amount = instr.quantity;
        while amount > 0 {
            head = calculate_head_movement(&head, &instr.direction);
            tail = calculate_tail_movement(&head, &tail);
            map.0
                .entry(tail.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
            amount -= 1;
        }
    }
    println!("{}", map);
    map
}

pub fn calculate_amount_of_tail_positions(instructions: &Vec<Instruction>) -> usize {
    calculate_tail_positions(instructions).0.len()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coordinate {
    x: isize,
    y: isize,
}

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
        // Diagonal movement
        modifier_y = (current_head.y - current_tail.y) / 2;
        modifier_x = (current_head.x - current_tail.x) / 2;
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
    let instruction = Instruction {
        direction: Direction::Up,
        quantity: 5,
    };

    assert_eq!(
        Coordinate { x: 1, y: -4 },
        calculate_head_movement(&head, &instruction)
    );
}

#[test]
fn calculate_head_movement_should_calc_hor() {
    let head = Coordinate { x: 1, y: 1 };
    let instruction = Instruction {
        direction: Direction::Right,
        quantity: 2,
    };
    assert_eq!(
        Coordinate { x: 3, y: 1 },
        calculate_head_movement(&head, &instruction)
    );
}

#[test]
fn calculate_head_movement_should_calc() {
    let head = Coordinate { x: 1, y: 1 };
    let instruction = Instruction {
        direction: Direction::Up,
        quantity: 5,
    };

    assert_eq!(
        Coordinate { x: 1, y: -4 },
        calculate_head_movement(&head, &instruction)
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
