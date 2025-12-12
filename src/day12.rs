use rayon::prelude::*;
use std::{fmt::Display, fs, time::Instant};

pub fn solve() -> impl Display {
    format!("Part 1: {}, Part 2: {}", solve_1(), solve_2())
}

#[cfg(feature = "debug")]
const FILE_NAME: &str = "inputs/day12-example.txt";

#[cfg(not(feature = "debug"))]
const FILE_NAME: &str = "inputs/day12.txt";

#[derive(Debug)]
struct Shape {
    pixels: Vec<Vec<bool>>,
    area: usize,
}

#[derive(Debug)]
struct Field {
    height: usize,
    width: usize,
    shape_reqs: Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Id,
    Cw,
    Hf,
    Ccw,
}

#[derive(Debug, Copy, Clone)]
enum Flip {
    Id,
    Ver,
    Hor,
}

impl Shape {
    fn get_pixel(&self, pos: (usize, usize), rotation: Rotation, flip: Flip) -> bool {
        let pos = match rotation {
            Rotation::Id => pos,
            Rotation::Cw => (pos.1, self.pixels.len() - pos.0 - 1),
            Rotation::Hf => (
                self.pixels.len() - pos.0 - 1,
                self.pixels[0].len() - pos.1 - 1,
            ),
            Rotation::Ccw => (self.pixels[0].len() - pos.1 - 1, pos.0),
        };

        let pos = match flip {
            Flip::Id => pos,
            Flip::Ver => (self.pixels.len() - pos.0 - 1, pos.1),
            Flip::Hor => (pos.0, self.pixels[0].len() - pos.1 - 1),
        };

        self.pixels[pos.0][pos.1]
    }
}

fn read_input_file() -> (Vec<Shape>, Vec<Field>) {
    let contents = fs::read_to_string(FILE_NAME).expect("No such file");

    let mut cur_shape: Vec<Vec<bool>> = vec![];
    let mut shapes = vec![];
    let mut fields = vec![];
    let mut area = 0;

    for line in contents.lines() {
        if line.ends_with(":") {
            continue;
        } else if line.starts_with("#") || line.starts_with(".") {
            cur_shape.push(line.chars().map(|c| c as u8 == b'#').collect());
            area += cur_shape.last().unwrap().iter().filter(|&&x| x).count();
        } else if line.is_empty() {
            shapes.push(Shape {
                pixels: cur_shape.clone(),
                area,
            });
            cur_shape = vec![];
            area = 0;
        } else if line.contains("x") {
            let (dims, shape_reqs) = line.split_once(":").expect("Invalid box format");
            let (width, height) = dims.split_once("x").expect("Invalid dims format");

            let (height, width) = (height.parse().unwrap(), width.parse().unwrap());
            let shape_reqs = shape_reqs
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            fields.push(Field {
                height,
                width,
                shape_reqs,
            });
        }
    }
    (shapes, fields)
}

fn intersects(
    shape: &Shape,
    field: &[Vec<bool>],
    pos: (usize, usize),
    rotation: Rotation,
    flip: Flip,
) -> bool {
    for (idx_i, line) in field[pos.0..pos.0 + shape.pixels.len()].iter().enumerate() {
        for (idx_j, &pixel) in line[pos.1..pos.1 + shape.pixels[0].len()]
            .iter()
            .enumerate()
        {
            if pixel && shape.get_pixel((idx_i, idx_j), rotation, flip) {
                return true;
            }
        }
    }

    false
}

fn fit_present(
    shape: &Shape,
    field: &mut [Vec<bool>],
    pos: (usize, usize),
    rotation: Rotation,
    flip: Flip,
) {
    for (idx_i, line) in field[pos.0..pos.0 + shape.pixels.len()]
        .iter_mut()
        .enumerate()
    {
        for (idx_j, pixel) in line[pos.1..pos.1 + shape.pixels[0].len()]
            .iter_mut()
            .enumerate()
        {
            *pixel |= shape.get_pixel((idx_i, idx_j), rotation, flip);
        }
    }
}

fn can_fit_all_presents(
    shapes: &[Shape],
    cur_field: &[Vec<bool>],
    shape_reqs: &[usize],
    area_left: usize,
) -> bool {
    if shape_reqs.iter().all(|&x| x == 0) {
        return true;
    }

    for (idx, &shapes_left) in shape_reqs.iter().enumerate() {
        if shapes_left == 0 {
            continue;
        }

        if area_left < shapes[idx].area {
            continue;
        }

        for pos_x in 0..=cur_field.len() - shapes[idx].pixels.len() {
            for pos_y in 0..=cur_field[pos_x].len() - shapes[idx].pixels[0].len() {
                for rotation in [Rotation::Id, Rotation::Cw, Rotation::Hf, Rotation::Ccw] {
                    for flip in [Flip::Id, Flip::Ver, Flip::Hor] {
                        if intersects(&shapes[idx], cur_field, (pos_x, pos_y), rotation, flip) {
                            continue;
                        }

                        let mut new_field = cur_field.to_vec();
                        fit_present(&shapes[idx], &mut new_field, (pos_x, pos_y), rotation, flip);
                        let mut new_shape_reqs = shape_reqs.to_vec();

                        new_shape_reqs[idx] -= 1;

                        if can_fit_all_presents(
                            shapes,
                            &new_field,
                            &new_shape_reqs,
                            area_left - shapes[idx].area,
                        ) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn solve_1() -> usize {
    let (shapes, fields) = read_input_file();

    fields
        .par_iter()
        .filter(|field| {
            let grid = vec![vec![false; field.width]; field.height];
            let start = Instant::now();
            let result = can_fit_all_presents(
                &shapes,
                &grid,
                &field.shape_reqs,
                field.height * field.width,
            );
            println!(
                "{:?} completed in {}mcs",
                field,
                start.elapsed().as_micros()
            );
            result
        })
        .count()
}

fn solve_2() -> usize {
    0
}
