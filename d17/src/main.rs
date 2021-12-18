
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::mem;
use lazy_static::lazy_static;
use regex::Regex;

fn parse_line(text: &str) -> (i32, i32, i32, i32) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^target area: x=(\d+)\.\.(\d+), y=-(\d+)\.\.-(\d+)$").unwrap();
  }
  let matches = RE.captures(text).unwrap();
  let mut xmin: i32 = matches.get(1).unwrap().as_str().parse().unwrap();
  let mut xmax: i32 = matches.get(2).unwrap().as_str().parse().unwrap();
  let mut ymin: i32 = matches.get(3).unwrap().as_str().parse::<i32>().unwrap() * -1;
  let mut ymax: i32 = matches.get(4).unwrap().as_str().parse::<i32>().unwrap() * -1;

  if xmin > xmax {
    mem::swap(&mut xmin, &mut xmax);
  }
  if ymin > ymax {
    mem::swap(&mut ymin, &mut ymax);
  }
  return(xmin, xmax, ymin, ymax);
}

fn fire_drone(init_xv: i32, init_yv: i32, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> (bool, i32) {
  let (mut x, mut y, mut xv, mut yv) = (0, 0, init_xv, init_yv);
  let mut hit = false;
  let mut max_y = 0;
  loop {
    x += xv;
    y += yv;
    xv = if xv == 0 {0} else {xv - 1};
    yv -= 1;
    max_y = if y > max_y {y} else {max_y};
    if x <= xmax && x >= xmin && y <= ymax && y >= ymin {
      hit = true;
      break;
    } else if x > xmax || y < ymin {
      break;
    }
  }
  return (hit, max_y);
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let mut f = BufReader::new(f);

  // Read target
  let mut target_line = String::new();
  f.read_line(&mut target_line).expect("Parse error");
  let (xmin, xmax, ymin, ymax) = parse_line(&target_line.trim());

  // Part 1
  let part1 = Instant::now();
  let xinit_min = (0.5 * ((8.0 * xmin as f64 + 1.0).sqrt() - 1.0)).ceil() as i32;
  let xinit_max = xmax;
  let yinit_min = ymin;
  let yinit_max = -ymin; // Ansatz
  let mut guess_grid: Vec::<(i32, i32)> = Vec::new();
  for xinit in xinit_min..=xinit_max {
    for yinit in yinit_min..=yinit_max {
      guess_grid.push((xinit, yinit));
    }
  }
  let best_y = guess_grid.iter()
                          .map(|(x, y)| {
                            let (hit, ytop) = fire_drone(*x, *y, xmin, xmax, ymin, ymax);
                            if hit {ytop} else {i32::MIN}
                          })
                          .max()
                          .unwrap();
  println!("Part 1: {}", best_y);

  // Part 2
  let part2 = Instant::now();
  let trajectories: i32 = guess_grid.iter()
                         .map(|(x, y)| {
                            let (hit, _ytop) = fire_drone(*x, *y, xmin, xmax, ymin, ymax);
                            if hit {1} else {0}
                          })
                          .sum();
  println!("Part 2: {}", trajectories);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

