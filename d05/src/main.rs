
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use ndarray::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

fn parse_coor(text: &str) -> (i32, i32, i32, i32) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
  }
  let matches = RE.captures(text).unwrap();
  let mut x1: i32 = matches.get(1).unwrap().as_str().parse().unwrap();
  let mut y1: i32 = matches.get(2).unwrap().as_str().parse().unwrap();
  let mut x2: i32 = matches.get(3).unwrap().as_str().parse().unwrap();
  let mut y2: i32 = matches.get(4).unwrap().as_str().parse().unwrap();

  if x1 * x1 + y1 * y1 > x2 * x2 + y2 * y2 {
    mem::swap(&mut x1, &mut x2);
    mem::swap(&mut y1, &mut y2);
  }
  return (x1, y1, x2, y2);
}

fn main() {
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read coordinates
  let (mut max_x, mut max_y) = (0, 0);
  let mut vents = Vec::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let (x1, y1, x2, y2) = parse_coor(&line);
    max_x = if x1 > max_x {x1} else {max_x};
    max_x = if x2 > max_x {x2} else {max_x};
    max_y = if y1 > max_y {y1} else {max_y};
    max_y = if y2 > max_y {y2} else {max_y};
    vents.push((x1, y1, x2, y2));
  }

  // Part 1
  let max_x: usize = max_x.try_into().unwrap();
  let max_y: usize = max_y.try_into().unwrap();
  let mut sea = Array2::<i32>::zeros((max_x + 1, max_y + 1));
  for coor in vents {
    let (x1, y1, x2, y2) = coor;
    if x1 == x2 {
      let mut slice = sea.slice_mut(s![y1..(y2+1), x1]);
      slice += 1;
    } else if y1 == y2 {
      let mut slice = sea.slice_mut(s![y1, x1..(x2+1)]);
      slice += 1;
    }
  }
  let mut danger = 0;
  for &pos in sea.iter() {
    if pos > 1 {
      danger += 1
    }
  }
  println!("Part 1: {}", danger);


}
