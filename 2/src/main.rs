use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_input(path: &str) -> Vec<Vec<i32>> {
    let input = File::open(path).expect("Failed to open file");
    let buffered = BufReader::new(input);
    let mut data: Vec<Vec<i32>> = Vec::new();

    for line in buffered.lines() {
        match line {
            Ok(line) => {
                let numbers: Vec<i32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();
                data.push(numbers);
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
            }
        }
    }

    data
}

fn check_row(row: &Vec<i32>) -> bool {
    let mut iterator = row.iter();
    let mut previous_value: &i32 = iterator.next().unwrap();
    let mut increasing: Option<bool> = None;

    for number in iterator {
        if number > previous_value {
            if increasing == Some(false) { return false; }
            increasing = Some(true);
        }
        else if number < previous_value {
            if increasing == Some(true) {return false;}
            increasing = Some(false);
        }
        else {return false;}

        if (number - previous_value).abs() > 3 {return false;}
        previous_value = number;
    }
    true
}

fn check_row_lenient(row: &Vec<i32>) -> bool {
    if check_row(row) {
        return true;
    }
    else {
        for i in 0..row.len() {
            let mut new_row = row.clone();
            new_row.remove(i);
            if check_row(&new_row) {
                return true;
            }
        }
    }
    false
}

fn problem_1(input_data: &Vec<Vec<i32>>) {
    let sum: usize = input_data.iter().filter(|row| check_row(row)).count();

    println!("solution 1: {}", sum);
}

fn problem_2(input_data: &Vec<Vec<i32>>) {
    let sum: usize = input_data.iter().filter(|row| check_row_lenient(row)).count();
    println!("solution 2: {}", sum);
}

fn main() {
    let input_data = load_input("input");
    problem_1(&input_data);
    problem_2(&input_data);
}
