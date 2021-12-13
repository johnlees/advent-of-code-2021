
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

// Taken from d09
pub struct Matrix<T> {
  data: Vec<T>,
  rows: usize,
  cols: usize,
  row_stride: usize,
  col_stride: usize,
}

impl Matrix<i32> {
  pub fn new(rows: usize, cols: usize) -> Self {
    let data: Vec::<i32> = vec![0; rows * cols];
    let col_stride: usize = cols;
    let row_stride: usize = 1;
    return Self {data, rows, cols, row_stride, col_stride};
  }

  pub fn at(&mut self, row: usize, col: usize) -> i32 {
    return self.data[row * self.row_stride + col * self.col_stride];
  }

  pub fn at_mut(&mut self, row: usize, col: usize) -> &mut i32 {
    return &mut self.data[row * self.row_stride + col * self.col_stride];
  }

  pub fn nrow(&self) -> usize {
    return self.rows;
  }

  pub fn ncol(&self) -> usize {
    return self.cols;
  }

  pub fn sense_danger(&self) -> usize {
    let danger: usize = self.data.iter().map(|x| if *x > 1 {1} else {0}).sum();
    return danger;
  }
}

fn parse_coor(text: &str) -> (usize, usize, usize, usize) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
  }
  let matches = RE.captures(text).unwrap();
  let mut x1: usize = matches.get(1).unwrap().as_str().parse().unwrap();
  let mut y1: usize = matches.get(2).unwrap().as_str().parse().unwrap();
  let mut x2: usize = matches.get(3).unwrap().as_str().parse().unwrap();
  let mut y2: usize = matches.get(4).unwrap().as_str().parse().unwrap();

  if x1 > x2 || y1 > y2 {
    mem::swap(&mut x1, &mut x2);
    mem::swap(&mut y1, &mut y2);
  }
  return (x1, y1, x2, y2);
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read coordinates
  let (mut max_x, mut max_y) = (0, 0);
  let mut vents = Vec::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let (x1, y1, x2, y2) = parse_coor(&line);
    max_x = if x1 > max_x {x1} else if x2 > max_x {x2} else {max_x};
    max_y = if y1 > max_y {y1} else if y2 > max_y {y2} else {max_y};
    vents.push((x1, y1, x2, y2));
  }

  // Part 1
  let part1 = Instant::now();
  let mut sea = Matrix::<i32>::new(max_x + 1, max_y + 1);
  for coor in &vents {
    let (x1, y1, x2, y2) = *coor;
    if x1 == x2 {
      for y in y1..=y2 {
        *sea.at_mut(x1, y) += 1;
      }
    } else if y1 == y2 {
      for x in x1..=x2 {
        *sea.at_mut(x, y1) += 1;
      }
    }
  }
  println!("Part 1: {}", sea.sense_danger());

  // Part 2
  let part2 = Instant::now();
  for coor in &vents {
    let (mut x1, mut y1, mut x2, mut y2) = *coor;
    if x1 != x2 && y1 != y2 {
      // Always start from left-most point
      if x1 > x2 {
        mem::swap(&mut x1, &mut x2);
        mem::swap(&mut y1, &mut y2);
      }
      let length = x2 - x1 + 1;
      for point in 0..length {
        // Line may go either up or down
        if y2 > y1 {
          *sea.at_mut(x1 + point, y1 + point) += 1;
        } else {
          *sea.at_mut(x1 + point, y1 - point) += 1;
        }
      }
    }
  }
  println!("Part 2: {}", sea.sense_danger());

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
  part1.duration_since(start).as_micros(),
  part2.duration_since(part1).as_micros(),
  end.duration_since(part2).as_micros());
}
