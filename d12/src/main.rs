
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Cave {
  large: bool,
  start: bool,
  end: bool,
  edges: HashSet<u8>,
}

fn parse_line(text: &str) -> (String, String) {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^(.+)-(.+)$").unwrap();
  }
  let matches = RE.captures(text).unwrap();
  let cave1 = matches.get(1).unwrap().as_str().to_owned();
  let cave2 = matches.get(2).unwrap().as_str().to_owned();

  return(cave1, cave2);
}

// Recursive DFS
fn cave_dive(caves: &Vec::<Cave>,
             root: u8,
             mut visited: &mut Vec::<u8>,
             double_visit: bool) -> usize {
  let mut count = 0;
  let cave = &caves[root as usize];
  if !cave.large {
    visited[root as usize] += 1;
  }
  if cave.end {
    count += 1;
  } else {
    for neighbour in cave.edges.iter() {
      if visited[*neighbour as usize] == 0 {
        count += cave_dive(&caves, *neighbour, &mut visited, double_visit);
      } else if double_visit {
        let neighbour_cave = &caves[*neighbour as usize];
        if !neighbour_cave.start && !neighbour_cave.end {
          count += cave_dive(&caves, *neighbour, &mut visited, false);
        }
      }
    }
  }
  if visited[root as usize] > 0 {
    visited[root as usize] -= 1;
  }
  return count;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read caves
  let mut caves: Vec<Cave> = Vec::new();
  let mut names: HashMap<String, u8> = HashMap::new();
  let mut id_cntr = 0;
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let (cave1, cave2) = parse_line(&line);
    if !names.contains_key(&cave1) {
      let new_cave = Cave {
        large: cave1.to_uppercase() == cave1,
        start: cave1 == "start",
        end: cave1 == "end",
        edges: HashSet::new()
      };
      caves.push(new_cave);
      names.insert(cave1.clone(), id_cntr);
      id_cntr += 1;
    }
    if !names.contains_key(&cave2) {
      let new_cave = Cave {
        large: cave2.to_uppercase() == cave2,
        start: cave2 == "start",
        end: cave2 == "end",
        edges: HashSet::new()
      };
      caves.push(new_cave);
      names.insert(cave2.clone(), id_cntr);
      id_cntr += 1;
    }
    caves[*names.get(&cave1).unwrap() as usize].edges.insert(*names.get(&cave2).unwrap());
    caves[*names.get(&cave2).unwrap() as usize].edges.insert(*names.get(&cave1).unwrap());
  }

  // Part 1
  let part1 = Instant::now();
  let mut visited: Vec<u8> = vec![0; caves.len()];
  let mut n_paths = cave_dive(&caves, *names.get("start").unwrap(), &mut visited, false);
  println!("Part 1: {}", n_paths);

  // Part 2
  let part2 = Instant::now();
  visited = vec![0; caves.len()];
  n_paths = cave_dive(&caves, *names.get("start").unwrap(), &mut visited, true);
  println!("Part 2: {}", n_paths);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

