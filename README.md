# Advent of code 2025 solutions

## Ideas

1. Simple loop over the dial rotations (no arithmetic), fast enough for this (2ms)
2. Bruteforce, fast enough (112ms)
3. 2D-dp, dp\[i\]\[j\] = the biggest number of `i` digits made from digits in the prefix of length `j`, optimal (335ms)
4. Simple bruteforce, nothing clever here (6ms)
5. Single-pass algorithm with pre-sorting/range-compactization, then 2 pointers (8ms)
6. Input manipulation, not very Rusty code (no inherent invariants for Input type held), good enough for AoC (3ms)
7. First part is easy simulation of the process, 2nd part is 2D-dp, dp\[`i`\]\[`j`\] = number of timelines that could get here from the start (0ms)
8. DSU for near O(1) performance, nothing fancy with distances, O(n^2). (36 ms)
9. First path is O(n^2) bruteforce, second part is ? (? ms)
10. First path is BFS, second part is ? (? ms)
11. Simple path-counting via topsort for both paths (0 ms)
