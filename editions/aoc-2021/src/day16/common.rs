//! Common

use aoc_sx::itertools::Itertools;
use aoc_sx::tap::Pipe;

pub struct BinUtils;

impl BinUtils {
    pub fn slice_to_bin_str(slice: &[u8]) -> String {
        slice.iter().join("")
    }

    pub fn str_to_bin_num(s: &str) -> u64 {
        u64::from_str_radix(s, 2).unwrap()
    }

    pub fn slice_to_bin_num(slice: &[u8]) -> u64 {
        Self::str_to_bin_num(&Self::slice_to_bin_str(slice))
    }

    pub fn bin_from_hex_string(hex_str: &str) -> Vec<u8> {
        hex_str
            .chars()
            .map(|x| format!("{:04b}", x.to_digit(16).unwrap()))
            .join("")
            .chars()
            .map(|x| x.to_digit(2).unwrap() as u8)
            .collect::<Vec<_>>()
    }

    pub fn bin_to_u8(s: &[u8]) -> u8 {
        s.iter()
            .join("")
            .pipe(|s| u8::from_str_radix(&s, 2).unwrap())
    }
}

#[derive(PartialEq, Debug)]
pub struct Packet {
    version: u8,
    packet_type: PacketType,
}

#[derive(PartialEq, Debug)]
pub enum PacketType {
    Literal(u64),
    Operator(OperatorPacket),
}

#[derive(PartialEq, Debug)]
pub struct OperatorPacket {
    operator_type: OperatorType,
    length_type: OperatorLengthType,
    subpackets: Vec<Packet>,
}

#[derive(PartialEq, Debug)]
pub enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl OperatorType {
    fn from_type_id(type_id: u8) -> Self {
        match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum OperatorLengthType {
    TotalLength(u64),
    SubPacketsNumber(u64),
}

impl OperatorPacket {
    pub fn from_bin(type_id: u8, length_type_id: u8, bin_str: &mut Vec<u8>) -> Self {
        let length_type = match length_type_id {
            0 => {
                // Total length on 15 bits
                let length = bin_str.drain(..15).collect_vec();
                OperatorLengthType::TotalLength(BinUtils::slice_to_bin_num(&length))
            }
            _ => {
                // Subpackets count on 11 bits
                let count = bin_str.drain(..11).collect_vec();
                OperatorLengthType::SubPacketsNumber(BinUtils::slice_to_bin_num(&count))
            }
        };

        let subpackets = Self::parse_subpackets(&length_type, bin_str);

        Self {
            operator_type: OperatorType::from_type_id(type_id),
            length_type,
            subpackets,
        }
    }

    fn parse_subpackets(length_type: &OperatorLengthType, bin_str: &mut Vec<u8>) -> Vec<Packet> {
        match length_type {
            OperatorLengthType::TotalLength(l) => {
                Packet::from_bin_with_length(bin_str, *l as usize)
            }
            OperatorLengthType::SubPacketsNumber(n) => {
                Packet::from_bin_with_count(bin_str, *n as usize)
            }
        }
    }
}

impl PacketType {
    pub fn from_bin(packet_type_id: u8, bin_str: &mut Vec<u8>) -> Self {
        match packet_type_id {
            4 => {
                let mut bin_num = String::new();
                loop {
                    let chunk = bin_str.drain(..5).collect_vec();
                    if chunk[0] == 1 {
                        bin_num.push_str(&BinUtils::slice_to_bin_str(&chunk[1..]));
                    } else if chunk[0] == 0 {
                        bin_num.push_str(&BinUtils::slice_to_bin_str(&chunk[1..]));
                        break;
                    }
                }

                PacketType::Literal(BinUtils::str_to_bin_num(&bin_num))
            }
            o => {
                let length_mode = bin_str.remove(0);
                PacketType::Operator(OperatorPacket::from_bin(o, length_mode, bin_str))
            }
        }
    }
}

impl Packet {
    pub fn from_bin(bin_str: &mut Vec<u8>) -> Self {
        let version = bin_str
            .drain(..3)
            .collect_vec()
            .pipe(|x| BinUtils::bin_to_u8(&x));
        let packet_type_id = bin_str
            .drain(..3)
            .collect_vec()
            .pipe(|x| BinUtils::bin_to_u8(&x));
        let packet_type = PacketType::from_bin(packet_type_id, bin_str);

        Self {
            version,
            packet_type,
        }
    }

    pub fn from_bin_with_count(bin_str: &mut Vec<u8>, packet_count: usize) -> Vec<Self> {
        let mut output = vec![];

        for _ in 0..packet_count {
            output.push(Self::from_bin(bin_str));
        }

        output
    }

    pub fn from_bin_with_length(bin_str: &mut Vec<u8>, total_length: usize) -> Vec<Self> {
        let mut output = vec![];

        let mut read = 0;
        let mut prev_len = bin_str.len();

        while read < total_length {
            output.push(Self::from_bin(bin_str));

            let new_len = bin_str.len();
            read += prev_len - new_len;
            prev_len = new_len;
        }

        output
    }

    pub fn get_version_sum(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(_) => self.version as usize,
            PacketType::Operator(p) => {
                p.subpackets
                    .iter()
                    .map(|x| x.get_version_sum())
                    .sum::<usize>()
                    + self.version as usize
            }
        }
    }

    pub fn get_value(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(l) => *l as usize,
            PacketType::Operator(o) => {
                let subpackets_values = o.subpackets.iter().map(|x| x.get_value()).collect_vec();
                match o.operator_type {
                    OperatorType::Sum => subpackets_values.iter().sum(),
                    OperatorType::Product => subpackets_values.iter().product(),
                    OperatorType::Minimum => subpackets_values.iter().min().copied().unwrap(),
                    OperatorType::Maximum => subpackets_values.iter().max().copied().unwrap(),
                    OperatorType::GreaterThan => {
                        if subpackets_values[0] > subpackets_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorType::LessThan => {
                        if subpackets_values[0] < subpackets_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    OperatorType::EqualTo => {
                        if subpackets_values[0] == subpackets_values[1] {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::itertools::Itertools;
    use aoc_sx::tap::Pipe;

    use super::{BinUtils, OperatorLengthType, OperatorPacket, OperatorType, Packet, PacketType};

    fn to_bin_str(vec: &[u8]) -> String {
        vec.iter().join("")
    }

    #[test]
    fn test_bin_from_hex_string() {
        let bin = BinUtils::bin_from_hex_string("D2FE28");
        assert_eq!((&bin[..]).pipe(to_bin_str), "110100101111111000101000");
    }

    #[test]
    fn test_from_bin() {
        let mut bin = BinUtils::bin_from_hex_string("D2FE28");
        assert_eq!(
            Packet::from_bin(&mut bin),
            Packet {
                version: 6,
                packet_type: PacketType::Literal(2021)
            }
        );
    }

    #[test]
    fn test_operator_1() {
        let mut bin = BinUtils::bin_from_hex_string("38006F45291200");
        assert_eq!(
            Packet::from_bin(&mut bin),
            Packet {
                version: 1,
                packet_type: PacketType::Operator(OperatorPacket {
                    operator_type: OperatorType::LessThan,
                    length_type: OperatorLengthType::TotalLength(27),
                    subpackets: vec![
                        Packet {
                            version: 6,
                            packet_type: PacketType::Literal(10)
                        },
                        Packet {
                            version: 2,
                            packet_type: PacketType::Literal(20)
                        }
                    ]
                })
            }
        )
    }

    #[test]
    fn test_operator_2() {
        let mut bin = BinUtils::bin_from_hex_string("EE00D40C823060");
        assert_eq!(
            Packet::from_bin(&mut bin),
            Packet {
                version: 7,
                packet_type: PacketType::Operator(OperatorPacket {
                    operator_type: OperatorType::Maximum,
                    length_type: OperatorLengthType::SubPacketsNumber(3),
                    subpackets: vec![
                        Packet {
                            version: 2,
                            packet_type: PacketType::Literal(1)
                        },
                        Packet {
                            version: 4,
                            packet_type: PacketType::Literal(2)
                        },
                        Packet {
                            version: 1,
                            packet_type: PacketType::Literal(3)
                        }
                    ]
                })
            }
        )
    }

    #[test]
    fn test_version_sum() {
        fn test(hex_str: &str, target: usize) {
            let mut bin = BinUtils::bin_from_hex_string(hex_str);
            assert_eq!(Packet::from_bin(&mut bin).get_version_sum(), target);
        }

        test("8A004A801A8002F478", 16);
        test("620080001611562C8802118E34", 12);
        test("C0015000016115A2E0802F182340", 23);
        test("A0016C880162017C3686B18A3D4780", 31);
    }

    #[test]
    fn test_get_value() {
        fn test(hex_str: &str, target: usize) {
            let mut bin = BinUtils::bin_from_hex_string(hex_str);
            assert_eq!(Packet::from_bin(&mut bin).get_value(), target);
        }

        test("C200B40A82", 3);
        test("04005AC33890", 54);
        test("880086C3E88112", 7);
        test("CE00C43D881120", 9);
        test("D8005AC2A8F0", 1);
        test("F600BC2D8F", 0);
        test("9C005AC2F8F0", 0);
        test("9C0141080250320F1802104A08", 1);
    }
}
