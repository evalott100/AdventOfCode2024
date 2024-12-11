use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Rock {
    number: u64,
}

impl Rock {
    fn new(number: u64) -> Rock {
        Rock { number }
    }

    fn blink(self) -> Vec<Rock> {
        if self.number == 0 {
            return Vec::from([Rock::new(1)]);
        }

        let string_number = self.number.to_string();
        if string_number.len() % 2 == 0 {
            return Vec::from([
                Rock::new(
                    string_number[string_number.len() / 2..string_number.len()]
                        .parse::<u64>()
                        .expect("failed to parse number"),
                ),
                Rock::new(
                    string_number[..string_number.len() / 2]
                        .parse::<u64>()
                        .expect("failed to parse number"),
                ),
            ]);
        }

        Vec::from([Rock::new(self.number * 2024)])
    }
}

#[derive(Clone)]
struct Rocks {
    rocks: HashMap<Rock, u64>,
}

impl Rocks {
    fn new() -> Rocks {
        Rocks {
            rocks: HashMap::new(),
        }
    }

    fn insert(&mut self, rock: Rock, new_multiplicity: u64) {
        self.rocks
            .entry(rock)
            .and_modify(|multiplicity| *multiplicity += new_multiplicity)
            .or_insert(new_multiplicity);
    }

    fn iter(&self) -> impl Iterator<Item = (&Rock, &u64)> {
        self.rocks.iter()
    }
}

fn load_input(path: &str) -> Rocks {
    let mut rocks: Rocks = Rocks::new();

    let raw_rocks = read_to_string(path).expect("failed to read file");
    let rock_vector: Vec<Rock> = raw_rocks
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("failed to parse number"))
        .map(Rock::new)
        .collect();

    for rock in rock_vector.iter() {
        rocks.insert(*rock, 1);
    }

    rocks
}

fn blink_n(number_of_blinks: usize, mut rocks: Rocks) -> Rocks {
    for _ in 0..number_of_blinks {
        let mut blunk_rocks: Rocks = Rocks::new();
        for (rock, multiplicity) in rocks.iter() {
            for new_rock in rock.blink().iter() {
                blunk_rocks.insert(*new_rock, *multiplicity);
            }
        }

        rocks = blunk_rocks;
    }
    rocks
}

fn number_of_rocks_after_n_blinks(number_of_blinks: usize, rocks: Rocks) -> u64 {
    let mut sum: u64 = 0;
    for (_, multiplicity) in blink_n(number_of_blinks, rocks).iter() {
        sum += multiplicity;
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
