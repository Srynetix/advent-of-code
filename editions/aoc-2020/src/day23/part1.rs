//! Part 1

use super::{
    INPUT,
    common::{parse_cups, run_steps},
};

pub fn run() -> String {
    let mut cups = parse_cups(INPUT);
    run_steps(&mut cups, 100);
    cups.to_string_from_one()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), "27865934")
    }
}
