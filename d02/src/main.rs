
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  let part1 = Instant::now();
  let (mut x, mut y1, mut y2, mut aim) = (0, 0, 0, 0);
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let v: Vec<&str> = line.split_whitespace().collect();
    let amount: i64 = v[1].parse().unwrap();
    match &v[0] {
      &"forward" => {
        x += amount;
        y2 += aim * amount;
      },
      &"down" => {
        y1 += amount;
        aim += amount;
      },
      &"up" => {
        y1 -= amount;
        aim -= amount;
      },
      _ => panic!("unparsed input")
    }
  }

  println!("Part 1: {}", x * y1);
  println!("Part 2: {}", x * y2);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           end.duration_since(part1).as_micros(),
           end.duration_since(part1).as_micros());
}
