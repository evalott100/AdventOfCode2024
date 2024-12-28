use aoc_utils::{Direction, Grid, Point};
use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Clone, Debug)]
struct PathNode {
    path: Vec<(Direction, Point)>,
    direction: Direction,
    sub_paths: Vec<PathNode>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Coordinate {
    Obstruction,
    Start,
    End,
    Empty,
}

pub trait ToStringGrid {
    fn to_string_grid(&self) -> String;
    fn to_string_grid_with_path(&self, path: &Vec<(Direction, Point)>) -> String;
}

// Implement the trait for Grid<Coordinate>
impl ToStringGrid for Grid<Coordinate> {
    fn to_string_grid(&self) -> String {
        let mut result = String::new();

        for row in self.rows.iter() {
            for coord in row.iter() {
                let c = match coord {
                    Coordinate::Obstruction => '#',
                    Coordinate::Empty => '.',
                    Coordinate::End => 'E',
                    Coordinate::Start => 'S',
                };
                result.push(c);
            }
            result.push('\n');
        }

        result
    }
    fn to_string_grid_with_path(&self, path: &Vec<(Direction, Point)>) -> String {
        let mut string_grid: Vec<String> = Vec::new();

        for row in self.rows.iter() {
            let mut string_row: String = "".to_string();
            for coord in row.iter() {
                let c = match coord {
                    Coordinate::Obstruction => '#',
                    Coordinate::Empty => '.',
                    Coordinate::End => 'E',
                    Coordinate::Start => 'S',
                };
                string_row.push(c);
            }
            string_grid.push(string_row);
        }

        for (direction, point) in path.iter() {
            let c = match direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            };
            let mut chars: Vec<char> = string_grid[point.y].chars().collect();
            chars[point.x] = c;
            string_grid[point.y] = chars.into_iter().collect();
        }
        string_grid.join("\n")
    }
}

pub trait Turn {
    fn turn_right(&self) -> Self;
    fn turn_left(&self) -> Self;
}

impl Turn for Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

fn load_input(path: &str) -> Grid<Coordinate> {
    let file = File::open(path).expect("file not found!");
    let reader = BufReader::new(file);

    let mut coordinates = VecDeque::new();

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    for line in lines.iter() {
        if line.starts_with('#') {
            let row: VecDeque<Coordinate> = line
                .chars()
                .map(|c| match c {
                    '#' => Coordinate::Obstruction,
                    '.' => Coordinate::Empty,
                    'E' => Coordinate::End,
                    'S' => Coordinate::Start,
                    _ => panic!("Unkown char in grid"),
                })
                .collect();
            coordinates.push_back(row);
        } else {
            break;
        }
    }

    Grid::new(coordinates)
}

fn get_paths(
    grid: &Grid<Coordinate>,
    start: Point,
    direction: Direction,
    visited: &mut BTreeSet<Point>,
) -> PathNode {
    let mut path_node = PathNode {
        path: Vec::new(),
        direction,
        sub_paths: Vec::new(),
    };

    let mut point = start;

    loop {
        if grid[point] == Coordinate::Obstruction {
            break;
        }

        visited.insert(point);

        for turned_direction in [direction.turn_left(), direction.turn_right()].iter() {
            let turned_point = point.adjacent(*turned_direction);
            if grid[turned_point] != Coordinate::Obstruction {
                let sub_path = get_paths(grid, turned_point, *turned_direction, visited);
                if !sub_path.path.is_empty() {
                    path_node.sub_paths.push(sub_path);
                }
            }
        }

        path_node.path.push((direction, point));
        point = point.adjacent(direction);
        if visited.contains(&point) || visited.contains(&point) {
            break;
        }
    }

    path_node
}

fn get_ending_paths(grid: &Grid<Coordinate>, path_node: PathNode) -> Vec<Vec<(Direction, Point)>> {
    let mut paths = Vec::new();

    if grid[path_node.path.last().unwrap().1] == Coordinate::End {
        paths.push(path_node.path.clone());
    }

    for sub_path_node in path_node.sub_paths.iter() {
        for sub_path in get_ending_paths(grid, sub_path_node.clone()) {
            if sub_path.is_empty() {
                continue;
            }
            let mut path_copy = path_node.path.clone();
            path_copy.extend(sub_path);
            paths.push(path_copy);
        }
    }
    paths
}

fn count_path_values(path: &Vec<(Direction, Point)>) -> u64 {
    let mut sum: u64 = 0;
    let mut path_iter = path.iter();
    let (mut last_direction, mut last_point) = path_iter.next().unwrap();
    for (direction, point) in path_iter {
        if *direction != last_direction {
            sum += 1000;
        }
        if *point != last_point {
            sum += 1;
        }
        (last_direction, last_point) = (*direction, *point);
    }
    sum
}

fn solution_1(grid: Grid<Coordinate>) -> u64 {
    let mut visited = BTreeSet::new();
    let mut start: Option<Point> = None;
    for (y_index, row) in grid.rows.iter().enumerate() {
        for (x_index, coordinate) in row.iter().enumerate() {
            if *coordinate == Coordinate::Start {
                start = Some(Point::new(x_index, y_index));
            }
        }
    }
    let paths = get_paths(&grid, start.unwrap(), Direction::Right, &mut visited);

    let ending_paths = get_ending_paths(&grid, paths);

    for path in ending_paths.iter() {
        println!(
            "SCORE {:?}, \n{}",
            count_path_values(path),
            grid.to_string_grid_with_path(path)
        );
    }

    0
}

fn main() {
    let input_start = Instant::now();
    let grid = load_input("input_simple.dat");
    println!("input took {:?}", input_start.elapsed());
    println!("{}", grid.to_string_grid());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(grid.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    // let solution_2_start = Instant::now();
    // let output_2 = solution_2(doubled_grid.clone(), directions.clone());
    //
    // println!(
    //     "solution_2: {:?}, took {:?}",
    //     output_2,
    //     solution_2_start.elapsed()
    // );
}
