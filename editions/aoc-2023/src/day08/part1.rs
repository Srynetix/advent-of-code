//! Part 1

use super::{
    INPUT,
    common::{CamelMap, MapWalker},
};

pub fn run() -> usize {
    let map = CamelMap::from_input(INPUT);
    let walker = MapWalker::new();
    walker.steps_to_zzz(&map)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 11_567);
    }
}
