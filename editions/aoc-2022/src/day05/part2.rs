//! Part 2

use super::{
    common::{CraneType, Ship},
    INPUT,
};

pub fn run() -> String {
    let mut ship = Ship::from_input(INPUT);
    ship.set_crane_type(CraneType::CrateMover9001);
    ship.apply_internal_procedures();
    ship.get_stack_tops_to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), "BRQWDBBJM")
    }
}
