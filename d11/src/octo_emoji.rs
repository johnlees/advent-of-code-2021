
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use std::{thread, time};
extern crate termion;

pub struct OctoMatrix {
  data: Vec<i32>,
  rows: usize,
  cols: usize,
  row_stride: usize,
  col_stride: usize,
  step_n: usize,
}

impl OctoMatrix {
  pub fn new(data: Vec::<i32>, rows: usize, cols: usize) -> Self {
    let col_stride: usize = 1;
    let row_stride = cols;
    let step_n: usize = 0;
    return Self {data, rows, cols, row_stride, col_stride, step_n};
  }

  fn power_octo(&mut self, row: i32, col: i32) -> bool {
    let mut powered = false;
    if row >= 0 && col >= 0 {
      let urow = row as usize;
      let ucol = col as usize;
      if urow < self.rows && ucol < self.cols {
        let energy = &mut self.data[urow * self.row_stride + ucol * self.col_stride];
        if *energy < 10 {
          *energy += 1;
          powered = true;
        }
      }
    }
    return powered;
  }

  // Increment all by 1
  fn increment_power(&mut self) {
    for row in 0..self.rows as i32 {
      for col in 0..self.cols as i32 {
        self.power_octo(row, col);
      }
    }
  }

  fn scan_octo(&mut self) {
    let mut changed = true;
    while changed {
      changed = false;
      for row in 0..self.rows as i32 {
        for col in 0..self.cols as i32 {
          if self.data[row as usize * self.row_stride + col as usize * self.col_stride] == 10 {
            changed |= self.power_octo(row + 1, col);
            changed |= self.power_octo(row - 1, col);
            changed |= self.power_octo(row, col + 1);
            changed |= self.power_octo(row, col - 1);
            changed |= self.power_octo(row + 1, col + 1);
            changed |= self.power_octo(row + 1, col - 1);
            changed |= self.power_octo(row - 1, col + 1);
            changed |= self.power_octo(row - 1, col - 1);
            self.data[row as usize * self.row_stride + col as usize * self.col_stride] = 11;
          }
        }
      }
    }
  }

  fn print_octo(&self) {
    print!("{}{}step: {}", termion::clear::All, termion::cursor::Goto(1, 1), self.step_n);
    for row in 0..self.rows as i32 {
      let mut octo_row: String = String::new();
      for col in 0..self.cols as i32 {
        if self.data[row as usize * self.row_stride + col as usize * self.col_stride] > 9 {
          octo_row.push('✨');
        } else {
          octo_row.push('🐙');
        }
      }
      print!("{}{}{}", termion::cursor::Down(1), termion::cursor::Left(20), octo_row);
    }
    thread::sleep(time::Duration::from_millis(120));
  }

  // Set 10s to zero, return number flashed
  fn reset(&mut self) -> usize {
    let mut flashed = 0;
    for octopus in self.data.iter_mut() {
      if *octopus > 9 {
        flashed += 1;
        *octopus = 0;
      }
    }
    return flashed;
  }

  pub fn step(&mut self, print: bool) -> usize {
    self.step_n += 1;
    println!();
    self.increment_power();
    self.scan_octo();
    if print {
      self.print_octo();
    }
    return self.reset();
  }

  pub fn nrow(&self) -> usize {
    return self.rows;
  }

  pub fn ncol(&self) -> usize {
    return self.cols;
  }
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read grid
  let mut grid_in: Vec::<i32> = Vec::new();
  const RADIX: u32 = 10;
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let mut row: Vec::<i32> = Vec::new();
    for octo in line.chars() {
      let octo_int: i32 = octo.to_digit(RADIX).unwrap() as i32;
      row.push(octo_int as i32)
    }
    grid_in.append(&mut row);
  }
  let grid_copy = grid_in.clone(); // For animation

  // Part 1
  let part1 = Instant::now();
  let mut octopuses: OctoMatrix = OctoMatrix::new(grid_in, 10, 10);
  let mut count = 0;
  for _step in 0..100 {
    count += octopuses.step(false);
  }
  println!("Part 1: {}", count);

  // Part 2
  let part2 = Instant::now();
  let mut step = 100;
  loop {
    step += 1;
    if octopuses.step(false) == 100 {
      break;
    }
  }
  println!("Part 2: {}", step);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());

  // Bonus animation
  octopuses = OctoMatrix::new(grid_copy, 10, 10);
  loop {
    step += 1;
    if octopuses.step(true) == 100 {
      break;
    }
  }
  println!();
}

