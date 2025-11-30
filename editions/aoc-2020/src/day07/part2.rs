//! Part 2

use super::{
    INPUT,
    common::{BagColor, BagSystem, INPUT_COLOR_NAME},
};

pub fn run() -> usize {
    let system = BagSystem::new_from_rules(INPUT);
    let color: BagColor = INPUT_COLOR_NAME.into();
    system.count_needed_bags_for_color(&color)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 27526);
    }
}
