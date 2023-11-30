//! Part 1

use super::{common::Ship, INPUT};

pub fn run() -> String {
    let mut ship = Ship::from_input(INPUT);
    ship.apply_internal_procedures();
    ship.get_stack_tops_to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), "LJSVLTWQM")
    }
}
