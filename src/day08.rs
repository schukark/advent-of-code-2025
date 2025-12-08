use std::{collections::HashMap, fmt::Display, fs, hash::Hash};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day08-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day08.txt";

#[derive(Debug)]
struct DSU<T: Eq + Hash> {
    parent: HashMap<T, T>,
    size: HashMap<T, usize>,
}

impl<T: Eq + Hash> DSU<T> {
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }
}

impl<T: Eq + Hash + Clone> DSU<T> {
    pub fn make_set(&mut self, vertex: &T) {
        self.parent.insert(vertex.clone(), vertex.clone());
        self.size.insert(vertex.clone(), 1);
    }

    pub fn find_set(&mut self, vertex: &T) -> T {
        if let Some(value) = self.parent.get(&vertex)
            && value == vertex
        {
            return vertex.clone();
        }

        let answer = self.find_set(&self.parent.get(vertex).cloned().unwrap());
        *self.parent.get_mut(vertex).unwrap() = answer.clone();
        answer
    }

    pub fn union_sets(&mut self, vertex_a: &T, vertex_b: &T) {
        let mut a = self.find_set(vertex_a);
        let mut b = self.find_set(vertex_b);

        if a != b {
            if self.size.get(&a) < self.size.get(&b) {
                std::mem::swap(&mut a, &mut b);
            }

            self.parent.insert(b.clone(), a.clone());
            let b_size = *self.size.get(&b).unwrap();
            *self.size.get_mut(&a).unwrap() += b_size;
        }
    }

    pub fn biggest_n(&self, n: usize) -> Vec<usize> {
        let mut values: Vec<_> = self.size.values().copied().collect();
        values.sort();
        values.iter().rev().take(n).copied().collect()
    }
}

fn read_input_file() -> Vec<Vec<u32>> {
    let content = fs::read_to_string(FILE_NAME).expect("No such file");

    content
        .lines()
        .map(|line| {
            line.split(",")
                .take(3)
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn distance_sqr(vec_a: &[u32], vec_b: &[u32]) -> u64 {
    assert_eq!(vec_a.len(), vec_b.len());

    let mut sum = 0;

    for i in 0..vec_a.len() {
        sum += (vec_a[i] as i64 - vec_b[i] as i64).pow(2);
    }

    sum as u64
}

#[cfg(feature = "debug")]
const SHORTEST_CONNECTIONS: usize = 10;

#[cfg(not(feature = "debug"))]
const SHORTEST_CONNECTIONS: usize = 1000;

fn solve_1() -> usize {
    let input = read_input_file();
    let mut distances = Vec::new();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let dist = distance_sqr(&input[i], &input[j]);

            distances.push((dist, i, j));
        }
    }

    distances.sort_unstable();

    let mut dsu = DSU::new();

    (0..input.len()).for_each(|idx| {
        dsu.make_set(&idx);
    });

    for i in 0..SHORTEST_CONNECTIONS {
        let point_a = distances[i].1;
        let point_b = distances[i].2;

        if dsu.find_set(&point_a) == dsu.find_set(&point_b) {
            continue;
        }
        // println!("Connecting {:?} and {:?}", input[point_a], input[point_b]);

        dsu.union_sets(&point_a, &point_b);
    }

    // dbg!(dsu.biggest_n(3));

    dsu.biggest_n(3).into_iter().product()
}

fn solve_2() -> usize {
    let input = read_input_file();
    let mut distances = Vec::new();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let dist = distance_sqr(&input[i], &input[j]);

            distances.push((dist, i, j));
        }
    }

    distances.sort_unstable();

    let mut dsu = DSU::new();

    (0..input.len()).for_each(|idx| {
        dsu.make_set(&idx);
    });

    let mut i = 0;

    loop {
        let point_a = distances[i].1;
        let point_b = distances[i].2;

        if dsu.find_set(&point_a) == dsu.find_set(&point_b) {
            i += 1;
            continue;
        }

        dsu.union_sets(&point_a, &point_b);

        if dsu.biggest_n(1)[0] == input.len() {
            return input[point_a][0] as usize * input[point_b][0] as usize;
        }
    }
}
