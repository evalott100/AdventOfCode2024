use aoc_utils::Point;
use bmp::Image;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn new(x: i32, y: i32) -> Self {
        Velocity { x, y }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    position: Point,
    velocity: Velocity,
}

impl Robot {
    fn new(x: usize, y: usize, vx: i32, vy: i32) -> Self {
        Robot {
            position: Point { x, y },
            velocity: Velocity { x: vx, y: vy },
        }
    }
    fn from_string(s: &str) -> Self {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re
            .captures(s)
            .unwrap_or_else(|| panic!("Failed to parse: {}", s));
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let vx = caps[3].parse().unwrap();
        let vy = caps[4].parse().unwrap();
        Robot::new(x, y, vx, vy)
    }
    fn proceed(&mut self, grid_width: u8, grid_height: u8) {
        let new_position_x = self.position.x as i32 + self.velocity.x;
        let new_position_y = self.position.y as i32 + self.velocity.y;
        self.position = bound_point(new_position_x, new_position_y, grid_width, grid_height);
    }
}

fn move_robots(robots: &mut Vec<Robot>, grid_width: u8, grid_height: u8) {
    for robot in robots {
        robot.proceed(grid_width, grid_height);
    }
}

fn save_robot_as_bitmap(robots: &Vec<Robot>, grid_width: u8, grid_height: u8, seconds: u64) {
    let mut img = Image::new(grid_width as u32, grid_height as u32);
    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, bmp::consts::WHITE);
    }
    for robot in robots {
        img.set_pixel(
            robot.position.x as u32,
            robot.position.y as u32,
            bmp::consts::GREEN,
        );
    }
    img.save(format!("out/{:?}.bmp", seconds)).unwrap();
}

fn save_robot_to_text_file(robots: &Vec<Robot>, grid_width: u8, grid_height: u8, seconds: u64) {
    let mut grid = vec![vec![' '; grid_width as usize]; grid_height as usize];

    for robot in robots {
        grid[robot.position.y][robot.position.x] = '@';
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("out.dat")
        .unwrap();

    writeln!(file, "seconds {}", seconds);
    for row in grid {
        writeln!(file, "{}", row.iter().collect::<String>()).unwrap();
    }
}

fn load_input(path: &str) -> Vec<Robot> {
    // Should probably use regex here but CBA...
    let file = File::open(path).expect("file not found!");
    let reader = BufReader::new(file);

    let robots: Vec<Robot> = reader
        .lines()
        .map(|line| Robot::from_string(&line.unwrap()))
        .collect();

    robots
}

fn bound_point(mut point_x: i32, mut point_y: i32, grid_width: u8, grid_height: u8) -> Point {
    if point_x >= grid_width as i32 {
        point_x %= grid_width as i32;
    }
    if point_y >= grid_height as i32 {
        point_y %= grid_height as i32;
    }
    if point_x < 0 {
        point_x = grid_width as i32 + (point_x % grid_width as i32)
    }
    if point_y < 0 {
        point_y = grid_height as i32 + (point_y % grid_height as i32)
    }

    Point::new(point_x as usize, point_y as usize)
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Quadrant {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
    None,
}

impl Quadrant {
    fn from_point(point: Point, grid_width: u8, grid_height: u8) -> Quadrant {
        if point.x == (grid_width / 2) as usize || point.y == (grid_height / 2) as usize {
            return Quadrant::None;
        }
        if (point.x as f32) < (grid_width as f32 / 2.)
            && (point.y as f32) < (grid_height as f32 / 2.)
        {
            return Quadrant::TopLeft;
        }
        if (point.x as f32) < (grid_width as f32 / 2.)
            && (point.y as f32) > (grid_height as f32 / 2.)
        {
            return Quadrant::BottomLeft;
        }
        if (point.x as f32) > (grid_width as f32 / 2.)
            && (point.y as f32) < (grid_height as f32 / 2.)
        {
            return Quadrant::TopRight;
        }
        if (point.x as f32) > (grid_width as f32 / 2.)
            && (point.y as f32) > (grid_height as f32 / 2.)
        {
            return Quadrant::BottomRight;
        }
        panic!("FAILED TO MAP {:?} to a quadrant", point);
    }
}

const GRID_HEIGHT: u8 = 103;
const GRID_WIDTH: u8 = 101;

fn solution_1(mut robots: Vec<Robot>) -> u64 {
    for _ in 0..100 {
        move_robots(&mut robots, GRID_WIDTH, GRID_HEIGHT);
    }

    let mut quadrant_count: HashMap<Quadrant, usize> = HashMap::new();

    for robot in robots {
        let quadrant = Quadrant::from_point(robot.position, GRID_WIDTH, GRID_HEIGHT);
        if quadrant == Quadrant::None {
            continue;
        }
        quadrant_count
            .entry(quadrant)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut multiply: u64 = 1;
    for (_, count) in quadrant_count.iter() {
        multiply *= *count as u64;
    }
    multiply
}

fn solution_2(mut robots: Vec<Robot>) {
    for iteration in 0..10000 {
        save_robot_as_bitmap(&robots, GRID_WIDTH, GRID_HEIGHT, iteration);
        save_robot_to_text_file(&robots, GRID_WIDTH, GRID_HEIGHT, iteration);
        move_robots(&mut robots, GRID_WIDTH, GRID_HEIGHT);
    }
}

fn main() {
    let input_start = Instant::now();
    let robots = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(robots.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    solution_2(robots.clone());
    println!(
        "solution_2: {:?}, took {:?}",
        "look through yourself, you'll feel christmasy",
        solution_2_start.elapsed()
    );
}
