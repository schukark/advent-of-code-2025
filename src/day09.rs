use std::{fmt::Display, fs, str::FromStr};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day09-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day09.txt";

fn read_input_file<T>() -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let content = fs::read_to_string(FILE_NAME).expect("No such file");

    content
        .lines()
        .map(|line| {
            line.split(",")
                .take(2)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_1() -> u128 {
    let mut best = 0;
    let input = read_input_file();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let area = (u128::abs_diff(input[i][0], input[j][0]) + 1)
                * (u128::abs_diff(input[i][1], input[j][1]) + 1);

            best = u128::max(best, area);
        }
    }

    best
}

fn solve_2() -> u128 {
    0
}
