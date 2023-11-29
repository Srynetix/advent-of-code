//! Part 2

use super::{common::CrabSwarm, INPUT};

pub fn run() -> u32 {
    let swarm = CrabSwarm::from(INPUT);
    let (_pos, sum) = swarm.min_cost_for_alignment_with_sum();
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 98_363_777)
    }
}
