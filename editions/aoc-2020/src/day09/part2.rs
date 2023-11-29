//! Part 2

use super::{common::XmasScanner, INPUT};

pub fn run() -> usize {
    let target = super::part1::run();
    XmasScanner::find_weakness(INPUT, target)
        .map(|x| x.get_sum())
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 76_096_372);
    }
}
