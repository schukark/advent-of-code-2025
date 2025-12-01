use std::{fmt::Display, fs};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Incorrect direction literal".into()),
        }
    }
}

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

fn read_input_file() -> Vec<(Direction, i32)> {
    let file_contents = fs::read_to_string("inputs/day01.txt").expect("No such file");
    let mut result_vec = Vec::new();

    for line in file_contents.lines() {
        result_vec.push((
            line.chars().next().unwrap().try_into().unwrap(),
            line.chars().skip(1).collect::<String>().parse().unwrap(),
        ));
    }

    result_vec
}

fn solve_1() -> i32 {
    let input = read_input_file();

    let mut dial = 50;
    let mut result = 0;

    for (direction, count) in input {
        dial = match direction {
            Direction::Right => (dial + count) % 100,
            Direction::Left => (dial - count).rem_euclid(100),
        };

        if dial == 0 {
            result += 1;
        }
    }

    result
}

fn solve_2() -> i32 {
    todo!()
}
