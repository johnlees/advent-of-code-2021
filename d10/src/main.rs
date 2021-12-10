
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read grid
  let mut nav_lines: Vec::<String> = Vec::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    nav_lines.push(line);
  }

  // Part 1
  let part1 = Instant::now();
  let p1_score_table = HashMap::from([
    (')', 3),
    (']', 57),
    ('}', 1197),
    ('>', 25137),
  ]);
  let mut score = 0;
  let mut incomplete_lines: Vec::<&String> = Vec::new();
  for nav in &nav_lines {
    let mut corrupt = false;
    let mut braces: Vec::<char> = Vec::new();
    for brace in nav.chars() {
      match brace {
        '(' | '[' | '<' | '{' => braces.push(brace),
        _ => {
          let open = braces.pop().unwrap();
          if (open == '(' && brace != ')') ||
             (open == '[' && brace != ']') ||
             (open == '<' && brace != '>') ||
             (open == '{' && brace != '}') {
            score += p1_score_table[&brace];
            corrupt = true;
            break;
          }
        }
      }
    }
    if !corrupt {
      incomplete_lines.push(nav);
    }
  }
  println!("Part 1: {}", score);

  // Part 2
  let part2 = Instant::now();
  let p2_score_table = HashMap::from([
    ('(', 1),
    ('[', 2),
    ('{', 3),
    ('<', 4),
  ]);
  let mut score_list: Vec<i64> = Vec::new();
  for nav in incomplete_lines {
    score = 0;
    let mut braces: Vec::<char> = Vec::new();
    for brace in nav.chars() {
      let mut _close;
      match brace {
        '(' | '[' | '<' | '{' => braces.push(brace),
        _ => _close = braces.pop().unwrap(),
      }
    }
    for opened in braces.iter().rev() {
      score *= 5;
      score += p2_score_table[opened];
    }
    score_list.push(score);
  }
  score_list.sort();
  println!("Part 2: {}", score_list[score_list.len() / 2]);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

