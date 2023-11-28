//! Part 1

use super::{common::{BagSystem, INPUT_COLOR_NAME, BagColor}, INPUT};

pub fn run() -> usize {
    let system = BagSystem::new_from_rules(INPUT);
    let color: BagColor = INPUT_COLOR_NAME.into();
    system.find_container_colors_for_color(&color).len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 378);
    }
}
