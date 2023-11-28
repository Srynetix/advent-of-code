//! Part 1

use super::{common::BitmaskMemory, INPUT};

pub fn run() -> usize {
    let mut mem = BitmaskMemory::new();
    for l in INPUT.lines() {
        mem.parse_line(l, false);
    }

    mem.get_memory_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 8_471_403_462_063);
    }
}
