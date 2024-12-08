use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, PartialEq, Eq)]
enum Coordinate {
    Empty,
    Obstruction,
}

type Coordinates = Vec<Vec<Coordinate>>;
type PathTaken = Vec<(Direction, usize, usize)>;

fn load_input(path: &str, path_taken: &mut PathTaken) -> Coordinates {
    let mut coordinates: Coordinates = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    for (y, line) in reader.lines().filter_map(|line| line.ok()).enumerate() {
        let mut row = Vec::new();
        for (x, raw_char) in line.chars().enumerate() {
            match raw_char {
                '.' => row.push(Coordinate::Empty),
                '#' => row.push(Coordinate::Obstruction),
                '^' => {
                    path_taken.push((Direction::Up, x, y));
                    row.push(Coordinate::Empty);
                }
                '<' => {
                    path_taken.push((Direction::Left, x, y));
                    row.push(Coordinate::Empty);
                }
                'v' => {
                    path_taken.push((Direction::Down, x, y));
                    row.push(Coordinate::Empty);
                }
                '>' => {
                    path_taken.push((Direction::Right, x, y));
                    row.push(Coordinate::Empty);
                }
                _ => panic!("invalid character in input"),
            }
        }
        coordinates.push(row);
    }
    coordinates
}

fn turn_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
    }
}

fn next_position(direction: Direction, x: usize, y: usize) -> (i32, i32) {
    // i32 instead of usize because it can be negative
    match direction {
        Direction::Up => (x as i32, y as i32 - 1),
        Direction::Left => (x as i32 - 1, y as i32),
        Direction::Down => (x as i32, y as i32 + 1),
        Direction::Right => (x as i32 + 1, y as i32),
    }
}

fn guard_in_bounds(x: i32, y: i32, x_bounds: usize, y_bounds: usize) -> bool {
    y <= y_bounds as i32 && y >= 0 && x <= x_bounds as i32 && x >= 0
}

fn walk_guard(coordinates: &Coordinates, path_taken: &mut PathTaken) -> bool {
    loop {
        // Returns true if the guard left the boundary, false if she entered a loop.
        let (mut direction, current_x, current_y) = {
            let last = path_taken.last().unwrap();
            (last.0, last.1, last.2)
        };

        let (maybe_next_x, maybe_next_y) = next_position(direction, current_x, current_y);

        if !guard_in_bounds(
            maybe_next_x,
            maybe_next_y,
            coordinates[0].len() - 1,
            coordinates.len() - 1,
        ) {
            return true;
        }

        let (mut next_x, mut next_y) = (maybe_next_x as usize, maybe_next_y as usize);

        if path_taken.contains(&(direction, next_x, next_y)) {
            return false;
        }

        if coordinates[next_y][next_x] == Coordinate::Obstruction {
            // A turn counts as a move on the path, but we don't advance the guard in x, y.
            (direction, next_x, next_y) = (turn_direction(direction), current_x, current_y);
        }

        path_taken.push((direction, next_x, next_y));
    }
}

fn solution_1(coordinates: &Coordinates, path_taken: &mut PathTaken) -> i32 {
    walk_guard(coordinates, path_taken);
    let unique_elements: HashSet<_> = path_taken.iter().map(|(_, x, y)| (x, y)).collect();
    unique_elements.len() as i32
}

fn check_obstruction(coordinates: &Coordinates, path_taken: &PathTaken) -> HashSet<(usize, usize)> {
    path_taken
        .par_iter()
        .enumerate()
        .skip(2)
        .filter_map(|(index, (_, obstruction_x, obstruction_y))| {
            let mut coordinates_copy = coordinates.clone();
            let mut path_so_far: Vec<(Direction, usize, usize)> = path_taken[0..index].to_vec();

            if path_so_far
                .iter()
                .filter(|(_, x, y)| x == obstruction_x && y == obstruction_y)
                .count()
                > 0
            {
                return None;
            }

            coordinates_copy[*obstruction_y][*obstruction_x] = Coordinate::Obstruction;

            if !walk_guard(&coordinates_copy, &mut path_so_far) {
                return Some((*obstruction_x, *obstruction_y));
            }
            None
        })
        .collect()
}

fn solution_2(coordinates: &Coordinates, path_taken: &mut PathTaken) -> i32 {
    // Giard already has the path taken
    check_obstruction(coordinates, path_taken).len() as i32
}

fn main() {
    let input_start = Instant::now();
    let mut path_taken: PathTaken = Vec::new();
    let coordinates = load_input("input.dat", &mut path_taken);
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&coordinates, &mut path_taken);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&coordinates, &mut path_taken);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
