use itertools::{repeat_n, Itertools};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn load_input(path: &str) -> Vec<(u64, Vec<u64>)> {
    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines().filter_map(|line| line.ok()) {
        let mut parts = line.splitn(2, ':');
        let result_raw = parts.next().unwrap_or("");
        let result: u64 = result_raw.parse().unwrap();

        let inputs_raw = parts.next().unwrap_or("");
        let inputs: Vec<u64> = inputs_raw
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        equations.push((result, inputs));
    }
    equations
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatinate,
}

impl Operation {
    fn apply(&self, number_1: u64, number_2: u64) -> u64 {
        match self {
            Operation::Add => number_1 + number_2,
            Operation::Multiply => number_1 * number_2,
            Operation::Concatinate => {
                let mut number_1_copy = number_1;
                let mut number_2_copy = number_2;
                while number_2_copy > 0 {
                    number_1_copy *= 10;
                    number_2_copy /= 10;
                }
                number_1_copy + number_2
            }
        }
    }
}

fn equation_could_be_true(equation: &(u64, &Vec<u64>), operations: &[Operation]) -> bool {
    let (result, inputs) = equation;

    for operations_permutation in repeat_n(operations, inputs.len() - 1).multi_cartesian_product() {
        let mut sum = inputs[0];
        for (input_index, operation) in operations_permutation.into_iter().enumerate() {
            sum = operation.apply(sum, inputs[input_index + 1]);
        }

        if sum == *result {
            return true;
        }
    }

    false
}

fn solution_1(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut total_calibration_result = 0;
    for (result, inputs) in equations.iter() {
        if equation_could_be_true(&(*result, inputs), &[Operation::Add, Operation::Multiply]) {
            total_calibration_result += result;
        }
    }

    total_calibration_result
}

fn solution_2(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut total_calibration_result = 0;
    for (result, inputs) in equations.iter() {
        if equation_could_be_true(
            &(*result, inputs),
            &[Operation::Add, Operation::Multiply, Operation::Concatinate],
        ) {
            total_calibration_result += result;
        }
    }

    total_calibration_result
}

fn main() {
    let input_start = Instant::now();
    let equations = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&equations);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );
    let solution_2_start = Instant::now();
    let output_2 = solution_2(&equations);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
