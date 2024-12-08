use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type Antenna = char;
type AntennaGrid = Vec<Vec<Antenna>>;
const EMPTY_ANTENNA: Antenna = '.';

fn load_input(path: &str) -> AntennaGrid {
    let mut antenna_uuids: HashMap<char, Antenna> = HashMap::from([('.', EMPTY_ANTENNA)]);
    let mut antenna_grid: AntennaGrid = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|line| line.ok()) {
        let mut antenna_grid_y = Vec::new();
        for character in line.chars() {
            let antenna = antenna_uuids.entry(character).or_insert(character);
            antenna_grid_y.push(*antenna);
        }
        antenna_grid.push(antenna_grid_y);
    }
    antenna_grid
}

fn in_bounds(x: i32, y: i32, x_bounds: i32, y_bounds: i32) -> bool {
    x >= 0 && x <= x_bounds && y >= 0 && y <= y_bounds
}

fn calculate_new_position(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> (i32, i32) {
    (2 * x_2 - x_1, 2 * y_2 - y_1)
}

fn calculate_antinodes_with_antenna(
    antenna: Antenna,
    antenna_x: usize,
    antenna_y: usize,
    antenna_grid: &AntennaGrid,
) -> HashSet<(usize, usize)> {
    let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for (x, coordinate) in x_vector.iter().enumerate() {
            if (x, y) == (antenna_x, antenna_y) || *coordinate != antenna {
                continue;
            }

            let (old_x, old_y) = (antenna_x as i32, antenna_y as i32);
            let (new_x, new_y) = (x as i32, y as i32);

            let (calculated_x, calculated_y) = calculate_new_position(old_x, old_y, new_x, new_y);

            if in_bounds(
                calculated_x,
                calculated_y,
                (x_vector.len() - 1) as i32,
                (antenna_grid.len() - 1) as i32,
            ) {
                antinode_locations.insert((calculated_x as usize, calculated_y as usize));
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
                antinode_locations.extend(calculate_antinodes_with_antenna(
                    *antenna,
                    x,
                    y,
                    antenna_grid,
                ))
            }
        }
    }
    antinode_locations.len() as u32
}

fn calculate_antinodes_repeating(
    antenna_x: usize,
    antenna_y: usize,
    antenna_grid: &AntennaGrid,
) -> HashSet<(usize, usize)> {
    let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for (x, coordinate) in x_vector.iter().enumerate() {
            if *coordinate == EMPTY_ANTENNA || (x, y) == (antenna_x, antenna_y) {
                continue;
            }

            let (mut old_x, mut old_y) = (antenna_x as i32, antenna_y as i32);
            let (mut new_x, mut new_y) = (x as i32, y as i32);

            loop {
                let (calculated_x, calculated_y) =
                    calculate_new_position(old_x, old_y, new_x, new_y);

                if !in_bounds(
                    calculated_x,
                    calculated_y,
                    (x_vector.len() - 1) as i32,
                    (antenna_grid.len() - 1) as i32,
                ) {
                    break;
                }

                //println!("found {}, {}", calculated_x + 1, calculated_y + 1);
                antinode_locations.insert((calculated_x as usize, calculated_y as usize));

                if (calculated_x, calculated_y) == (new_x, new_y) {
                    // Only one node on the antenna itself
                    break;
                }

                (old_x, old_y) = (new_x, new_y);
                (new_x, new_y) = (calculated_x, calculated_y);
            }
        }
    }
    antinode_locations
}

fn solution_2(antenna_grid: &AntennaGrid) -> u32 {
    let mut antinode_locations = HashSet::new();

    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for (x, antenna) in x_vector.iter().enumerate() {
            if *antenna != EMPTY_ANTENNA {
                antinode_locations.extend(calculate_antinodes_repeating(x, y, antenna_grid))
            }
        }
    }
    for (y, x_vector) in antenna_grid.iter().enumerate() {
        for (x, antenna) in x_vector.iter().enumerate() {
            if *antenna == EMPTY_ANTENNA {
                if antinode_locations.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!("{}", EMPTY_ANTENNA);
                }
            } else {
                print!("{}", antenna);
            }
        }
        println!();
    }
    antinode_locations.len() as u32
}

fn main() {
    let input_start = Instant::now();
    let antenna_grid: AntennaGrid = load_input("input_simple.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&antenna_grid);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    // NOT 901 (too low)
    let solution_2_start = Instant::now();
    let output_2 = solution_2(&antenna_grid);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
