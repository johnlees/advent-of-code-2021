
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::time::Instant;

use ndarray::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

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

fn sense_danger(sea: &Array2::<i32>) -> usize {
  let mut danger = 0;
  for &pos in sea.iter() {
    if pos > 1 {
      danger += 1
    }
  }
  return danger;
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
  let mut sea = Array2::<i32>::zeros((max_x + 1, max_y + 1));
  for coor in &vents {
    let (x1, y1, x2, y2) = *coor;
    if x1 == x2 {
      let mut slice = sea.slice_mut(s![x1, y1..(y2+1)]);
      slice += 1;
    } else if y1 == y2 {
      let mut slice = sea.slice_mut(s![x1..(x2+1), y1]);
      slice += 1;
    }
  }
  println!("Part 1: {}", sense_danger(&sea));

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
          sea[[x1 + point, y1 + point]] += 1;
        } else {
          sea[[x1 + point, y1 - point]] += 1;
        }
      }
    }
  }
  println!("Part 2: {}", sense_danger(&sea));

  let end = Instant::now();
  println!("parsing: {}??s\npart 1: {}??s\npart 2: {}??s",
  part1.duration_since(start).as_micros(),
  part2.duration_since(part1).as_micros(),
  end.duration_since(part2).as_micros());
}
