
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use bitvec::prelude::*;

struct Packet {
  version: u32,
  type_id: u32,
  value: Option<u64>,
  children: Vec<usize>,
}

// Clearly a generic would be better
// I don't know if I just don't get it, but maybe rust doesn't have SFINAE?
fn bslice_to_int(bv: &BitSlice::<Lsb0, u8>) -> u32 {
  let mut int = 0;
  for bit in bv {
    int = int << 1;
    if bit == true {
      int += 1;
    }
  }
  return int;
}

fn bv_to_int(bv: &BitVec::<Lsb0, u8>) -> u64 {
  let mut int = 0;
  for bit in bv {
    int = int << 1;
    if bit == true {
      int += 1;
    }
  }
  return int;
}


fn recurse_packets(bits: &BitVec::<Lsb0, u8>, mut bit_ptr: usize, mut packets: &mut Vec<Packet>) -> usize {
  let version = bslice_to_int(&bits[bit_ptr..(bit_ptr + 3)]);
  bit_ptr += 3;
  let type_id = bslice_to_int(&bits[bit_ptr..(bit_ptr + 3)]);
  bit_ptr += 3;
  let mut children: Vec::<usize> = Vec::new();
  let mut value: Option<u64> = None;
  if type_id == 4 {
    let mut value_bv: BitVec::<Lsb0, u8> = BitVec::new();
    loop {
      let mut sub_value = BitVec::from_bitslice(&bits[(bit_ptr + 1)..(bit_ptr + 5)]);
      value_bv.append(&mut sub_value);
      bit_ptr += 5;
      if !bits[bit_ptr - 5] {
        break;
      }
    }
    value = Some(bv_to_int(&value_bv));
  } else {
    if bits[bit_ptr] {
      bit_ptr += 1;
      // 11 bits with count of subpackets
      let num_packets = bslice_to_int(&bits[(bit_ptr)..(bit_ptr + 11)]);
      bit_ptr += 11;
      for _packet in 0..num_packets {
        bit_ptr = recurse_packets(&bits, bit_ptr, &mut packets);
        children.push(packets.len() - 1);
      }
    } else {
      bit_ptr += 1;
      // 15 bits with total length of subpackets
      let subpacket_len = bslice_to_int(&bits[(bit_ptr)..(bit_ptr + 15)]) as usize;
      bit_ptr += 15;
      let subpacket_end = subpacket_len + bit_ptr;
      while bit_ptr < subpacket_end {
        bit_ptr = recurse_packets(&bits, bit_ptr, &mut packets);
        children.push(packets.len() - 1);
      }
    }
  }
  packets.push(Packet{version, type_id, value, children});

  return bit_ptr;
}

fn parse_packets(bits: &BitVec::<Lsb0, u8>) -> Vec<Packet> {
  let mut packets = Vec::new();
  let bit_ptr = 0;
  recurse_packets(bits, bit_ptr, &mut packets);
  return packets;
}

fn run_operations(packets: &Vec<Packet>, packet_idx: usize) -> u64 {
  let mut value: u64;
  let current_packet = &packets[packet_idx];
  match current_packet.type_id {
    0 => {
      value = 0;
      for child_idx in &current_packet.children {
        value += run_operations(&packets, *child_idx);
      }
    },
    1 => {
      value = 1;
      for child_idx in &current_packet.children {
        value *= run_operations(&packets, *child_idx);
      }
    },
    2 => {
      value = u64::MAX;
      for child_idx in &current_packet.children {
        let subpacket_val = run_operations(&packets, *child_idx);
        value = if subpacket_val < value {subpacket_val} else {value};
      }
    },
    3 => {
      value = u64::MIN;
      for child_idx in &current_packet.children {
        let subpacket_val = run_operations(&packets, *child_idx);
        value = if subpacket_val > value {subpacket_val} else {value};
      }
    },
    4 => {
      value = packets[packet_idx].value.unwrap();
    },
    5 | 6 | 7 => {
      let subpacket_val1 = run_operations(&packets, current_packet.children[0]);
      let subpacket_val2 = run_operations(&packets, current_packet.children[1]);
      value = match current_packet.type_id {
        5 => if subpacket_val1 > subpacket_val2 {1} else {0},
        6 => if subpacket_val1 < subpacket_val2 {1} else {0},
        _ => if subpacket_val1 == subpacket_val2 {1} else {0}
      }
    },
    _ => {
      panic!("Error in packet tree structure");
    }
  }
  return value;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let mut f = BufReader::new(f);

  // Read hex
  let hex_map: [BitVec::<Lsb0, u64>; 16] = [
    bitvec![Lsb0, u64; 0,0,0,0],
    bitvec![Lsb0, u64; 0,0,0,1],
    bitvec![Lsb0, u64; 0,0,1,0],
    bitvec![Lsb0, u64; 0,0,1,1],
    bitvec![Lsb0, u64; 0,1,0,0],
    bitvec![Lsb0, u64; 0,1,0,1],
    bitvec![Lsb0, u64; 0,1,1,0],
    bitvec![Lsb0, u64; 0,1,1,1],
    bitvec![Lsb0, u64; 1,0,0,0],
    bitvec![Lsb0, u64; 1,0,0,1],
    bitvec![Lsb0, u64; 1,0,1,0],
    bitvec![Lsb0, u64; 1,0,1,1],
    bitvec![Lsb0, u64; 1,1,0,0],
    bitvec![Lsb0, u64; 1,1,0,1],
    bitvec![Lsb0, u64; 1,1,1,0],
    bitvec![Lsb0, u64; 1,1,1,1],
  ];

  let mut bits: BitVec::<Lsb0, u8> = BitVec::new();
  let mut hex_string = String::new();
  f.read_line(&mut hex_string).expect("Parse error");
  for hex_char in hex_string.trim().chars() {
    let four_bits: u32 = hex_char.to_digit(16).unwrap();
    bits.append(&mut hex_map[four_bits as usize].clone());
  }

  // Part 1
  let part1 = Instant::now();
  let packets = parse_packets(&bits);
  let version_sum: u32 = packets.iter().map(|x| x.version).sum();
  println!("Part 1: {}", version_sum);

  let part2 = Instant::now();
  println!("Part 2: {}", run_operations(&packets, packets.len() - 1));

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}
