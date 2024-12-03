use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_columns() -> Vec<Vec<i32>> {
    let mut columns: Vec<Vec<i32>> = Vec::new();

    if let Ok(content) = read_lines("src/1/input") {
        for raw_line in content.flatten() {
            let split_line: Vec<&str> = raw_line.split("   ").collect();
            for (i, part) in split_line.iter().enumerate() {
                if columns.len() <= i {
                    columns.push(Vec::new());
                }
                if let Ok(num) = part.trim().parse::<i32>() {
                    columns[i].push(num);
                } else {
                    eprintln!("Failed to parse '{}' as i32", part);
                }
            }
        }
    }
    columns
}

pub fn problem_1() {
    let columns = load_columns();
    let mut left = columns[0].clone();
    let mut right = columns[1].clone();
    left.sort();
    right.sort();

    let mut differences: Vec<i32> = Vec::new();
    for (l, r) in left.iter().zip(right.iter()) {
        differences.push((l - r).abs());
    }

    let sum: i32 = differences.iter().sum();
    println!("DIFFERENCES SUM: {}", sum);
}

pub fn problem_2() {
    let columns = load_columns();
    let left = columns[0].clone();
    let right = columns[1].clone();

    let mut occurrences: HashMap<i32, usize> = HashMap::new();

    for r in right.iter() {
        let counter = occurrences.entry(*r).or_insert(0);
        *counter += 1;
    }

    let mut scaled_left: Vec<i32> = Vec::new();
    for l in left.iter() {
        if let Some(occurence) = occurrences.get(l) {
            scaled_left.push(l * (*occurence as i32));
        } else {
            scaled_left.push(0);
        }
    }
    let sum: i32 = scaled_left.iter().sum();
    println!("SCALED SUM: {}", sum);
}
