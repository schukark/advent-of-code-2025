use std::{collections::VecDeque, fmt::Display, fs};

use rayon::prelude::*;
use simple_tqdm::ParTqdm;

use rustc_hash::FxHashSet;
use z3::{Solver, ast::Int};

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

fn find_smallest_press_count(machine: &Machine) -> u16 {
    let mut deque = VecDeque::new();

    deque.push_back((0, vec![0; machine.state.len()]));
    let mut visited = FxHashSet::with_hasher(rustc_hash::FxBuildHasher);
    visited.insert(vec![0; machine.state.len()]);

    while let Some((top_dist, top)) = deque.pop_front() {
        if top == machine.state {
            return top_dist;
        }

        for transition in &machine.transitions {
            let mut new_state = top.clone();

            for light in transition {
                new_state[*light] ^= 1;
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
        .map(|machine| find_smallest_press_count(&machine) as usize)
        .sum()
}

fn solve_z3(machine: &Machine) -> u64 {
    let solver = Solver::new();

    let mut coefficients = Vec::with_capacity(machine.transitions.len());

    for i in 0..machine.transitions.len() {
        coefficients.push(Int::fresh_const(&i.to_string()));
        solver.assert(coefficients[i].ge(0));
    }

    for i in 0..machine.joltage.len() {
        let coefs: Vec<_> = machine
            .transitions
            .iter()
            .enumerate()
            .filter(|(_idx, transition)| transition.contains(&i))
            .map(|(idx, _transition)| &coefficients[idx])
            .cloned()
            .collect();

        let mut expr = coefs[0].clone();

        for coef in coefs.iter().skip(1) {
            expr = &expr + coef;
        }

        solver.assert(expr.eq(machine.joltage[i] as u64));
    }

    solver
        .solutions(coefficients, false)
        .map(|solution| solution.iter().map(Int::as_u64).map(Option::unwrap).sum())
        .min()
        .unwrap()
}

fn solve_2() -> u64 {
    let input = read_input_file();

    input
        .par_iter()
        .tqdm()
        .map(|machine| solve_z3(machine))
        .sum()
}
