# advent-of-code-2021
Rust this time

## Timings

Run with `cargo run --release`. All times in µs.

| Day | Title | Parsing  | Part 1 | Part 2 | Notes |
| --- | ----- | -------- | ------ | ------ | ----- |
| 1 | Sonar Sweep |  255  | 25 | 8 | Sliding window. |
| 2 | Dive!  | 36  | 356 | 356 | Time is both parts together. |
| 3 | Binary Diagnostic  |  378  | 20 | 686 | `BitVec`; not working ideally for part 2. |
| 4 | Giant Squid  |  204  | 240 | 181 | `ndarray`, but suboptimal iteration. |
| 5 | Hydrothermal Venture  |  666  | 4879 | 1902 | `ndarray`: Slices for part 1; index for part 2. |
| 6 | Lanternfish  |  57  | 17 | 2 | State space model. `memcpy` faster than ptr move. |
| 7 | The Treachery of Whales  |  78  | 65 | 646 | Part 1 median. Part 2 `iter().map()`. |
| 8 | Seven Segment Search  |  440  | 21 | 1503 | Use `HashSet` for logic. |
| 9 | Smoke Basin  |  228  | 108 | 304 | Manual `Matrix` class. Part 2 recursive. |
| 10 | Syntax Scoring  |  79  | 123 | 61 | `Vec.pop()` to make a stack. |
| 11 | Dumbo Octopus  |  66  | 132 | 126 | Manual `OctoMatrix` class. Line scan to update. `octo_emoji` writes 🐙/✨. |
| 12 | Passage Pathing  |  275  | 256 | 7045 | Recursive DFS ignoring some nodes. |

