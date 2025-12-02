use std::{fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day02-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day02.txt";

fn read_input_file() -> Vec<Vec<i128>> {
    let contents = fs::read_to_string(FILE_NAME).expect("No such file");
    contents
        .replace("\n", "")
        .split(',')
        .map(|x| {
            x.split('-')
                .take(2)
                .map(|y| y.parse::<i128>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn check_repeating(num: i128) -> bool {
    let str_num = num.to_string();

    if !str_num.len().is_multiple_of(2) {
        return false;
    }

    let size = str_num.len() / 2;
    let first_half = &str_num[..size];
    let second_half = &str_num[size..];

    first_half == second_half
}

fn solve_1() -> i128 {
    let input = read_input_file();

    let mut result = 0_i128;

    for segment in input {
        let start = segment[0];
        let end = segment[1];

        for i in start..=end {
            if check_repeating(i) {
                result += i as i128;
            }
        }
    }

    result
}

fn check_repeating_at_least_2(num: i128) -> bool {
    let str_num = num.to_string();

    for i in 1..=(str_num.len() / 2) {
        if !str_num.len().is_multiple_of(i) {
            continue;
        }

        let prefix = &str_num[..i];

        if prefix.repeat(str_num.len() / i) == str_num {
            return true;
        }
    }

    false
}

fn solve_2() -> i128 {
    read_input_file()
        .iter()
        .map(|segment| {
            (segment[0]..=segment[1])
                .filter(|x| check_repeating_at_least_2(*x))
                .sum::<i128>()
        })
        .sum()
}
