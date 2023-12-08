//! Part 2

use super::{
    common::{CamelMap, MapWalker},
    INPUT,
};

pub fn run() -> usize {
    let map = CamelMap::from_input(INPUT);
    let walker = MapWalker::new();
    walker.parallel_steps_to_zzz(&map)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 9_858_474_970_153)
    }
}
