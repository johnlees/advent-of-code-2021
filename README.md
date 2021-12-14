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
| 13 | Transparent Origami  |  609  | 102 | 347 | `BTreeSet` for paper. Termion to print letters. |
| 14 | Extended polymerization  |  127  | 2440 | 440 | Part 1: brute force + FM-index. Part 2: `BTreeMap` of 2-mers. |

## Other notes

### Day 5

Seems slow. I've tried `ndarray` and `nalgebra` slices which are very similar and neat. Manual Matrix class is similar. I would guess a sparse matrix solution (like for d13) would be better.

### Day 13

The part two solution is of course faster, but I tried using an FM-index to do the counts in part 1, which works easily enough (although of
course these could have been computed on-the-fly).
