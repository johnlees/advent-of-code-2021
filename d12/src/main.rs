
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Cave {
  large: bool,
  end: bool,
  edges: HashSet<String>,
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
fn cave_dive(caves: &HashMap::<String, Cave>,
             mut path: &mut Vec::<String>,
             root: &String,
             mut visited: &mut HashSet<String>,
             mut double_visit: bool) -> usize {
  let mut count = 0;
  if !caves.get(root).unwrap().large {
    if double_visit && root != "start" {
      double_visit = false;
    } else {
      visited.insert(root.clone());
    }
  }
  path.push(root.clone());
  if caves.get(root).unwrap().end {
    println!("{:?} {}", path, double_visit);
    count += 1;
  } else {
    for neighbour in caves.get(root).unwrap().edges.iter() {
      if !visited.contains(neighbour) {
        count += cave_dive(&caves, &mut path, neighbour, &mut visited, double_visit);
      }
    }
  }
  path.pop();
  visited.remove(root);
  return count;
}

fn main() {
  let start = Instant::now();
  let input_file = "test_input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read caves
  let mut caves: HashMap::<String, Cave> = HashMap::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let (cave1, cave2) = parse_line(&line);
    if !caves.contains_key(&cave1) {
      let new_cave = Cave {
        large: cave1.to_uppercase() == cave1,
        end: cave1 == "end",
        edges: HashSet::new()
      };
      caves.insert(cave1.clone(), new_cave);
    }
    if !caves.contains_key(&cave2) {
      let new_cave = Cave {
        large: cave2.to_uppercase() == cave2,
        end: cave2 == "end",
        edges: HashSet::new()
      };
      caves.insert(cave2.clone(), new_cave);
    }
    caves.get_mut(&cave1).unwrap().edges.insert(cave2.clone());
    caves.get_mut(&cave2).unwrap().edges.insert(cave1.clone());
  }

  // Part 1
  let part1 = Instant::now();
  let mut visited: HashSet<String> = HashSet::new();
  let mut path: Vec<String> = Vec::new();
  let mut n_paths = cave_dive(&caves, &mut path, &"start".to_owned(), &mut visited, false);
  println!("Part 1: {}", n_paths);

  // Part 2
  let part2 = Instant::now();
  visited = HashSet::new();
  path = Vec::new();
  n_paths = cave_dive(&caves, &mut path, &"start".to_owned(), &mut visited, true);
  println!("Part 2: {}", n_paths);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

