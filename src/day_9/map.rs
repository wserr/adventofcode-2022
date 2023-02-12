use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

pub struct Map(pub HashMap<Coordinate, usize>);

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
