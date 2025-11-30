//! Part 2

use super::{
    INPUT,
    common::{Ship, compute_manhattan_distance},
};

pub fn run() -> usize {
    compute_manhattan_distance(Ship::new().parse_and_execute_input_commands_waypoint(INPUT))
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 71586);
    }
}
