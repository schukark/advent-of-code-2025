use std::{fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day03-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day03.txt";

fn read_input_file() -> Vec<String> {
    fs::read_to_string(FILE_NAME)
        .expect("No such file")
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

fn find_biggest_battery_2(line: &str) -> i32 {
    let mut biggest = 11;

    for i in 0..line.len() {
        for j in i + 1..line.len() {
            let cur_digit = (line.chars().nth(i).unwrap() as u8 - b'0') * 10
                + (line.chars().nth(j).unwrap() as u8 - b'0');

            if cur_digit as i32 > biggest {
                biggest = cur_digit as i32;
            }
        }
    }

    biggest
}

fn solve_1() -> i32 {
    read_input_file()
        .iter()
        .map(|x| find_biggest_battery_2(&x))
        .sum()
}

fn find_biggest_battery_12(line: &str) -> i128 {
    // dp[i][j] = the biggest number of j digits made up from the prefix of length i
    let mut dp = vec![[0; 13]; line.len() + 1];
    // dp[i][j] = max(dp[i][j], max_{k \in 0..i- 1} dp[k][j - 1] ## a[i - 1]) for i > 0

    for i in 1..=line.len() {
        for j in 1..=12 {
            let mut maximum = dp[i][j];

            for k in 0..=i - 1 {
                maximum = i128::max(
                    maximum,
                    dp[k][j - 1] * 10 + (line.chars().nth(i - 1).unwrap() as u8 - b'0') as i128,
                );
            }

            dp[i][j] = maximum;
        }
    }

    (1..=line.len()).map(|i| dp[i][12]).max().unwrap()
}

fn solve_2() -> i128 {
    read_input_file()
        .iter()
        .map(|x| find_biggest_battery_12(&x))
        .sum()
}
