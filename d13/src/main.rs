
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::{HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

extern crate termion;

fn parse_line(text: &str) -> (bool, i32) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
  }
  let matches = RE.captures(text).unwrap();
  let dir = matches.get(1).unwrap().as_str();
  let mut horizontal = false;
  if dir == "y" {
    horizontal = true;
  }
  let intercept: i32 = matches.get(2).unwrap().as_str().parse().unwrap();

  return(horizontal, intercept);
}

fn do_fold(paper: &HashSet::<(i32, i32)>, horizontal: bool, intercept: i32) -> HashSet::<(i32, i32)> {
  let mut folded_paper: HashSet::<(i32, i32)> = HashSet::new();
  for point in paper.iter() {
    let (mut x, mut y) = point;
    if horizontal && y > intercept {
      y = 2 * intercept - y;
    } else if !horizontal && x > intercept {
      x = 2 * intercept - x;
    }
    folded_paper.insert((x, y));
  }
  return folded_paper;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read grid
  let mut paper: HashSet::<(i32, i32)> = HashSet::new();
  let mut fold_directions: VecDeque::<(bool, i32)> = VecDeque::new();
  let mut fold_lines = false;
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    if line == "" {
      fold_lines = true;
    } else if fold_lines {
      fold_directions.push_back(parse_line(&line));
    } else {
      let mut line_split = line.split(",");
      let x: i32 = line_split.next().unwrap().parse().unwrap();
      let y: i32 = line_split.next().unwrap().parse().unwrap();
      paper.insert((x, y));
    }
  }

  // Part 1
  let part1 = Instant::now();
  let (horizontal, intercept) = fold_directions.pop_front().unwrap();
  paper = do_fold(&paper, horizontal, intercept);
  println!("Part 1: {}", paper.len());

  // Part 2
  let part2 = Instant::now();
  for instruction in fold_directions.iter() {
    let (horizontal, intercept) = instruction;
    paper = do_fold(&paper, *horizontal, *intercept);
  }
  let x_len = *paper.iter().map(|(x,_y)| x).max().unwrap() as usize + 1;
  let y_len = *paper.iter().map(|(_x,y)| y).max().unwrap() as usize + 1;
  let mut empty_lines: Vec::<char> = vec!['.'; x_len * y_len];
  for point in paper.iter() {
    let (x, y) = point;
    empty_lines[*y as usize * x_len + *x as usize] = '#';
  }
  println!("Part 2");
  for row in 0..y_len {
    let row_string: String = empty_lines[(row * x_len)..((row + 1) * x_len)].into_iter().collect();
    println!("{}", row_string);
  }

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

