//! Part 2

use super::{
    common::{compute_manhattan_distance, Ship},
    INPUT,
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
