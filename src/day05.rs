use std::{fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day05-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day05.txt";

#[derive(Debug)]
struct Input {
    ranges: Vec<(u64, u64)>,
    ingridients: Vec<u64>,
}

impl Input {
    pub fn compact(&mut self) {
        self.ranges.sort_unstable();

        let mut new_ranges: Vec<(u64, u64)> = vec![];

        for (start, end) in self.ranges.iter() {
            if new_ranges.is_empty() || *start > new_ranges.last().unwrap().1 {
                new_ranges.push((*start, *end));
            } else if *end > new_ranges.last().unwrap().1 {
                new_ranges.last_mut().unwrap().1 = *end;
            }
        }

        self.ingridients.sort_unstable();
        self.ranges = new_ranges;
    }

    pub fn fresh_count(&self) -> u64 {
        let mut result = 0;

        let mut range_index = 0;
        let mut ingr_index = 0;

        while ingr_index < self.ingridients.len() {
            while range_index < self.ranges.len()
                && self.ranges[range_index].1 < self.ingridients[ingr_index]
            {
                range_index += 1;
            }

            if range_index == self.ranges.len() {
                break;
            }

            if self.ranges[range_index].0 <= self.ingridients[ingr_index] {
                result += 1;
            }

            ingr_index += 1;
        }

        result
    }

    pub fn total_id_count(&self) -> u64 {
        self.ranges.iter().map(|(start, end)| end - start + 1).sum()
    }
}

fn read_input_file() -> Input {
    let contents = fs::read_to_string(FILE_NAME).expect("No such file");

    let mut ranges = vec![];
    let mut ingridients = vec![];
    let mut input_mode = 0;

    for line in contents.lines() {
        if line.is_empty() {
            input_mode = 1;
            continue;
        }

        match input_mode {
            0 => {
                let splitted = line.split_once("-").unwrap();
                ranges.push((splitted.0.parse().unwrap(), splitted.1.parse().unwrap()));
            }
            1 => ingridients.push(line.parse().unwrap()),
            _ => unreachable!(),
        }
    }

    let mut ans = Input {
        ranges,
        ingridients,
    };

    // dbg!(&ans);

    ans.compact();

    ans
}

fn solve_1() -> u64 {
    let input = read_input_file();
    dbg!(&input);

    input.fresh_count()
}

fn solve_2() -> u64 {
    let input = read_input_file();

    input.total_id_count()
}
