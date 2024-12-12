use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Rock {
    number: u64,
    multiplicity: u64,
}
impl Ord for Rock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Rock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Rock {
    fn new(number: u64, multiplicity: u64) -> Rock {
        Rock {
            number,
            multiplicity,
        }
    }

    fn blink(self) -> Vec<Rock> {
        if self.number == 0 {
            return Vec::from([Rock::new(1, self.multiplicity)]);
        }

        let string_number = self.number.to_string();
        if string_number.len() % 2 == 0 {
            return Vec::from([
                Rock::new(
                    string_number[string_number.len() / 2..string_number.len()]
                        .parse::<u64>()
                        .expect("failed to parse number"),
                    self.multiplicity,
                ),
                Rock::new(
                    string_number[..string_number.len() / 2]
                        .parse::<u64>()
                        .expect("failed to parse number"),
                    self.multiplicity,
                ),
            ]);
        }

        Vec::from([Rock::new(self.number * 2024, self.multiplicity)])
    }
}

#[derive(Clone)]
struct Rocks {
    rocks: BTreeSet<Rock>,
}

impl Rocks {
    fn new() -> Rocks {
        Rocks {
            rocks: BTreeSet::new(),
        }
    }

    fn insert(&mut self, rock: Rock) {
        if let Some(&existing_rock) = self.rocks.get(&rock) {
            self.rocks.insert(Rock::new(
                rock.number,
                rock.multiplicity + existing_rock.multiplicity,
            ));
        } else {
            self.rocks.insert(rock);
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Rock> {
        self.rocks.iter()
    }

    fn blink_n(mut rocks: Rocks, number_of_blinks: usize) -> Rocks {
        for _ in 0..number_of_blinks {
            let mut new_rocks: Rocks = Rocks::new();
            for rock in rocks.iter() {
                for new_rock in rock.blink() {
                    new_rocks.insert(new_rock);
                }
            }
            rocks = new_rocks
        }

        rocks
    }
}

fn load_input(path: &str) -> Rocks {
    let mut rocks: Rocks = Rocks::new();

    let raw_rocks = read_to_string(path).expect("failed to read file");
    let rock_vector: Vec<Rock> = raw_rocks
        .split_whitespace()
        .map(|s| Rock::new(s.parse::<u64>().expect("failed to parse number"), 1))
        .collect();

    for rock in rock_vector.iter() {
        rocks.insert(*rock);
    }

    rocks
}

fn number_of_rocks_after_n_blinks(number_of_blinks: usize, rocks: Rocks) -> u64 {
    let mut sum: u64 = 0;
    let blunk_rocks = Rocks::blink_n(rocks, number_of_blinks);
    for rock in blunk_rocks.iter() {
        sum += rock.multiplicity;
    }
    sum
}

fn solution_1(rocks: Rocks) -> u64 {
    number_of_rocks_after_n_blinks(25, rocks)
}

fn solution_2(rocks: Rocks) -> u64 {
    number_of_rocks_after_n_blinks(75, rocks)
}

fn main() {
    let input_start = Instant::now();
    let rocks: Rocks = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(rocks.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    // Too low 65601038650482
    let solution_2_start = Instant::now();
    let output_2 = solution_2(rocks.clone());
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
