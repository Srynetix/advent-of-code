//! Part 1

use super::{
    INPUT,
    common::{BinUtils, Packet},
};

pub fn run() -> usize {
    let mut bin = BinUtils::bin_from_hex_string(INPUT.trim());
    Packet::from_bin(&mut bin).get_version_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 877)
    }
}
