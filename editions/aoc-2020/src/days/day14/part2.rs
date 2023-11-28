//! Part 2

use super::{common::BitmaskMemory, INPUT};

pub fn run() -> usize {
    let mut mem = BitmaskMemory::new();
    for l in INPUT.lines() {
        mem.parse_line(l, true);
    }

    mem.get_memory_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2_667_858_637_669);
    }
}
