use std::{collections::HashMap, fmt::Display, fs};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day11-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day11.txt";

#[derive(Debug)]
struct Graph {
    vertex_names: HashMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum GraphError {
    AlreadyExists(usize),
    NoSuchNode(String),
}

impl Graph {
    fn new() -> Self {
        Self {
            vertex_names: HashMap::new(),
            edges: vec![],
        }
    }

    fn add_vertex(&mut self, vertex: &str) -> Result<(), GraphError> {
        if let Some(&value) = self.vertex_names.get(vertex) {
            Err(GraphError::AlreadyExists(value))
        } else {
            self.vertex_names
                .insert(vertex.to_owned(), self.vertex_names.len());
            self.edges.push(vec![]);
            Ok(())
        }
    }

    fn add_edge(&mut self, start: &str, end: &str) -> Result<(), GraphError> {
        if let Some(&start_index) = self.vertex_names.get(start) {
            if let Some(&end_index) = self.vertex_names.get(end) {
                self.edges[start_index].push(end_index);
                Ok(())
            } else {
                Err(GraphError::NoSuchNode(end.to_owned()))
            }
        } else {
            Err(GraphError::NoSuchNode(start.to_owned()))
        }
    }
}

fn read_input_file() -> Graph {
    let contents = fs::read_to_string(FILE_NAME).expect("No such file");

    let mut graph = Graph::new();

    contents.lines().for_each(|line| {
        let (start, _remainder) = line.split_once(":").unwrap();

        graph.add_vertex(start).unwrap();
    });

    graph.add_vertex("out").unwrap();

    contents.lines().for_each(|line| {
        let (start, remainder) = line.split_once(":").unwrap();

        remainder.split_whitespace().for_each(|vertex| {
            graph.add_edge(start, vertex).unwrap();
        });
    });

    graph
}

fn topological_sort(graph: &Graph) -> Result<Vec<usize>, &'static str> {
    let mut answer = vec![];
    let mut no_incoming_edges = vec![];

    let mut indegree = vec![0; graph.vertex_names.len()];

    for (_start_idx, edge_ends) in graph.edges.iter().enumerate() {
        for &edge_idx in edge_ends {
            indegree[edge_idx] += 1;
        }
    }

    for (idx, value) in indegree.iter().enumerate() {
        if *value == 0 {
            no_incoming_edges.push(idx);
        }
    }

    while let Some(vertex) = no_incoming_edges.pop() {
        answer.push(vertex);

        for &edge_end in &graph.edges[vertex] {
            indegree[edge_end] -= 1;

            if indegree[edge_end] == 0 {
                no_incoming_edges.push(edge_end);
            }
        }
    }

    if indegree.iter().any(|x| *x != 0) {
        Err("Graph contains cycles")
    } else {
        Ok(answer)
    }
}

fn count_paths(graph: &Graph, topological_order: &[usize], src: &str, dst: &str) -> usize {
    let source = *graph.vertex_names.get(src).unwrap();
    let destination = *graph.vertex_names.get(dst).unwrap();

    let mut paths = vec![0; graph.vertex_names.len()];
    paths[source] = 1;

    for &vertex in topological_order {
        for &neighbor in &graph.edges[vertex] {
            paths[neighbor] += paths[vertex];
        }
    }

    paths[destination]
}

fn solve_1() -> usize {
    let input = read_input_file();

    let topological_order = topological_sort(&input).unwrap();

    count_paths(&input, &topological_order, "you", "out")
}

fn solve_2() -> usize {
    let input = read_input_file();

    let top = topological_sort(&input).unwrap();

    let dac_fft = count_paths(&input, &top, "svr", "dac")
        * count_paths(&input, &top, "dac", "fft")
        * count_paths(&input, &top, "fft", "out");

    let fft_dac = count_paths(&input, &top, "svr", "fft")
        * count_paths(&input, &top, "fft", "dac")
        * count_paths(&input, &top, "dac", "out");

    dac_fft + fft_dac
}
