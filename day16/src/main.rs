use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let bits: Vec<String> = lines.into_iter().next()
        .expect("Could not get input")
        .chars()
        .map(|hex_char| hex_char.to_digit(16).expect(format!("Could not parse to digit: {0:?}", hex_char).as_str()))
        .map(|hex_digit| format!("{0:04b}", hex_digit))
        .collect();

    let bits: String = bits.iter()
        .flat_map(|binary_string| binary_string.chars())
        .collect();
    let mut bits = bits.as_str();
    // println!("bits={0:?}", bits);

    let mut packets = Vec::new();

    while !bits.is_empty() {
        if bits.chars().all(|bit| bit == '0') {
            break;
        }

        let (packet, remainder) = Day16::parse_packet(&bits);
        bits = remainder;

        packets.push(packet);
    }
    // println!("packets={0:#?}", packets);

    let versions_sum: u64 = packets.iter().map(|packet| packet.sum_versions()).sum();
    println!("part1: versions_sum={0:?}", versions_sum);
    assert_eq!(versions_sum, 873);

    let result: Vec<u64> = packets.iter().map(|packet| packet.process()).collect();
    println!("part2: result={0:?}", result);
    // assert_eq!(versions_sum, 873);
}

#[derive(Debug)]
enum PacketType {
    LiteralValue(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u32,
    packet_type: PacketType,
}

struct Day16;

impl Day16 {
    fn parse_packet(bits: &str) -> (Packet, &str) {
        let (version, bits) = bits.split_at(3);
        let version = u32::from_str_radix(version, 2).expect(format!("Could not parse version: {0:?}", version).as_str());

        let (type_id, bits) = bits.split_at(3);
        let type_id = u32::from_str_radix(type_id, 2).expect(format!("Could not parse type_id: {0:?}", type_id).as_str());

        let (packet_type, bits) = match type_id {
            0 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::Sum(subpackets), bits)
            }
            1 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::Product(subpackets), bits)
            }
            2 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::Minimum(subpackets), bits)
            }
            3 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::Maximum(subpackets), bits)
            }
            4 => Self::parse_literal_value(bits),
            5 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::GreaterThan(subpackets), bits)
            }
            6 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::LessThan(subpackets), bits)
            }
            7 => {
                let (subpackets, bits) = Self::parse_operator(bits);
                (PacketType::EqualTo(subpackets), bits)
            }
            _ => panic!("Unknown type_id={0:?}", type_id),
        };

        (
            Packet {
                version,
                packet_type,
            },
            bits,
        )
    }

    fn parse_literal_value(bits: &str) -> (PacketType, &str) {
        let mut bits = bits;

        let mut nibbles: Vec<&str> = Vec::new();

        loop {
            let (nibble, remainder) = bits.split_at(5);
            bits = remainder;

            let (marker, nibble) = nibble.split_at(1);

            nibbles.push(nibble);

            if marker == "0" {
                break;
            }
        }

        let literal_value = String::from_iter(nibbles.into_iter());
        let literal_value = u64::from_str_radix(&literal_value, 2).expect(format!("Could not parse literal value: {0:?}", literal_value).as_str());

        (
            PacketType::LiteralValue(literal_value),
            bits
        )
    }

    fn parse_operator(bits: &str) -> (Vec<Packet>, &str) {
        let mut subpackets = Vec::new();

        let (length_type_id, mut bits) = bits.split_at(1);
        let length_type_id = u32::from_str_radix(&length_type_id, 2).expect(format!("Could not parse length_type_id: {0:?}", length_type_id).as_str());

        match length_type_id {
            0 => {
                let (subpacket_length, remainder) = bits.split_at(15);
                bits = remainder;

                let subpacket_length = usize::from_str_radix(&subpacket_length, 2).expect(format!("Could not parse subpacket_length: {0:?}", subpacket_length).as_str());

                let (mut subpacket_bits, remainder) = bits.split_at(subpacket_length);
                bits = remainder;

                while !subpacket_bits.is_empty() {
                    let (packet, remainder) = Self::parse_packet(subpacket_bits);
                    subpackets.push(packet);
                    subpacket_bits = remainder;
                }
            }
            1 => {
                let (subpacket_count, remainder) = bits.split_at(11);
                bits = remainder;

                let subpacket_count = u32::from_str_radix(&subpacket_count, 2).expect(format!("Could not parse subpacket_count: {0:?}", subpacket_count).as_str());

                for _i in 0..subpacket_count {
                    let (packet, remainder) = Self::parse_packet(bits);
                    subpackets.push(packet);
                    bits = remainder;
                }
            }
            _ => panic!("Unknown length_type_id={0:?}", length_type_id),
        }

        (
            subpackets,
            bits,
        )
    }
}

impl Packet {
    fn sum_versions(&self) -> u64 {
        self.version as u64
            + match &self.packet_type {
            PacketType::Sum(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            PacketType::Product(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            PacketType::Minimum(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            PacketType::Maximum(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            PacketType::GreaterThan(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            PacketType::LessThan(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            PacketType::EqualTo(subpackets) => subpackets.iter().map(|subpacket| subpacket.sum_versions()).sum(),
            _ => 0,
        } as u64
    }

    fn process(&self) -> u64 {
        match &self.packet_type {
            PacketType::LiteralValue(value) => *value,
            PacketType::Sum(subpackets) => subpackets.iter().map(|subpacket| subpacket.process()).sum(),
            PacketType::Product(subpackets) => subpackets.iter().map(|subpacket| subpacket.process()).fold(1, |acc, current| acc * current),
            PacketType::Minimum(subpackets) => subpackets.iter().map(|subpacket| subpacket.process()).min().unwrap(),
            PacketType::Maximum(subpackets) => subpackets.iter().map(|subpacket| subpacket.process()).max().unwrap(),
            PacketType::GreaterThan(subpackets) => if subpackets[0].process() > subpackets[1].process() { 1 } else { 0 },
            PacketType::LessThan(subpackets) => if subpackets[0].process() < subpackets[1].process() { 1 } else { 0 },
            PacketType::EqualTo(subpackets) => if subpackets[0].process() == subpackets[1].process() { 1 } else { 0 },
        }
    }
}
