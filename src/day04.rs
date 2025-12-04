use std::{fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day04-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day04.txt";

fn read_input_file() -> Vec<Vec<bool>> {
    let contents = fs::read_to_string(FILE_NAME).expect("No such file");

    contents
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_neighbors(grid: &[Vec<bool>], pos: (usize, usize)) -> i32 {
    const MAT: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, -1),
        (-1, -1),
    ];

    let mut result = 0;

    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    for (dx, dy) in MAT {
        let (new_x, new_y) = (pos.0 as isize + dx, pos.1 as isize + dy);

        if new_x < 0 || new_x >= n || new_y < 0 || new_y >= m {
            continue;
        }

        if grid[new_x as usize][new_y as usize] {
            result += 1;
        }
    }

    result
}

fn solve_1() -> i128 {
    let grid = read_input_file();

    let mut result = 0;

    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        for j in 0..m {
            if !grid[i][j] {
                continue;
            }

            if count_neighbors(&grid, (i, j)) < 4 {
                result += 1;
            }
        }
    }

    result
}

fn solve_2() -> i128 {
    let mut grid = read_input_file();

    let mut result = 0;

    let n = grid.len();
    let m = grid[0].len();

    let mut changed = true;

    while changed {
        changed = false;

        let mut new_grid = grid.clone();

        for i in 0..n {
            for j in 0..m {
                if !grid[i][j] {
                    continue;
                }

                if count_neighbors(&grid, (i, j)) < 4 {
                    changed = true;
                    new_grid[i][j] = false;
                    result += 1;
                }
            }
        }

        grid = new_grid;
    }

    result
}
