use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{zip, Iterator};
use std::time::Instant;

fn load_input(path: &str) -> Vec<Vec<char>> {
    let input = File::open(path).expect("failed to open file");
    let buffered = BufReader::new(input);
    buffered
        .lines()
        .map(|line| line.expect("failed to read line").chars().collect())
        .collect()
}

fn char_at(input: &[Vec<char>], x: i32, y: i32) -> char {
    // I found a Vec<Vec<char>> was much fater than a Vec<&str>
    // once unwrapping and indexing was taken into account.
    *input.get(y as usize).unwrap().get(x as usize).unwrap()
}

enum Direction {
    Positive,
    Negative,
    Constant,
}

struct MasDirection {
    current: i32,
    end: i32,
    direction: Direction,
}

impl MasDirection {
    // TODO: Simplify this logic by adding a scale field.
    // current -> end will always be positive, but the outputted
    // iterated number will be scaled (by 1, 0, -1).
    //
    fn new(direction: Direction) -> Self {
        let current = 1;
        let end = 4;
        match direction {
            Direction::Positive => MasDirection {
                current,
                end,
                direction,
            },
            Direction::Negative => MasDirection {
                current: -1,
                end: -4,
                direction,
            },
            Direction::Constant => MasDirection {
                current: 0,
                end: 0,
                direction,
            },
        }
    }
}

impl Iterator for MasDirection {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::Positive => {
                if self.end > self.current {
                    let result = self.current;
                    self.current += 1;
                    Some(result)
                } else {
                    None
                }
            }
            Direction::Negative => {
                if self.current > self.end {
                    let result = self.current;
                    self.current -= 1;
                    Some(result)
                } else {
                    None
                }
            }
            Direction::Constant => Some(self.current),
        }
    }
}

fn word_search(
    input: &[Vec<char>],
    x: i32,
    y: i32,
    x_iter: MasDirection,
    y_iter: MasDirection,
) -> bool {
    if y + y_iter.end + 1 < 0 || (input.len() as i32) < y + y_iter.end {
        return false;
    }
    if x + x_iter.end + 1 < 0 || (input[y as usize].len() as i32) < x + x_iter.end {
        return false;
    }

    for (mas_index, (x_diff, y_diff)) in zip(x_iter.into_iter(), y_iter.into_iter()).enumerate() {
        if char_at(input, x + x_diff, y + y_diff) != ['M', 'A', 'S'][mas_index] {
            return false;
        }
    }

    true
}

fn solution_1(input: &[Vec<char>]) -> i32 {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if char_at(input, x as i32, y as i32) == 'X' {
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Positive),
                    MasDirection::new(Direction::Constant),
                ) as i32; // Right
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Negative),
                    MasDirection::new(Direction::Constant),
                ) as i32; // Left
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Constant),
                    MasDirection::new(Direction::Positive),
                ) as i32; // Up
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Constant),
                    MasDirection::new(Direction::Negative),
                ) as i32; // Down
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Positive),
                    MasDirection::new(Direction::Positive),
                ) as i32; // Top right diag
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Negative),
                    MasDirection::new(Direction::Negative),
                ) as i32; // Bottom left diag
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Negative),
                    MasDirection::new(Direction::Positive),
                ) as i32; // Top left diag
                count += word_search(
                    input,
                    x as i32,
                    y as i32,
                    MasDirection::new(Direction::Positive),
                    MasDirection::new(Direction::Negative),
                ) as i32; // Bottom right diag
            }
        }
    }
    count
}

fn is_xmas(input: &[Vec<char>], x: i32, y: i32) -> bool {
    if y == 0
        || x == 0
        || y == (input.len() as i32) - 1
        || x == (input[y as usize].len() - 1) as i32
    {
        return false;
    }

    let up_left = char_at(input, x - 1, y - 1);
    let up_right = char_at(input, x + 1, y - 1);
    let down_left = char_at(input, x - 1, y + 1);
    let down_right = char_at(input, x + 1, y + 1);
    [
        ('M', 'S', 'M', 'S'),
        ('M', 'S', 'S', 'M'),
        ('S', 'M', 'M', 'S'),
        ('S', 'M', 'S', 'M'),
    ]
    .contains(&(up_left, down_right, up_right, down_left))
}

fn solution_2(input: &[Vec<char>]) -> i32 {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if char_at(input, x as i32, y as i32) == 'A' {
                count += is_xmas(input, x as i32, y as i32) as i32;
            }
        }
    }
    count
}

fn main() {
    let input_start = Instant::now();
    let input = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&input);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&input);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
