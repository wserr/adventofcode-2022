pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn calculate_amount_of_visible_trees(field: &Vec<Vec<usize>>) -> usize {
    let result = calculate_result_matrix(field);
    result.iter().fold(0, |mut a, x| {
        a += x.iter().fold(0, |y, z| if *z { y + 1 } else { y });
        a
    })
}

pub fn calculate_max_scenic_score(field: &Vec<Vec<usize>>) -> usize {
    let result = calculate_result_scenic_score_matrix(field);
    println!("{:?}", result);
    *result
        .iter()
        .map(|x| x.iter().max().unwrap())
        .max()
        .unwrap()
}

fn calculate_result_matrix(field: &Vec<Vec<usize>>) -> Vec<Vec<bool>> {
    let new_matrix = field.clone();
    let mut current_y = 0;
    let mut current_x = 0;
    new_matrix
        .into_iter()
        .map(|x| {
            let result = x
                .into_iter()
                .map(|y| {
                    let result = visible_in_any_direction(field, current_y, current_x);
                    current_x += 1;
                    result
                })
                .collect();
            current_y += 1;
            current_x = 0;
            result
        })
        .collect()
}

fn calculate_result_scenic_score_matrix(field: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let new_matrix = field.clone();
    let mut current_y = 0;
    let mut current_x = 0;
    new_matrix
        .into_iter()
        .map(|x| {
            let result = x
                .into_iter()
                .map(|y| {
                    let result = scenic_score_in_any_direction(field, current_y, current_x);
                    current_x += 1;
                    result
                })
                .collect();
            current_y += 1;
            current_x = 0;
            result
        })
        .collect()
}

fn visible_in_any_direction(field: &Vec<Vec<usize>>, current_x: usize, current_y: usize) -> bool {
    is_visible_in_direction(field, current_y, current_x, &Direction::Up)
        || is_visible_in_direction(field, current_y, current_x, &Direction::Down)
        || is_visible_in_direction(field, current_y, current_x, &Direction::Left)
        || is_visible_in_direction(field, current_y, current_x, &Direction::Right)
}

fn scenic_score_in_any_direction(
    field: &Vec<Vec<usize>>,
    current_x: usize,
    current_y: usize,
) -> usize {
    scenic_score_in_direction(field, current_y, current_x, &Direction::Up)
        * scenic_score_in_direction(field, current_y, current_x, &Direction::Down)
        * scenic_score_in_direction(field, current_y, current_x, &Direction::Left)
        * scenic_score_in_direction(field, current_y, current_x, &Direction::Right)
}

fn is_visible_in_direction(
    field: &Vec<Vec<usize>>,
    current_x: usize,
    current_y: usize,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Up => visible_in_vertical_direction(field, current_y, current_x, true),
        Direction::Down => visible_in_vertical_direction(field, current_y, current_x, false),
        Direction::Left => visible_in_horizontal_direction(field, current_y, current_x, true),
        Direction::Right => visible_in_horizontal_direction(field, current_y, current_x, false),
    }
}

fn scenic_score_in_direction(
    field: &Vec<Vec<usize>>,
    current_x: usize,
    current_y: usize,
    direction: &Direction,
) -> usize {
    match direction {
        Direction::Up => scenic_score_in_vertical_direction(field, current_y, current_x, true),
        Direction::Down => scenic_score_in_vertical_direction(field, current_y, current_x, false),
        Direction::Left => scenic_score_in_horizontal_direction(field, current_y, current_x, true),
        Direction::Right => {
            scenic_score_in_horizontal_direction(field, current_y, current_x, false)
        }
    }
}

fn scenic_score_in_vertical_direction(
    field: &Vec<Vec<usize>>,
    current_y: usize,
    current_x: usize,
    is_up: bool,
) -> usize {
    let current_height = field[current_y][current_x];

    let limit_reached = |x| match is_up {
        true => x == 0,
        false => x == (field.len() - 1),
    };
    let calc_new_y = |x| {
        if is_up && !limit_reached(x) {
            x - 1
        } else if !limit_reached(x) {
            x + 1
        } else {
            x
        }
    };

    let mut new_y = current_y;
    let mut index = 0;

    while !limit_reached(new_y) {
        new_y = calc_new_y(new_y);
        index += 1;
        let new_height = field[new_y][current_x];
        if new_height >= current_height {
            return index;
        }
    }
    return index;
}

fn visible_in_vertical_direction(
    field: &Vec<Vec<usize>>,
    current_y: usize,
    current_x: usize,
    is_up: bool,
) -> bool {
    let current_height = field[current_y][current_x];

    let limit_reached = |x| match is_up {
        true => x == 0,
        false => x == (field.len() - 1),
    };
    let calc_new_y = |x| {
        if is_up && !limit_reached(x) {
            x - 1
        } else if !limit_reached(x) {
            x + 1
        } else {
            x
        }
    };

    let mut new_y = current_y;

    while !limit_reached(new_y) {
        new_y = calc_new_y(new_y);
        let new_height = field[new_y][current_x];
        if new_height >= current_height {
            return false;
        }
    }
    return true;
}

fn visible_in_horizontal_direction(
    field: &Vec<Vec<usize>>,
    current_y: usize,
    current_x: usize,
    is_left: bool,
) -> bool {
    let current_height = field[current_y][current_x];
    let limit_reached = |x| match is_left {
        true => x == 0,
        false => x == field[0].len() - 1,
    };
    let calc_new_x = |x| {
        if is_left && !limit_reached(x) {
            x - 1
        } else if !limit_reached(x) {
            x + 1
        } else {
            x
        }
    };

    let mut new_x = current_x;

    while !limit_reached(new_x) {
        new_x = calc_new_x(new_x);
        let new_height = field[current_y][new_x];
        if new_height >= current_height {
            return false;
        }
    }
    return true;
}

fn scenic_score_in_horizontal_direction(
    field: &Vec<Vec<usize>>,
    current_y: usize,
    current_x: usize,
    is_left: bool,
) -> usize {
    let current_height = field[current_y][current_x];
    let limit_reached = |x| match is_left {
        true => x == 0,
        false => x == field[0].len() - 1,
    };
    let calc_new_x = |x| {
        if is_left && !limit_reached(x) {
            x - 1
        } else if !limit_reached(x) {
            x + 1
        } else {
            x
        }
    };

    let mut new_x = current_x;
    let mut index = 0;

    while !limit_reached(new_x) {
        new_x = calc_new_x(new_x);
        index += 1;
        let new_height = field[current_y][new_x];
        if new_height >= current_height {
            return index;
        }
    }
    return index;
}

#[test]
fn calculate_visible_trees_should_eq_2() {
    let mut a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    a[4][5] = 10;
    a[6][7] = 10;

    assert_eq!(38, calculate_amount_of_visible_trees(&a));
}

#[test]
fn should_not_be_visible_in_any_direction() {
    let a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];

    assert_eq!(false, visible_in_any_direction(&a, 4, 5));
}

#[test]
fn should_be_visible_in_any_direction() {
    let mut a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    a[4][5] = 10;

    assert_eq!(true, visible_in_any_direction(&a, 4, 5));
}

#[test]
fn should_be_visible_in_vertical_direction() {
    let mut a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    a[4][5] = 10;

    assert_eq!(true, visible_in_vertical_direction(&a, 4, 5, true));
    assert_eq!(true, visible_in_vertical_direction(&a, 4, 5, false));
    assert_eq!(false, visible_in_any_direction(&a, 1, 5));
}

#[test]
fn should_not_be_visible_in_vertical_direction() {
    let a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];

    assert_eq!(false, visible_in_vertical_direction(&a, 4, 5, true));
    assert_eq!(false, visible_in_vertical_direction(&a, 4, 5, false));
}

#[test]
fn should_be_visible_in_horizontal_direction() {
    let mut a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    a[4][5] = 10;

    assert_eq!(true, visible_in_horizontal_direction(&a, 4, 5, true));
    assert_eq!(true, visible_in_horizontal_direction(&a, 4, 5, false));
}

#[test]
fn should_not_be_visible_in_horizontal_direction() {
    let a: Vec<Vec<usize>> = vec![vec![0; 10]; 10];

    assert_eq!(false, visible_in_horizontal_direction(&a, 4, 5, true));
    assert_eq!(false, visible_in_horizontal_direction(&a, 4, 5, false));
}

// This test is just to play around with 2D arrays
#[test]
fn should_create_and_traverse_field() {
    let array = vec![vec![0; 10]; 10];
    let mut sum: usize = 0;

    let new_array = array.clone();

    let new_array_2: Vec<Vec<i32>> = new_array
        .into_iter()
        .map(|x| x.into_iter().map(|y| y + 1).collect())
        .collect();

    for el in new_array_2.iter() {
        for el_2 in el.iter() {
            if *el_2 == 1 {
                sum += 1;
            }
        }
    }

    assert_eq!(100, sum);
}
