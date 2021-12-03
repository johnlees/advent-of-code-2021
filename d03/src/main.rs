
use std::fs::File;
use std::io::{BufRead, BufReader};

use bitvec::prelude::*;

fn cut_candidates(candidate_vec: &Vec<BitVec>, col_idx: usize, common: bool) -> Vec<BitVec> {
  let mut on_cnt = 0;
  for candidate in candidate_vec {
    if candidate[col_idx] == true {
      on_cnt += 1;
    }
  }
  let mut keep = false;
  let bound = candidate_vec.len() / 2 + candidate_vec.len() % 2;
  if (common && on_cnt >= bound) ||
     (!common && on_cnt < bound) {
    keep = true;
  }
  let mut new_candidates: Vec<BitVec> = Vec::new();
  for candidate in candidate_vec {
    if candidate[col_idx] == keep {
      new_candidates.push(candidate.clone());
    }
  }
  return new_candidates;
}

fn bv_to_int(bv: &BitVec) -> u32 {
  let mut int = 0;
  for bit in bv {
    int = int << 1;
    if bit == true {
      int += 1;
    }
  }
  return int
}

fn main() {
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Build horizontal and vertical bitvecs
  let mut col_bits: Vec<BitVec> = Vec::new();
  let mut row_bits: Vec<BitVec> = Vec::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    if col_bits.len() == 0 {
      col_bits.resize(line.len(), BitVec::new());
    }
    let mut row: BitVec = BitVec::new();
    for (idx, bit) in line.chars().enumerate() {
      match bit {
        '0' => {
          col_bits[idx].push(false);
          row.push(false);
        },
        '1' => {
          col_bits[idx].push(true);
          row.push(true);
        }
        _ => panic!("Parse error")
      }
    }
    row_bits.push(row);
  }

  // Part 1
  let (mut gamma, mut epsilon) = (0, 0);
  for col in &col_bits {
    gamma = gamma << 1;
    epsilon = epsilon << 1;
    if col.count_ones() > col.len() / 2 {
      gamma += 1
    } else {
      epsilon += 1
    }
  }
  println!("Part 1: {}", gamma * epsilon);

  // Part 2
  let mut oxy_candidates = row_bits.clone();
  let mut co2_candidates = row_bits.clone();
  for col_idx in 0..col_bits.len() {
    if oxy_candidates.len() > 1 {
      oxy_candidates = cut_candidates(&oxy_candidates, col_idx, true);
    }
    if co2_candidates.len() > 1 {
      co2_candidates = cut_candidates(&co2_candidates, col_idx, false);
    }
  }
  let oxy = bv_to_int(&oxy_candidates[0]);
  let co2 = bv_to_int(&co2_candidates[0]);
  println!("Part 2: {}", oxy * co2);
}
