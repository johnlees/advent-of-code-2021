
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// Trying to learn a bit about classes in rust
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
    let col_stride: usize = 1;
    let row_stride = cols;
    return Self {data, rows, cols, row_stride, col_stride};
  }

  // Overriding Index and IndexMut may be better,
  // but I was finding it difficult to get it to take two values
  pub fn at(&mut self, row: i32, col: i32) -> i32 {
    // This is specialised for the problem
    if row < 0 || col < 0 {
      return i32::MAX;
    }
    let urow = row as usize;
    let ucol = col as usize;
    if urow >= self.rows || ucol >= self.cols {
      return i32::MAX;
    }
    return self.data[urow * self.row_stride + ucol * self.col_stride];
  }

  pub fn at_mut(&mut self, row: usize, col: usize) -> &mut i32 {
    if row > self.rows {
      panic!("Row index greater than size")
    }
    if col > self.cols {
      panic!("Col index greater than size")
    }
    return &mut self.data[row * self.row_stride + col * self.col_stride];
  }

  pub fn nrow(&self) -> usize {
    return self.rows;
  }

  pub fn ncol(&self) -> usize {
    return self.cols;
  }
}

fn count_basin(mut floor: &mut Matrix<i32>, mut basin_cnt: &mut Matrix<i32>, i: i32, j: i32) -> usize {
  let mut size = 1;
  *basin_cnt.at_mut(i as usize, j as usize) = 1;
  if floor.at(i + 1, j) < 9 && basin_cnt.at(i + 1, j) == 0 {
    size += count_basin(&mut floor, &mut basin_cnt, i + 1, j);
  }
  if floor.at(i - 1, j) < 9 && basin_cnt.at(i - 1, j) == 0 {
    size += count_basin(&mut floor, &mut basin_cnt, i - 1, j);
  }
  if floor.at(i, j + 1) < 9 && basin_cnt.at(i, j + 1) == 0 {
    size += count_basin(&mut floor, &mut basin_cnt, i, j + 1);
  }
  if floor.at(i, j - 1) < 9 && basin_cnt.at(i, j - 1) == 0 {
    size += count_basin(&mut floor, &mut basin_cnt, i, j - 1);
  }
  return size;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read grid
  let mut grid_in: Vec::<Vec::<i32>> = Vec::new();
  const RADIX: u32 = 10;
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let mut row: Vec::<i32> = Vec::new();
    for height in line.chars() {
      let height_int: i32 = height.to_digit(RADIX).unwrap() as i32;
      row.push(height_int as i32)
    }
    grid_in.push(row);
  }
  let mut floor: Matrix<i32> = Matrix::new(grid_in.len(), grid_in[0].len());
  for i in 0..floor.nrow() {
    for j in 0..floor.ncol() {
      *floor.at_mut(i, j) = grid_in[i][j];
    }
  }

  // Part 1
  let part1 = Instant::now();
  let mut count = 0;
  for i in 0..floor.nrow() as i32 {
    for j in 0..floor.ncol() as i32 {
      let val = floor.at(i, j);
      if val != 9 &&
         val < floor.at(i + 1, j) &&
         val < floor.at(i - 1, j) &&
         val < floor.at(i, j + 1) &&
         val < floor.at(i, j - 1) {
        count += val + 1;
      }
    }
  }
  println!("Part 1: {}", count);

  // Part 2
  let part2 = Instant::now();
  let mut basin_cnt: Matrix<i32> = Matrix::new(floor.nrow(), floor.ncol());
  let mut basin_sizes: Vec<usize> = Vec::new();
  for i in 0..floor.nrow() as i32 {
    for j in 0..floor.ncol() as i32 {
      if floor.at(i, j) != 9 && basin_cnt.at(i, j) == 0 {
        basin_sizes.push(count_basin(&mut floor, &mut basin_cnt, i, j));
      }
    }
  }
  basin_sizes.sort();
  let mut size_mul = 1;
  for idx in (basin_sizes.len() - 3)..(basin_sizes.len()) {
    size_mul *= basin_sizes[idx];
  }
  println!("Part 2: {}", size_mul);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

