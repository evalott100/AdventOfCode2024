use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone)]
struct Guard {
    pub direction: Direction,
    pub path_taken: Vec<(Direction, usize, usize)>, // Used as a stack
}

impl Guard {
    fn new() -> Self {
        Guard {
            direction: Direction::Up,
            path_taken: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
enum Coordinate {
    Empty,
    Obstruction,
}

fn load_input(path: &str, guard: &mut Guard) -> Vec<Vec<Coordinate>> {
    let mut coordinates: Vec<Vec<Coordinate>> = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    for (y, line) in reader.lines().filter_map(|line| line.ok()).enumerate() {
        let mut row = Vec::new();
        for (x, raw_char) in line.chars().enumerate() {
            match raw_char {
                '.' => row.push(Coordinate::Empty),
                '#' => row.push(Coordinate::Obstruction),
                '^' => {
                    guard.direction = Direction::Up;
                    guard.path_taken.push((Direction::Up, x, y));
                    row.push(Coordinate::Empty);
                }
                '<' => {
                    guard.direction = Direction::Left;
                    guard.path_taken.push((Direction::Left, x, y));
                    row.push(Coordinate::Empty);
                }
                'v' => {
                    guard.direction = Direction::Down;
                    guard.path_taken.push((Direction::Down, x, y));
                    row.push(Coordinate::Empty);
                }
                '>' => {
                    guard.direction = Direction::Right;
                    guard.path_taken.push((Direction::Right, x, y));
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

fn turn_guard(guard: &mut Guard) {
    guard.direction = turn_direction(guard.direction);
}

fn guard_next_position(guard: &mut Guard) -> (i32, i32) {
    // i32 instead of usize because it can be negative
    match guard.direction {
        Direction::Up => (
            guard.path_taken.last().unwrap().1 as i32,
            (guard.path_taken.last().unwrap().2 as i32) - 1,
        ),
        Direction::Left => (
            (guard.path_taken.last().unwrap().1 as i32) - 1,
            guard.path_taken.last().unwrap().2 as i32,
        ),
        Direction::Down => (
            guard.path_taken.last().unwrap().1 as i32,
            (guard.path_taken.last().unwrap().2 as i32) + 1,
        ),
        Direction::Right => (
            (guard.path_taken.last().unwrap().1 as i32) + 1,
            guard.path_taken.last().unwrap().2 as i32,
        ),
    }
}

fn guard_in_bounds(x: i32, y: i32, x_bounds: usize, y_bounds: usize) -> bool {
    y <= y_bounds as i32 && y >= 0 && x <= x_bounds as i32 && x >= 0
}

fn walk_guard_until_obstacle(coordinates: &mut [Vec<Coordinate>], guard: &mut Guard) -> bool {
    // Returns true if the guard left the boundary, false if she entered a loop.
    loop {
        let (maybe_next_x, maybe_next_y) = guard_next_position(guard);
        if !guard_in_bounds(
            maybe_next_x,
            maybe_next_y,
            coordinates[0].len() - 1,
            coordinates.len() - 1,
        ) {
            return true;
        }

        let (next_x, next_y) = (maybe_next_x as usize, maybe_next_y as usize);

        if coordinates[next_y][next_x] == Coordinate::Obstruction {
            turn_guard(guard);
            continue;
        }
        if guard
            .path_taken
            .contains(&(guard.direction, next_x, next_y))
        {
            return false;
        }
        guard.path_taken.push((guard.direction, next_x, next_y));
    }
}

fn solution_1(coordinates: &mut Vec<Vec<Coordinate>>, guard: &mut Guard) -> i32 {
    walk_guard_until_obstacle(coordinates, guard);
    let unique_elements: HashSet<_> = guard.path_taken.iter().map(|(_, x, y)| (x, y)).collect();
    unique_elements.len() as i32
}

fn print_board(
    coordinates: &mut Vec<Vec<Coordinate>>,
    path_taken: Vec<(Direction, usize, usize)>,
    obstruction: (usize, usize),
) {
    let mut char_board: Vec<Vec<char>> = Vec::new();
    println!("===============================");
    for y in 0..coordinates.len() {
        char_board.push(Vec::new());
        for x in 0..coordinates[y].len() {
            char_board[y].push(match coordinates[y][x] {
                Coordinate::Empty => '.',
                Coordinate::Obstruction => '#',
            });
        }
    }

    for (direction, x, y) in path_taken.iter() {
        char_board[*y][*x] = match direction {
            Direction::Up => '|',
            Direction::Down => '|',
            Direction::Left => '-',
            Direction::Right => '-',
        }
    }

    char_board[obstruction.1][obstruction.0] = 'O';
    for row in char_board.iter() {
        for position in row.iter() {
            print!("{}", position);
        }
        println!();
    }
}

fn check_obstruction(
    coordinates: &mut Vec<Vec<Coordinate>>,
    guard: &mut Guard,
) -> HashSet<(usize, usize)> {
    let mut obstructions_to_add = HashSet::new();

    for index in 2..guard.path_taken.len() {
        let path_so_far: Vec<(Direction, usize, usize)> =
            guard.path_taken[0..index].iter().cloned().collect();

        let obstruction_x = guard.path_taken[index].1;
        let obstruction_y = guard.path_taken[index].2;

        coordinates[obstruction_y][obstruction_x] = Coordinate::Obstruction;
        let direction = guard.path_taken[index].0;

        let mut trial_guard = Guard {
            path_taken: path_so_far,
            direction,
        };

        if !walk_guard_until_obstacle(coordinates, &mut trial_guard) {
            obstructions_to_add.insert((obstruction_x, obstruction_y));
            print_board(
                coordinates,
                trial_guard.path_taken,
                (obstruction_x, obstruction_y),
            )
        }

        coordinates[obstruction_y][obstruction_x] = Coordinate::Empty;
    }

    obstructions_to_add
}

fn solution_2(coordinates: &mut Vec<Vec<Coordinate>>, guard: &mut Guard) -> i32 {
    // Giard already has the path taken
    check_obstruction(coordinates, guard).len() as i32
}

fn main() {
    let input_start = Instant::now();
    let mut guard: Guard = Guard::new();
    let mut coordinates = load_input("input.dat", &mut guard);
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&mut coordinates, &mut guard);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&mut coordinates, &mut guard);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
