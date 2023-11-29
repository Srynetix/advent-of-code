//! Part 2

use super::{
    common::{Polymer, PolymerSum},
    INPUT,
};

pub fn run() -> u64 {
    let polymer = Polymer::from(INPUT);
    let mut chain = PolymerSum::new(&polymer);
    chain.step_for(40);
    chain.get_common_score()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 4_302_675_529_689)
    }
}
