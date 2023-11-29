//! Common

use std::collections::HashMap;

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

static RGX_MASK: Lazy<Regex> = Lazy::new(|| Regex::new(r"mask = (?P<mask>[01X]{36})").unwrap());

static RGX_MEM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mem\[(?P<mem_idx>\d+)\] = (?P<mem_value>\d+)").unwrap());

/// Bitmask memory
#[derive(Debug, Default)]
pub struct BitmaskMemory {
    current_mask: Vec<Option<bool>>,
    memory: HashMap<usize, usize>,
}

impl BitmaskMemory {
    /// Creates a new bitmask memory.
    pub fn new() -> Self {
        Self {
            current_mask: vec![None; 36],
            memory: HashMap::new(),
        }
    }

    /// Set mask from str.
    ///
    /// # Arguments
    ///
    /// * `mask` - Mask
    pub fn set_mask_from_str(&mut self, mask: &str) {
        if mask.len() != 36 {
            panic!("Mask str should be 36 characters");
        }

        for (idx, c) in mask.chars().enumerate() {
            match c {
                'X' => self.current_mask[idx] = None,
                '0' => self.current_mask[idx] = Some(false),
                '1' => self.current_mask[idx] = Some(true),
                _ => panic!("Invalid mask character: {}", c),
            }
        }

        // Reverse mask
        self.current_mask.reverse();
    }

    /// Set value in memory using value mask.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    /// * `value` - Value
    pub fn set_value_in_memory_using_value_mask(&mut self, addr: usize, value: usize) -> usize {
        let output = Self::apply_mask_on_value(&self.current_mask, value);
        self.memory.insert(addr, output);
        output
    }

    /// Set value in memory using value mask.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    /// * `value` - Value
    pub fn set_value_in_memory_using_address_mask(&mut self, addr: usize, value: usize) {
        let addresses = Self::apply_mask_on_address(&self.current_mask, addr);
        for addr in addresses {
            self.memory.insert(addr, value);
        }
    }

    /// Get memory sum.
    pub fn get_memory_sum(&self) -> usize {
        self.memory.values().filter(|&&x| x != 0).sum()
    }

    /// Parse an input line.
    ///
    /// # Arguments
    ///
    /// * `input` - Input line
    /// * `use_address_mask` - Use address mask
    pub fn parse_line(&mut self, input: &str, use_address_mask: bool) {
        if let Some(captures) = RGX_MASK.captures(input.trim()) {
            let mask = captures.name("mask").unwrap().as_str();
            self.set_mask_from_str(mask);
        } else {
            let captures = RGX_MEM.captures(input.trim()).unwrap();
            let mem_idx = captures
                .name("mem_idx")
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .unwrap();
            let mem_value = captures
                .name("mem_value")
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .unwrap();
            if use_address_mask {
                self.set_value_in_memory_using_address_mask(mem_idx, mem_value);
            } else {
                self.set_value_in_memory_using_value_mask(mem_idx, mem_value);
            }
        }
    }

    fn apply_mask_on_value(mask: &[Option<bool>], value: usize) -> usize {
        let mut output = value;
        for (idx, x) in mask.iter().enumerate() {
            match x {
                Some(true) => output |= 1 << idx,
                Some(false) => output &= !(1 << idx),
                None => (),
            }
        }

        output
    }

    fn apply_mask_on_address(mask: &[Option<bool>], addr: usize) -> Vec<usize> {
        let mut addresses = vec![];
        let mut first_value = addr;

        for (idx, x) in mask.iter().enumerate() {
            match x {
                Some(true) => first_value |= 1 << idx,
                Some(false) => (),
                None => first_value &= !(1 << idx),
            }
        }
        addresses.push(first_value);

        let bits: Vec<usize> = mask
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if x.is_none() { Some(idx) } else { None })
            .collect();
        for num in Self::generate_numbers_for_bits(&bits) {
            addresses.push(first_value + num);
        }

        addresses
    }

    #[allow(clippy::cast_possible_truncation)]
    fn generate_numbers_for_bits(bits: &[usize]) -> Vec<usize> {
        let mut output = vec![];

        let max_v = 2_u64.pow(bits.len() as u32);
        for i in 1..max_v {
            let mut number = 0;
            let bin = format!("{:b}", i);
            for (idx, n) in bin.chars().rev().enumerate() {
                if n == '1' {
                    number += 2_u64.pow(bits[idx] as u32);
                }
            }

            output.push(number as usize);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0"#;

    const SAMPLE_2: &str = r#"mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1"#;

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn test_sample() {
        let mut memory = BitmaskMemory::new();

        let mut lines = SAMPLE.lines();
        let mask = lines.next().unwrap();

        memory.parse_line(mask, false);
        assert_eq!(memory.current_mask[0], None);
        assert_eq!(memory.current_mask[1], Some(false));
        assert_eq!(memory.current_mask[6], Some(true));

        let mem8 = lines.next().unwrap();
        memory.parse_line(mem8, false);
        assert_eq!(memory.memory[&8], 73);

        let mem7 = lines.next().unwrap();
        memory.parse_line(mem7, false);
        assert_eq!(memory.memory[&7], 101);

        let mem8 = lines.next().unwrap();
        memory.parse_line(mem8, false);
        assert_eq!(memory.memory[&8], 64);

        assert_eq!(memory.get_memory_sum(), 165);
    }

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn test_sample_2() {
        let mut memory = BitmaskMemory::new();

        let mut lines = SAMPLE_2.lines();

        // Parse first mask
        memory.parse_line(lines.next().unwrap(), true);
        memory.parse_line(lines.next().unwrap(), true);
        for x in &[26, 27, 58, 59] {
            assert_eq!(memory.memory[x], 100);
        }

        // Parse second mask
        memory.parse_line(lines.next().unwrap(), true);
        memory.parse_line(lines.next().unwrap(), true);
        for x in &[16, 17, 18, 19, 24, 25, 26, 27] {
            assert_eq!(memory.memory[x], 1);
        }

        assert_eq!(memory.get_memory_sum(), 208);
    }
}
