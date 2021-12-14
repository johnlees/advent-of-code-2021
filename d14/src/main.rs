
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::{HashMap, BTreeMap};

use fm_index::converter::RangeConverter;
use fm_index::suffix_array::NullSampler;
use fm_index::{BackwardSearchIndex, FMIndex};

// OK for part 1
fn extend_polymer_brute(steps: i32, polymer_in: &String, replication_rules: &HashMap<(char, char), char>) -> String {
  let mut step = 0;
  let mut current_polymer = polymer_in.clone();
  let mut next_polymer = String::new();
  while step < steps {
    let poly_vec: Vec<char> = current_polymer.chars().collect();
    for (idx, substrate) in poly_vec.iter().enumerate() {
      next_polymer.push(*substrate);
      if idx == poly_vec.len() - 1 {
        break;
      }
      let rule = replication_rules.get(&(*substrate, poly_vec[idx + 1]));
      if rule.is_some() {
        next_polymer.push(*rule.unwrap());
      }
    }
    current_polymer = next_polymer;
    next_polymer = String::new();
    step += 1;
  }
  return current_polymer;
}

// Do counts with an FM-index (for 'fun')
fn get_counts(polymer: &String) -> u64 {
  let text = polymer.as_bytes().to_vec();
  let converter = RangeConverter::new(b' ', b'~');
  let sampler = NullSampler::new();
  let index = FMIndex::new(text, converter, sampler);
  let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
  let mut max_n = 0;
  let mut min_n = u64::MAX;
  for letter in alphabet.iter() {
    let search = index.search_backward(String::from(*letter));
    let n = search.count();
    max_n = if n > max_n {n} else {max_n};
    min_n = if n > 0 && n < min_n {n} else {min_n};
  }
  return max_n - min_n;
}

fn extend_polymer_pairs(steps: i32,
                        polymer_in: &String,
                        replication_rules: &HashMap<(char, char), char>) -> u64 {
  let mut polymer_pairs: BTreeMap<(char, char), u64> = BTreeMap::new();
  let poly_vec: Vec<char> = polymer_in.chars().collect();
  for (idx, substrate) in poly_vec.iter().enumerate() {
    if idx == poly_vec.len() - 1 {
      break;
    } else {
      let new_pair = (*substrate, poly_vec[idx + 1]);
      match polymer_pairs.get_mut(&new_pair) {
        Some(x) => *x += 1,
        None => {polymer_pairs.insert(new_pair, 1);}
      }
    }
  }

  let mut step = 0;
  while step < steps {
    step += 1;
    let mut next_polymer_pairs: BTreeMap<(char, char), u64> = BTreeMap::new();
    for (pair, count) in polymer_pairs.iter() {
      let rule = replication_rules.get(pair);
      if rule.is_some() {
        // If in rules, add count to pair[0] + rule and pair[1] + rule
        // If not yet set, initialise
        let (pair0, pair1) = pair;
        let add = *rule.unwrap();
        let new_pair_0 = (*pair0, add);
        let new_pair_1 = (add, *pair1);
        match next_polymer_pairs.get_mut(&new_pair_0) {
          Some(x) => *x += count,
          None => {next_polymer_pairs.insert(new_pair_0, *count);}
        }
        match next_polymer_pairs.get_mut(&new_pair_1) {
          Some(x) => *x += count,
          None => {next_polymer_pairs.insert(new_pair_1, *count);}
        }
      } else {
        match next_polymer_pairs.get_mut(&pair) {
          Some(x) => *x += count,
          None => {next_polymer_pairs.insert(*pair, *count);}
        }
      }
    }
    polymer_pairs = next_polymer_pairs;
  }

  let mut substrate_counts: HashMap<char, u64> = HashMap::new();
  for (pair, count) in polymer_pairs.iter() {
    let (p1, p2) = pair;
    match substrate_counts.get_mut(&p1) {
      Some(x) => *x += count,
      None => {substrate_counts.insert(*p1, *count);}
    }
    match substrate_counts.get_mut(&p2) {
      Some(x) => *x += count,
      None => {substrate_counts.insert(*p2, *count);}
    }
  }

  // Note that all subtrates are double counted by counting pairs,
  // apart from the first and last values which are counted once
  // The map deals with this
  let counts: Vec::<u64> = substrate_counts.into_values().map(|x| x / 2 + x % 2).collect();
  let min = counts.iter().min().unwrap();
  let max = counts.iter().max().unwrap();

  return max - min;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let mut f = BufReader::new(f);

  // Read grid
  let mut current_polymer = String::new();
  f.read_line(&mut current_polymer).expect("Parse error");
  let mut current_polymer_p1 = current_polymer.trim().to_string();
  let current_polymer_p2 = current_polymer_p1.clone();
  let mut empty = String::new();
  f.read_line(&mut empty).expect("Parse error");
  let mut replication_rules: HashMap<(char, char), char> = HashMap::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let line_parse: Vec<char> = line.chars().collect();
    replication_rules.insert((line_parse[0], line_parse[1]), line_parse[6]);
  }

  // Part 1
  let part1 = Instant::now();
  let steps = 10;
  current_polymer_p1 = extend_polymer_brute(steps, &current_polymer_p1, &replication_rules);
  println!("Part 1: {}", get_counts(&current_polymer_p1));

  // Part 2
  let part2 = Instant::now();
  let steps = 40;
  let answer = extend_polymer_pairs(steps, &current_polymer_p2, &replication_rules);
  println!("Part 2: {}", answer);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

