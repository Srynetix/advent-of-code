//! Part 1

use super::{common::Sorter, INPUT};

pub fn run() -> usize {
    let (mut v1, mut v2) = Sorter::from_input(INPUT);
    Sorter::sort_lists(&mut v1, &mut v2);
    Sorter::sum_distances(Sorter::distances(&v1, &v2)) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2_285_373)
    }
}
