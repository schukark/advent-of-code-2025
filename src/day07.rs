use std::{collections::HashSet, fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day07-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day07.txt";

fn read_input_file() -> Vec<Vec<char>> {
    let content = fs::read_to_string(FILE_NAME).expect("No such file");

    content.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for line in grid {
        for elem in line {
            print!("{}", elem);
        }
        println!("");
    }
    println!("");
}

fn solve_1() -> u128 {
    let mut input = read_input_file();
    let start = find_start(&input).expect("No start found");

    let mut stack = vec![start];
    let mut splits = 0;
    let mut visited = HashSet::new();

    while let Some(top) = stack.pop() {
        if top.0 == input.len() {
            continue;
        }

        // print_grid(&input);

        if input[top.0][top.1] == '^' {
            if visited.contains(&top) {
                continue;
            }

            visited.insert(top);

            let left = (top.0, top.1 - 1);
            let right = (top.0, top.1 + 1);

            if input[left.0][left.1] != '|' || input[right.0][right.1] != '|' {
                splits += 1;
            }

            stack.push(left);
            stack.push(right);
        } else {
            stack.push((top.0 + 1, top.1));
            input[top.0][top.1] = '|';
        }
    }

    splits
}

fn solve_2() -> u128 {
    let mut input = read_input_file();

    let mut dp = vec![vec![0; input[0].len()]; input.len()];
    let start = find_start(&input).expect("Found no start");

    dp[start.0][start.1] = 1;
    input[start.0][start.1] = '|';

    for i in start.0..input.len() - 1 {
        for j in 0..input[0].len() {
            if input[i + 1][j] == '^' {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j - 1] += dp[i][j];

                input[i + 1][j + 1] = '|';
                input[i + 1][j - 1] = '|';
            } else if input[i][j] == '|' {
                dp[i + 1][j] += dp[i][j];
                input[i + 1][j] = '|';
            }
        }
    }

    // print_grid(&input);

    dp.last().unwrap().iter().sum()
}
