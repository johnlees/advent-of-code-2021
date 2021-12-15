
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// See https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
  // dist[node] = current shortest distance from `start` to `node`
  let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

  let mut heap = BinaryHeap::new();

  // We're at `start`, with a zero cost
  dist[start] = 0;
  heap.push(State { cost: 0, position: start });

  // Examine the frontier with lower cost nodes first (min-heap)
  while let Some(State { cost, position }) = heap.pop() {
      // Alternatively we could have continued to find all shortest paths
      if position == goal { return Some(cost); }

      // Important as we may have already found a better way
      if cost > dist[position] { continue; }

      // For each node we can reach, see if we can find a way with
      // a lower cost going through this node
      for edge in &adj_list[position] {
          let next = State { cost: cost + edge.cost, position: edge.node };

          // If so, add it to the frontier and continue
          if next.cost < dist[next.position] {
              heap.push(next);
              // Relaxation, we have now found a better way
              dist[next.position] = next.cost;
          }
      }
  }

  // Goal not reachable
  None
}

fn idx_to_x_y(idx: usize, len: usize) -> (usize, usize) {
  let x_val = idx % len;
  let y_val = idx / len;
  return(x_val, y_val);
}

fn x_y_to_idx(x: usize, y: usize, len: usize) -> usize {
  return y * len + x;
}

fn grid_to_graph(grid_in: &Vec::<usize>) -> Vec<Vec<Edge>> {
  let mut graph: Vec<Vec<Edge>> = Vec::new();
  let grid_length = (grid_in.len() as f64).sqrt().round() as usize;
  for (idx, val) in grid_in.iter().enumerate() {
    let (x_val, y_val) = idx_to_x_y(idx, grid_length);
    let mut edges: Vec<Edge> = Vec::new();
    if (x_val as i32 - 1) > 0 {
      let edge_idx = x_y_to_idx(x_val - 1, y_val, grid_length);
      edges.push(Edge {node: edge_idx, cost: *val});
    }
    if (x_val as i32 + 1) < grid_length as i32 {
      let edge_idx = x_y_to_idx(x_val + 1, y_val, grid_length);
      edges.push(Edge {node: edge_idx, cost: *val});
    }
    if (y_val as i32 - 1) > 0 {
      let edge_idx = x_y_to_idx(x_val, y_val - 1, grid_length);
      edges.push(Edge {node: edge_idx, cost: *val});
    }
    if (y_val as i32 + 1) < grid_length as i32 {
      let edge_idx = x_y_to_idx(x_val, y_val + 1, grid_length);
      edges.push(Edge {node: edge_idx, cost: *val});
    }
    graph.push(edges);
  }
  return graph;
}

fn main() {
  let start = Instant::now();
  let input_file = "input.txt";
  let f = File::open(input_file).expect("Unable to open file");
  let f = BufReader::new(f);

  // Read chitons
  let mut grid_in: Vec::<usize> = Vec::new();
  const RADIX: u32 = 10;
  for line in f.lines() {
    let line = line.expect("Unable to read line");
    let mut row: Vec::<usize> = Vec::new();
    for chiton in line.chars() {
      let chiton_int: i32 = chiton.to_digit(RADIX).unwrap() as i32;
      row.push(chiton_int as usize)
    }
    grid_in.append(&mut row);
  }

  // Part 1
  let part1 = Instant::now();
  let graph = grid_to_graph(&grid_in);
  let path_len = shortest_path(&graph, 0, grid_in.len() - 1).unwrap() - grid_in[0] + grid_in[grid_in.len() - 1];
  println!("Part 1: {}", path_len);

  // Part 2
  let part2 = Instant::now();
  let repeats = 5;
  let grid_length = (grid_in.len() as f64).sqrt().round() as usize;
  let mut repeated_grid = vec![0; repeats * repeats * grid_in.len()];
  for (idx, val) in grid_in.iter().enumerate() {
    let (x_val, y_val) = idx_to_x_y(idx, grid_length);
    for x_repeat in 0..repeats {
      let x_val_rep = x_val + x_repeat * grid_length;
      for y_repeat in 0..repeats {
        let y_val_rep = y_val + y_repeat * grid_length;
        let mut risk = *val + x_repeat + y_repeat;
        risk = if risk > 9 {risk - 9} else {risk};
        repeated_grid[x_y_to_idx(x_val_rep, y_val_rep, grid_length * repeats)] = risk;
      }
    }
  }
  let graph = grid_to_graph(&repeated_grid);
  let path_len = shortest_path(&graph, 0, repeated_grid.len() - 1).unwrap() - repeated_grid[0] + repeated_grid[repeated_grid.len() - 1];
  println!("Part 2: {}", path_len);

  let end = Instant::now();
  println!("parsing: {}µs\npart 1: {}µs\npart 2: {}µs",
           part1.duration_since(start).as_micros(),
           part2.duration_since(part1).as_micros(),
           end.duration_since(part2).as_micros());
}

