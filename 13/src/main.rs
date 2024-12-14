use aoc_utils::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn is_integer(num: f64) -> bool {
    num.fract() == 0.0
}
#[derive(Debug, Copy, Clone)]
struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn new(x: usize, y: usize) -> Self {
        Button { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Prize {
    x: usize,
    y: usize,
}

fn load_input(path: &str) -> Vec<(Prize, Button, Button)> {
    // Should probably use regex here but CBA...
    let file = File::open(path).expect("file not found!");
    let reader = BufReader::new(file);

    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("Button A:")
            || line.starts_with("Button B:")
            || line.starts_with("Prize:")
        {
            let point = parse_coords(&line);
            points.push(point);
        }
    }

    let mut prizes: Vec<(Prize, Button, Button)> = Vec::new();

    for point in points.chunks(3) {
        prizes.push((
            Prize {
                x: point[2].x,
                y: point[2].y,
            },
            Button::new(point[0].x, point[0].y),
            Button::new(point[1].x, point[1].y),
        ));
    }

    prizes
}

fn parse_coords(line: &str) -> Point {
    let line = line
        .trim_start_matches("Button A: ")
        .trim_start_matches("Button B: ")
        .trim_start_matches("Prize:");
    let parts: Vec<&str> = line.split_whitespace().collect();
    let x_part = parts[0]
        .trim_start_matches("X")
        .trim_start_matches('+')
        .trim_start_matches('=')
        .trim_end_matches(',');
    let y_part = parts[1]
        .trim_start_matches("Y")
        .trim_start_matches('+')
        .trim_start_matches('=')
        .trim_end_matches(',');

    let x = x_part.parse::<usize>().unwrap();
    let y = y_part.parse::<usize>().unwrap();
    Point::new(x, y)
}

fn get_button_press_solutions(
    prize: Prize,
    button_a: Button,
    button_b: Button,
) -> Option<(u64, u64)> {
    let a = button_a.x as f64;
    let b = button_b.x as f64;
    let c = button_a.y as f64;
    let d = button_b.y as f64;
    let determinant = a * d - b * c;

    if determinant == 0. {
        return None;
    }

    let a_presses = (prize.x as f64 * d - prize.y as f64 * b) / determinant;
    let b_presses = (prize.y as f64 * a - prize.x as f64 * c) / determinant;

    if !is_integer(a_presses) || !is_integer(b_presses) {
        return None;
    }

    Some((a_presses as u64, b_presses as u64))
}

fn solution_1(prizes: Vec<(Prize, Button, Button)>) -> u32 {
    let mut sum = 0;
    for machine in prizes.iter() {
        let prize = machine.0;
        let button_a = machine.1;
        let button_b = machine.2;
        sum += get_button_press_solutions(prize, button_a, button_b)
            .iter()
            .map(|(a, b)| (3 * *a) + *b)
            .min()
            .unwrap_or(0) as u32;
    }
    sum
}

const INCREMENT: usize = 10000000000000;

fn solution_2(prizes: Vec<(Prize, Button, Button)>) -> u64 {
    let mut sum = 0;
    for machine in prizes.iter() {
        let button_a = machine.1;
        let button_b = machine.2;
        let prize = Prize {
            x: machine.0.x + INCREMENT,
            y: machine.0.y + INCREMENT,
        };
        sum += get_button_press_solutions(prize, button_a, button_b)
            .iter()
            .map(|(a, b)| (3 * *a) + *b)
            .min()
            .unwrap_or(0) as u64;
    }
    sum
}

fn main() {
    let input_start = Instant::now();
    let prizes = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(prizes.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(prizes.clone());

    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
