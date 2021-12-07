
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::cmp;

use nalgebra::{DVector};

fn fuel_used(pos: i32, destination: i32) -> i32 {
  let dist = (pos - destination).abs();
  return (dist * (dist + 1)) >> 1;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let mut f = BufReader::new(f);

  // Read crabs
  let mut crabs: Vec::<i32> = Vec::new();
  let mut init_crabs = String::new();
  f.read_line(&mut init_crabs).expect("Parse error");
  for crab in init_crabs.trim().split(',') {
    let crab_pos: i32 = crab.parse().unwrap();
    crabs.push(crab_pos);
  }

  // Part 1
  let part1 = Instant::now();
  crabs.sort();
  let median = crabs[crabs.len() / 2];
  let crab_vec: DVector::<i32> = DVector::from_vec(crabs.clone());
  let mut fuel = crab_vec.add_scalar(-median).abs().sum();
  println!("Part 1: {}", fuel);

  let part2 = Instant::now();
  let candidate_alignment: f64 = crab_vec.sum() as f64 / crab_vec.len() as f64;
  let fuel1: i32 = crabs.iter().map(|crab| fuel_used(*crab, candidate_alignment.floor() as i32)).sum();
  let fuel2: i32 = crabs.iter().map(|crab| fuel_used(*crab, candidate_alignment.ceil() as i32)).sum();
  fuel = cmp::min(fuel1, fuel2);
  println!("Part 2: {}", fuel);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}
