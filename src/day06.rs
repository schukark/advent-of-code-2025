use std::{fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day06-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day06.txt";

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<&str> for Op {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err("Invalid op".into()),
        }
    }
}

impl Op {
    fn identity(&self) -> u128 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

#[derive(Debug)]
struct Input {
    values: Vec<Vec<u128>>,
    ops: Vec<Op>,
}

impl Input {
    fn calculate_horizontal(&self) -> Vec<u128> {
        let mut answer = Vec::with_capacity(self.values[0].len());

        for i in 0..self.values[0].len() {
            let mut result = self.ops[i].identity();

            for j in 0..self.values.len() {
                match self.ops[i] {
                    Op::Add => result += self.values[j][i],
                    Op::Mul => result *= self.values[j][i],
                }
            }

            answer.push(result);
        }

        answer
    }

    fn calculate_vertical(&self) -> Vec<u128> {
        let mut answer = Vec::with_capacity(self.values.len());
        for i in 0..self.values.len() {
            let mut result = self.ops[i].identity();

            for j in 0..self.values[i].len() {
                match self.ops[i] {
                    Op::Add => result += self.values[i][j],
                    Op::Mul => result *= self.values[i][j],
                }
            }

            answer.push(result);
        }

        answer
    }
}

fn read_input_file_horizontal() -> Input {
    let contents = fs::read_to_string(FILE_NAME).expect("No such file");

    let mut values = vec![];
    let mut ops = vec![];

    for line in contents.lines() {
        if line.contains("*") || line.contains("+") {
            ops = line
                .split_whitespace()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            continue;
        }

        values.push(
            line.split_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect(),
        );
    }

    Input { values, ops }
}

fn read_input_file_vertically() -> Input {
    let binding = fs::read_to_string(FILE_NAME).expect("No such file");
    let contents: Vec<_> = binding.lines().collect();

    let mut values = vec![vec![]];

    for i in 0..contents[0].len() {
        let mut cur_number = 0;
        for j in 0..contents.len() - 1 {
            if contents[j].chars().nth(i).unwrap() == ' ' {
                continue;
            }

            cur_number =
                cur_number * 10 + (contents[j].chars().nth(i).unwrap() as u8 - b'0') as u128;
        }

        if cur_number == 0 {
            values.push(vec![]);
        } else {
            values.last_mut().unwrap().push(cur_number);
        }
    }

    let ops = contents
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.try_into())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    Input { values, ops }
}

fn solve_1() -> u128 {
    let input = read_input_file_horizontal();

    input.calculate_horizontal().iter().sum()
}

fn solve_2() -> u128 {
    let input = read_input_file_vertically();

    input.calculate_vertical().iter().sum()
}
