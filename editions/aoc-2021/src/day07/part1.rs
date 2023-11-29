//! Part 1

use super::{common::CrabSwarm, INPUT};

pub fn run() -> u32 {
    let swarm = CrabSwarm::from(INPUT);
    let (_pos, sum) = swarm.min_cost_for_alignment();
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 347_011)
    }
}
