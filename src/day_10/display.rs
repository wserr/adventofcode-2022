use super::instructions::Cycles;
use std::fmt::Display;

impl Display for Cycles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_count: isize = 0;
        for i in self.0.iter() {
            let register_converted: isize = (*i.0).try_into().unwrap();

            let ray_position = *i.1;

            let ray = vec![ray_position, ray_position + 1, ray_position + 2];

            if ray.contains(&(register_converted - 40 * row_count)) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
            if i.0 % 40 == 0 {
                writeln!(f, "")?;
                row_count += 1;
            }
        }
        writeln!(f, "")
    }
}
