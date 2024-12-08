use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type Antenna = char;
type AntennaGrid = Vec<Vec<Antenna>>;
const EMPTY_ANTENNA: Antenna = '.';

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn in_bounds(&self, x_bounds: i32, y_bounds: i32) -> bool {
        self.x >= 0 && self.x <= x_bounds && self.y >= 0 && self.y <= y_bounds
    }
}

impl From<(usize, usize)> for Position {
    fn from(coords: (usize, usize)) -> Self {
        Position::new(coords.0 as i32, coords.1 as i32)
    }
}

impl From<(Position, Position)> for Position {
    fn from(positions: (Position, Position)) -> Self {
        Position::new(
            2 * positions.1.x - positions.0.x,
            2 * positions.1.y - positions.0.y,
        )
    }
}

fn load_input(path: &str) -> AntennaGrid {
    let mut antenna_grid: AntennaGrid = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|line| line.ok()) {
        antenna_grid.push(line.chars().collect());
    }

    antenna_grid
}

fn calculate_antenna_antinodes(
    antenna: Antenna,
    position: Position,
    antenna_grid: &AntennaGrid,
) -> HashSet<Position> {
    let mut antinode_locations: HashSet<Position> = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for other_position in x_vector.iter().enumerate().filter_map(|(x, coordinate)| {
            let other_position = Position::from((x, y));
            if *coordinate == antenna && other_position != position {
                Some(other_position)
            } else {
                None
            }
        }) {
            let calculated = Position::from((position, other_position));

            if calculated.in_bounds((x_vector.len() - 1) as i32, (antenna_grid.len() - 1) as i32) {
                antinode_locations.insert(calculated);
            }
        }
    }

    antinode_locations
}

fn calculate_antenna_antinodes_repeating(
    antenna: Antenna,
    position: Position,
    antenna_grid: &AntennaGrid,
) -> HashSet<Position> {
    let mut antinode_locations: HashSet<Position> = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for other_position in x_vector.iter().enumerate().filter_map(|(x, coordinate)| {
            if *coordinate == antenna {
                let other_position = Position::from((x, y));
                Some(other_position)
            } else {
                None
            }
        }) {
            let mut old = position;
            let mut new = other_position;

            if new == old {
                antinode_locations.insert(other_position);
                continue;
            }

            loop {
                let calculated = Position::from((old, new));

                if !calculated
                    .in_bounds((x_vector.len() - 1) as i32, (antenna_grid.len() - 1) as i32)
                {
                    break;
                }

                antinode_locations.insert(calculated);

                if calculated == new {
                    // Only one node on the antenna itself
                    break;
                }

                old = new;
                new = calculated;
            }
        }
    }

    antinode_locations
}

fn solution_1(antenna_grid: &AntennaGrid) -> u32 {
    let mut antinode_locations = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for (x, antenna) in x_vector.iter().enumerate() {
            if *antenna != EMPTY_ANTENNA {
                antinode_locations.extend(calculate_antenna_antinodes(
                    *antenna,
                    Position::from((x, y)),
                    antenna_grid,
                ))
            }
        }
    }

    antinode_locations.len() as u32
}

fn solution_2(antenna_grid: &AntennaGrid) -> u32 {
    let mut antinode_locations = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for (x, antenna) in x_vector.iter().enumerate() {
            if *antenna != EMPTY_ANTENNA {
                antinode_locations.extend(calculate_antenna_antinodes_repeating(
                    *antenna,
                    Position::from((x, y)),
                    antenna_grid,
                ))
            }
        }
    }

    antinode_locations.len() as u32
}

fn main() {
    let input_start = Instant::now();
    let antenna_grid: AntennaGrid = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&antenna_grid);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&antenna_grid);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
