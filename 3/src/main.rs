use regex::Regex;
use std::fs;
use std::time::Instant;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

fn solution_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: i32 = 0;

    for capture in re.captures_iter(input) {
        sum += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
    }
    sum
}

fn solution_2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut do_flag: bool = true;
    let mut sum: i32 = 0;

    for capture in re.captures_iter(input) {
        match (do_flag, &capture[0]) {
            (_, "do()") => {
                do_flag = true;
            }
            (_, "don't()") => {
                do_flag = false;
            }
            (true, _) => {
                sum += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
            }
            (_, _) => {}
        }
    }
    sum
}

fn main() {
    let input_start = Instant::now();
    let input = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&input);
    println!(
        "solution_1: {}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&input);
    println!(
        "solution_2: {}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
