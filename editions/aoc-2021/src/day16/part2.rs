//! Part 2

use super::{
    common::{BinUtils, Packet},
    INPUT,
};

pub fn run() -> usize {
    let mut bin = BinUtils::bin_from_hex_string(INPUT.trim());
    Packet::from_bin(&mut bin).get_value()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 194_435_634_456)
    }
}
