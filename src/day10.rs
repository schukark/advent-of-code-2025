use std::{collections::VecDeque, fmt::Display, fs};

use rayon::prelude::*;

use rustc_hash::FxHashSet;
use simple_tqdm::ParTqdm;

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day10-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day10.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    state: Vec<usize>,
    transitions: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn read_input_file() -> Vec<Machine> {
    let content = fs::read_to_string(FILE_NAME).expect("No such file");

    content
        .lines()
        .map(|line| {
            let (lights, remainder) = line.split_once(" ").expect("Invalid input format");
            let (transitions, joltage) = remainder.rsplit_once(" ").expect("Invalid input format");

            Machine {
                state: lights
                    .strip_prefix("[")
                    .expect("Invalid light format")
                    .strip_suffix("]")
                    .expect("Invalid light format")
                    .chars()
                    .map(|x| (x as u8 == b'#') as usize)
                    .collect(),
                transitions: transitions
                    .split_whitespace()
                    .map(|transition| {
                        transition
                            .strip_suffix(")")
                            .expect("Invalid transition format")
                            .strip_prefix("(")
                            .expect("Invalid transition format")
                            .split(",")
                            .map(|x| x.parse().unwrap())
                            .collect()
                    })
                    .collect(),
                joltage: joltage
                    .strip_suffix("}")
                    .expect("Invalid joltage format")
                    .strip_prefix("{")
                    .expect("Invalid joltage format")
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

trait Transition {
    fn transition(new_state: &mut usize);
    fn should_stop(machine: &Machine, cur_state: &[usize]) -> Decision;
}

struct LightCounter {}
struct JoltageCounter {}

enum Decision {
    Continue,
    ReturnAnswer,
    Skip,
}

impl Transition for LightCounter {
    fn transition(new_state: &mut usize) {
        *new_state ^= 1;
    }

    fn should_stop(_machine: &Machine, _cur_state: &[usize]) -> Decision {
        Decision::Continue
    }
}

impl Transition for JoltageCounter {
    fn transition(new_state: &mut usize) {
        *new_state += 1;
    }

    fn should_stop(machine: &Machine, cur_state: &[usize]) -> Decision {
        let goal_state = &machine.joltage;
        if goal_state == cur_state {
            return Decision::ReturnAnswer;
        }

        match goal_state
            .iter()
            .zip(cur_state.iter())
            .any(|(goal_value, cur_value)| goal_value < cur_value)
        {
            true => Decision::Skip,
            false => Decision::Continue,
        }
    }
}

fn find_smallest_press_count<T: Transition>(machine: &Machine) -> u16 {
    let mut deque = VecDeque::new();

    deque.push_back((0, vec![0; machine.state.len()]));
    let mut visited = FxHashSet::with_hasher(rustc_hash::FxBuildHasher);
    visited.insert(vec![0; machine.state.len()]);

    while let Some((top_dist, top)) = deque.pop_front() {
        match T::should_stop(&machine, &top) {
            Decision::Continue => {}
            Decision::ReturnAnswer => return top_dist,
            Decision::Skip => continue,
        };

        for transition in &machine.transitions {
            let mut new_state = top.clone();

            for light in transition {
                T::transition(&mut new_state[*light]);
            }

            if visited.contains(&new_state) {
                continue;
            }

            deque.push_back((top_dist + 1, new_state.clone()));
            visited.insert(new_state);
        }
    }

    0
}

fn solve_1() -> usize {
    let input = read_input_file();

    input
        .iter()
        .map(|machine| find_smallest_press_count::<LightCounter>(&machine) as usize)
        .sum()
}

fn solve_2() -> usize {
    let input = read_input_file();

    input
        .into_par_iter()
        .tqdm()
        .map(|machine| find_smallest_press_count::<JoltageCounter>(&machine) as usize)
        .sum()
}
