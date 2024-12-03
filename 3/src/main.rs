use regex::Regex;
use std::fs;

fn solution_1(path: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let content = fs::read_to_string(path).expect("Failed to read file");
    let mut sum: i32 = 0;

    for cap in re.captures_iter(&content) {
        let n: i32 = cap[1].parse().unwrap();
        let m: i32 = cap[2].parse().unwrap();
        sum += n * m;
    }
    sum
}

fn solution_2(path: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let content = fs::read_to_string(path).expect("Failed to read file");
    let mut do_flag: bool = true;
    let mut sum: i32 = 0;

    for cap in re.captures_iter(&content) {
        if let Some(mul_cap) = cap.get(1) {
            if !do_flag {
                continue;
            }
            let n: i32 = mul_cap.as_str().parse().unwrap();
            let m: i32 = cap[2].parse().unwrap();
            sum += n * m;
        } else {
            do_flag = cap.get(0).unwrap().as_str() == "do()";
        }
    }
    sum
}

fn main() {
    println!("solution_1: {}", solution_1("input.dat"));
    println!("solution_2: {}", solution_2("input.dat"));
}
