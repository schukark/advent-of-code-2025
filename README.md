# Advent of code 2025 solutions

## Ideas

1. Simple loop over the dial rotations (no arithmetic), fast enough for this (2ms)
2. Bruteforce, fast enough (112ms)
3. 2D-dp, dp\[i\]\[j\] = the biggest number of `i` digits made from digits in the prefix of length `j`, optimal (335ms)
4. Simple bruteforce, nothing clever here (6ms)
