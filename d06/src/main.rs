
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::time::Instant;

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let mut f = BufReader::new(f);

  // Read coordinates
  let mut school: Vec::<usize> = vec![0; 9];
  let mut init_fish = String::new();
  f.read_line(&mut init_fish).expect("Parse error");
  for fish in init_fish.trim().split(',') {
    let fish_days: usize = fish.parse().unwrap();
    school[fish_days] += 1;
  }

  // Part 1
  let start_sim = Instant::now();
  let mut next_school: Vec::<usize> = vec![0; 9];
  for _day in 0..80 {
    next_school[0..8].copy_from_slice(&school[1..9]);
    next_school[6] += school[0];
    next_school[8] = school[0];
    mem::swap(&mut school, &mut next_school);
  }
  let total_fish: usize = school.iter().sum();
  println!("Part 1: {}", total_fish);

  // Part 2
  for _day in 80..256 {
    next_school[0..8].copy_from_slice(&school[1..9]);
    next_school[6] += school[0];
    next_school[8] = school[0];
    mem::swap(&mut school, &mut next_school);
  }
  let total_fish: usize = school.iter().sum();
  println!("Part 2: {}", total_fish);

  println!("parsing: {}µs\nsimulation: {}µs\nall: {}µs", start_sim.duration_since(start).as_micros(), start_sim.elapsed().as_micros(), start.elapsed().as_micros());
}
