
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;

use ndarray::Array2;

const GRID_SIZE: usize = 5;

fn score_grid(grid: &Array2::<i32>, calls: &HashMap<i32, usize>) -> (i32, usize) {
  let mut position = Vec::new();
  // Iterate over rows and columns
  for row in 0..GRID_SIZE {
    let mut last_called_row = 0;
    let mut last_called_col = 0;
    for col in 0..GRID_SIZE {
      if calls[&grid[[row, col]]] > last_called_row {
        last_called_row = calls[&grid[[row, col]]];
      }
      if calls[&grid[[col, row]]] > last_called_col {
        last_called_col = calls[&grid[[col, row]]];
      }
    }
    position.push(last_called_row);
    position.push(last_called_col);
  }
  // Calculate score
  let best_pos = *position.iter().min().unwrap();
  let mut score = 0;
  for val in grid.iter() {
    if calls[val] > best_pos {
      score += val;
    }
  }
  let last_call = calls.iter().find_map(|(key, &val)| if val == best_pos { Some(key) } else { None });
  score *= last_call.unwrap();

  return (score.try_into().unwrap(), best_pos);
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let mut f = BufReader::new(f);

  // Read calls
  let mut call_line = String::new();
  f.read_line(&mut call_line).expect("Parse error");
  let mut calls = HashMap::new();
  for (idx, call) in call_line.trim().split(',').enumerate() {
    let call_num: i32 = call.parse().unwrap();
    calls.insert(call_num, idx);
  }
  f.read_line(&mut call_line).expect("Parse error");

  // Read grids
  let mut grids: Vec<Array2::<i32>> = Vec::new();
  let mut array_buf = Array2::<i32>::zeros((GRID_SIZE, GRID_SIZE));
  let mut row_idx = 0;
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    if line.is_empty() {
      row_idx = 0;
      grids.push(array_buf.clone());
    } else {
      let grid_vals = line.split_whitespace();
      for (col_idx, val) in grid_vals.enumerate() {
        array_buf[[row_idx, col_idx]] = val.parse().unwrap();
      }
      row_idx += 1;
    }
  }
  grids.push(array_buf);

  // Part 1
  let part1 = Instant::now();
  let mut min_rank = calls.len();
  let mut score = 0;
  for grid in &grids {
    let (grid_score, position) = score_grid(&grid, &calls);
    if position < min_rank {
      score = grid_score;
      min_rank = position;
    }
  }
  println!("Part 1: {}", score);

  // Part 2
  let part2 = Instant::now();
  let mut max_rank = 0;
  score = 0;
  for grid in &grids {
    let (grid_score, position) = score_grid(&grid, &calls);
    if position > max_rank {
      score = grid_score;
      max_rank = position;
    }
  }
  println!("Part 2: {}", score);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}
