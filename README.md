# advent-of-code-2021
Rust this time

## Timings

Run with `RUSTFLAGS="-C target-cpu=native" cargo run --release`. All times in µs.

| Day | Title | Parsing  | Part 1 | Part 2 | Notes |
| --- | ----- | -------- | ------ | ------ | ----- |
| 1 | Sonar Sweep |  255  | 25 | 8 | Sliding window. |
| 2 | Dive!  | 36  | 356 | 356 | Time is both parts together. |
| 3 | Binary Diagnostic  |  378  | 20 | 686 | `BitVec`; not working ideally for part 2. |
| 4 | Giant Squid  |  204  | 240 | 181 | `ndarray`, but suboptimal iteration. |
| 5 | Hydrothermal Venture  |  666  | 1842 | 281 | `ndarray`: Slices for part 1; index for part 2. |
| 6 | Lanternfish  |  57  | 17 | 2 | State space model. `memcpy` faster than ptr move. |
| 7 | The Treachery of Whales  |  182  | 89 | 245 | Part 1 median. Part 2 `iter().map()`. |
| 8 | Seven Segment Search  |  440  | 21 | 1503 | Use `HashSet` for logic. |
| 9 | Smoke Basin  |  228  | 108 | 248 | Manual `Matrix` class. Part 2 recursive. |
| 10 | Syntax Scoring  |  79  | 123 | 61 | `Vec.pop()` to make a stack. |
| 11 | Dumbo Octopus  |  66  | 132 | 126 | Manual `OctoMatrix` class. Line scan to update. `octo_emoji` writes 🐙/✨. |
| 12 | Passage Pathing  |  275  | 256 | 5866 | Recursive DFS ignoring some nodes. |
| 13 | Transparent Origami  |  609  | 80 | 237 | `HashSet` for paper. |
| 14 | Extended Polymerization  |  127  | 2440 | 440 | Part 1: brute force + FM-index. Part 2: `BTreeMap` of 2-mers. |
| 15 | Chiton  |  209  | 1937 | 64008 | Djikstra's algorithm from `BinaryHeap` docs. |
| 16 | Packet Decoder  |  226  | 122 | 4 | `BitVec`, recusion and struct with children indexed in a `Vec`. |
| 17 | Trick Shot  |  370  | 523 | 232 | Grid-search, using `.map()` over (x, y) pairs. |

## Other notes

### Day 5

Seems slow. I've tried `ndarray` and `nalgebra` slices which are very similar and neat. Manual Matrix class is similar. I would guess a sparse matrix solution (like for d13) would be better.

### Day 13

Originally I used Termion, but in many cases it was printing over other terminal output.
`HashSet` slightly faster than `BTreeSet`.

### Day 14

The part two solution is of course faster, but I tried using an FM-index to do the counts in part 1, which works easily enough (although of
course these could have been computed on-the-fly).

### Day 16

- I was very frustrated by not being able to work out how to write a generic `bv_to_int` which would have been easy in C++. Writing `let x: T = 0` fails to compile!
- Not sure `BitVec` is quite what I think it is, need to check its underlying representation later. (Note: `u8` and `u64` template parameter made no difference to speed in this case).
- Avoided using `Rc` to point to other nodes in the graph which might have been nice. Was easier to find a way of avoiding using smart pointers, which wasn't hugely satisfying.

### Day 17

Regex is quite slow here.
