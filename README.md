# Advent of code 2025 solutions

## Ideas

1. Simple loop over the dial rotations (no arithmetic), fast enough for this (2ms)
2. Bruteforce, fast enough (112ms)
3. 2D-dp, dp\[i\]\[j\] = the biggest number of `i` digits made from digits in the prefix of length `j`, optimal (335ms)
4. Simple bruteforce, nothing clever here (6ms)
5. Single-pass algorithm with pre-sorting/range-compactization, then 2 pointers (8ms)
6. Input manipulation, not very Rusty code (no inherent invariants for Input type held), good enough for AoC (3ms)
