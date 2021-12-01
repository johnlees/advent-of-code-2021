
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let input_file = "input.txt";
  let mut depths: Vec<i32> = Vec::new();
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let depth: i32 = line.parse().unwrap();
    depths.push(depth);
  }

  let mut larger = 0;
  for (idx, depth) in depths.iter().enumerate() {
    if idx < depths.len() - 1 && depth < &depths[idx + 1] {
      larger += 1;
    }
  }
  println!("Part 1: {}", larger);

  larger = 0;
  let window_size = 3;
  let mut sliding: i32 = depths[0..window_size].iter().sum();
  for idx in 0..(depths.len() - window_size) {
    let new_sliding = sliding - depths[idx] + depths[idx + window_size];
    if new_sliding > sliding {
      larger += 1;
    }
    sliding = new_sliding;
  }
  println!("Part 2: {}", larger);
}
