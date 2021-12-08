
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;

fn add_to_set(segments: &String, set: &mut HashSet<char>) {
  for seg in segments.chars() {
    set.insert(seg);
  }
}

/* set diffs
* a = 3 - 2
c and f = 2
c and e and d = diff between all 6s
a and b and f and g = common between all 6s
b and d = 4 - 2
=> b = common between all 6s ^ (4 - 2)
=> d = (4 - 2) - b
=> c = diff between all 6s ^ 2
=> f = 2 - c
=> e = diff between all 6s - c - d
=> g = left over */
fn map_digits(output: &Vec::<String>) -> HashMap<char, char> {
  let mut two_len: HashSet<char> = HashSet::new();
  let mut three_len: HashSet<char> = HashSet::new();
  let mut four_len: HashSet<char> = HashSet::new();
  let mut sixes: Vec<HashSet<char>> = Vec::new();
  for out_segs in output {
    match out_segs.len() {
      2 => add_to_set(&out_segs, &mut two_len),
      3 => add_to_set(&out_segs, &mut three_len),
      4 => add_to_set(&out_segs, &mut four_len),
      6 => {
        let mut six = HashSet::new();
        add_to_set(&out_segs, &mut six);
        sixes.push(six);},
      _ => ()
    }
  }
  let mut char_map: HashMap<char, char> = HashMap::new();
  char_map.insert('a', *three_len.difference(&two_len).next().unwrap());

  let six_common = &sixes[0] & &sixes[1];
  let six_common = &six_common & &sixes[2];
  let six_diff1 = &sixes[0] ^ &sixes[1];
  let six_diff2 = &sixes[1] ^ &sixes[2];
  let mut six_diff = &six_diff1 | &six_diff2;

  let mut b_and_d: HashSet<char> = HashSet::new();
  for seg in four_len.difference(&two_len) {
    b_and_d.insert(*seg);
  }
  char_map.insert('b', *six_common.intersection(&b_and_d).next().unwrap());
  b_and_d.remove(&char_map[&'b']);
  char_map.insert('d', *b_and_d.iter().next().unwrap());
  char_map.insert('c', *six_diff.intersection(&two_len).next().unwrap());
  two_len.remove(&char_map[&'c']);
  char_map.insert('f', *two_len.iter().next().unwrap());
  six_diff.remove(&char_map[&'c']);
  six_diff.remove(&char_map[&'d']);
  char_map.insert('e', *six_diff.iter().next().unwrap());
  let mut all_chars: HashSet<char> = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
  for found in char_map.values() {
    all_chars.remove(found);
  }
  char_map.insert('g', *all_chars.iter().next().unwrap());

  // Above is the wrong way round... sort out later, easiest to just:
  let mut inverse_map: HashMap<char, char> = HashMap::new();
  for (key, value) in char_map {
    inverse_map.insert(value, key);
  }

  return inverse_map;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read digits
  let mut signal: Vec::<Vec::<String>> = Vec::new();
  let mut output: Vec::<Vec::<String>> = Vec::new();
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let mut sig_line: Vec::<String> = Vec::new();
    let mut out_line: Vec::<String> = Vec::new();
    for (idx, entry) in line.split_whitespace().enumerate() {
      match idx {
        0..=9 => sig_line.push(entry.to_string()),
        10 => (),
        11..=14 => out_line.push(entry.to_string()),
        _ => panic!("Unparsed input")
      }
    }
    signal.push(sig_line);
    output.push(out_line);
  }

  // Part 1
  let part1 = Instant::now();
  let mut count = 0;
  for out_line in &output {
    for out_segs in out_line {
      match out_segs.len() {
        2 | 3 | 4 | 7 => count += 1,
        _ => ()
      }
    }
  }
  println!("Part 1: {}", count);

  let part2 = Instant::now();
  let mut total = 0;
  for idx in 0..signal.len() {
    let map = map_digits(&signal[idx]);
    let mut out_str = String::new();
    for out in &output[idx] {
      let mut sig_str = Vec::new();
      for seg in out.chars() {
        sig_str.push(map[&seg]);
      }
      sig_str.sort();
      let sig_str: String = sig_str.into_iter().collect();
      match sig_str.as_str() {
        "abcefg" => out_str += "0",
        "cf" => out_str += "1",
        "acdeg" => out_str += "2",
        "acdfg" => out_str += "3",
        "bcdf" => out_str += "4",
        "abdfg" => out_str += "5",
        "abdefg" => out_str += "6",
        "acf" => out_str += "7",
        "abcdefg" => out_str += "8",
        "abcdfg" => out_str += "9",
        _ => panic!("failed display")
      }
    }
    let signal_num: i32 = out_str.parse().unwrap();
    total += signal_num;
  }
  println!("Part 2: {}", total);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}
