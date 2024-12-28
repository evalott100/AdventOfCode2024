use aoc_utils::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Clone, Eq, PartialEq, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Coordinate {
    Box,
    Obstruction,
    Robot,
    Empty,
    LeftBox,
    RightBox,
}

#[derive(Clone, Debug)]
struct Grid {
    coordinates: Vec<Vec<Coordinate>>,
    robot: Point,
}

fn point_of_direction(point: Point, direction: Direction) -> Point {
    match direction {
        Direction::Up => point.up(),
        Direction::Down => point.down(),
        Direction::Left => point.left(),
        Direction::Right => point.right(),
    }
}

impl Grid {
    fn new(coordinates: Vec<Vec<Coordinate>>, robot: Point) -> Self {
        Grid { coordinates, robot }
    }

    fn move_box(&mut self, direction: Direction, point: Point) -> bool {
        assert_eq!(self.coordinates[point.y][point.x], Coordinate::Box);
        let move_point = point_of_direction(point, direction);

        if self.coordinates[move_point.y][move_point.x] == Coordinate::Box
            && !self.move_box(direction, move_point)
        {
            return false;
        }

        if self.coordinates[move_point.y][move_point.x] == Coordinate::Empty {
            self.coordinates[point.y][point.x] = Coordinate::Empty;
            self.coordinates[move_point.y][move_point.x] = Coordinate::Box;
            return true;
        }

        false
    }

    fn move_double_box(
        &mut self,
        direction: Direction,
        left_point: Point,
        right_point: Point,
    ) -> bool {
        // Ugly but I'm sleepy

        let left_move_point = point_of_direction(left_point, direction);
        let right_move_point = point_of_direction(right_point, direction);

        if self.coordinates[left_move_point.y][left_move_point.x] == Coordinate::Obstruction
            || self.coordinates[right_move_point.y][right_move_point.x] == Coordinate::Obstruction
        {
            return false;
        }

        if (self.coordinates[left_move_point.y][left_move_point.x] != Coordinate::Empty
            || self.coordinates[right_move_point.y][right_move_point.x] != Coordinate::Empty)
            && !self.move_double_box(direction, left_move_point, right_move_point)
        {
            return false;
        }
        if self.coordinates[left_move_point.y][left_move_point.x] == Coordinate::Empty
            && self.coordinates[right_move_point.y][right_move_point.x] == Coordinate::Empty
        {
            self.coordinates[left_point.y][left_point.x] = Coordinate::Empty;
            self.coordinates[right_point.y][right_point.x] = Coordinate::Empty;
            self.coordinates[left_move_point.y][left_move_point.x] = Coordinate::LeftBox;
            self.coordinates[right_move_point.y][right_move_point.x] = Coordinate::RightBox;
            return true;
        }
        false
    }

    fn move_robot_to_empty(&mut self, move_point: Point) {
        assert_eq!(
            self.coordinates[move_point.y][move_point.x],
            Coordinate::Empty
        );
        self.coordinates[self.robot.y][self.robot.x] = Coordinate::Empty;
        self.coordinates[move_point.y][move_point.x] = Coordinate::Robot;
        self.robot = move_point;
    }

    fn move_robot(&mut self, direction: Direction) {
        let move_point = point_of_direction(self.robot, direction);

        if self.coordinates[move_point.y][move_point.x] == Coordinate::Empty {
            self.move_robot_to_empty(move_point)
        } else if self.coordinates[move_point.y][move_point.x] == Coordinate::Box
            && self.move_box(direction, move_point)
        {
            self.move_robot_to_empty(move_point);
        } else if self.coordinates[move_point.y][move_point.x] == Coordinate::LeftBox
            && self.move_double_box(
                direction,
                move_point,
                Point::new(move_point.x + 1, move_point.y),
            )
        {
            self.move_robot_to_empty(move_point);
        } else if self.coordinates[move_point.y][move_point.x] == Coordinate::RightBox
            && self.move_double_box(
                direction,
                Point::new(move_point.x - 1, move_point.y),
                move_point,
            )
        {
            self.move_robot_to_empty(move_point);
        }
    }

    fn double(self) -> Self {
        let mut doubled_coordinates = Vec::new();
        for row in self.coordinates.iter() {
            let mut doubled_row = Vec::new();
            for coordinate in row.iter() {
                match coordinate {
                    Coordinate::Obstruction => {
                        doubled_row.push(Coordinate::Obstruction);
                        doubled_row.push(Coordinate::Obstruction);
                    }
                    Coordinate::Robot => {
                        doubled_row.push(Coordinate::Robot);
                        doubled_row.push(Coordinate::Empty);
                    }
                    Coordinate::Empty => {
                        doubled_row.push(Coordinate::Empty);
                        doubled_row.push(Coordinate::Empty);
                    }
                    Coordinate::Box => {
                        doubled_row.push(Coordinate::LeftBox);
                        doubled_row.push(Coordinate::RightBox);
                    }
                    _ => panic!("Already doubled!"),
                }
            }
            doubled_coordinates.push(doubled_row);
        }

        let doubled_robot = Point::new(self.robot.x * 2, self.robot.y);
        Grid::new(doubled_coordinates, doubled_robot)
    }
}

impl Into<String> for Grid {
    fn into(self) -> String {
        let mut result = String::new();

        for (y, row) in self.coordinates.iter().enumerate() {
            for (x, coord) in row.iter().enumerate() {
                let c = match coord {
                    Coordinate::Obstruction => '#',
                    Coordinate::LeftBox => '[',
                    Coordinate::RightBox => ']',
                    Coordinate::Box => 'O',
                    Coordinate::Robot => '@',
                    Coordinate::Empty => '.',
                };
                result.push(c);
            }
            result.push('\n');
        }

        result
    }
}

type Directions = Vec<Direction>;

fn load_input(path: &str) -> (Grid, Directions) {
    let file = File::open(path).expect("file not found!");
    let reader = BufReader::new(file);

    let mut coordinates = Vec::new();
    let mut commands = Vec::new();

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    for line in lines.iter() {
        if line.starts_with('#') {
            let row: Vec<Coordinate> = line
                .chars()
                .map(|c| match c {
                    '#' => Coordinate::Obstruction,
                    '@' => Coordinate::Robot,
                    'O' => Coordinate::Box,
                    '.' => Coordinate::Empty,
                    _ => panic!("Unkown char in grid"),
                })
                .collect();
            coordinates.push(row);
        } else {
            break;
        }
    }

    for line in &lines[coordinates.len()..] {
        commands.extend(line.chars().map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Unkown char in instructions"),
        }))
    }

    let mut robot = None;

    for (y_index, row) in coordinates.iter().enumerate() {
        for (x_index, coordinate) in row.iter().enumerate() {
            if *coordinate == Coordinate::Robot {
                if robot.is_some() {
                    panic!("Multiple guards in input.")
                }
                robot = Some(Point::new(x_index, y_index))
            }
        }
    }

    let grid = Grid::new(coordinates, robot.unwrap());
    (grid, commands)
}

fn solution_1(mut grid: Grid, directions: Directions) -> u64 {
    for direction in directions.iter() {
        grid.move_robot(*direction);
    }

    let mut sum: u64 = 0;
    for (y_index, row) in grid.coordinates.iter().enumerate() {
        for (x_index, coordinate) in row.iter().enumerate() {
            if *coordinate == Coordinate::Box {
                sum += (y_index as u64 * 100) + x_index as u64;
            }
        }
    }
    sum
}

fn solution_2(mut grid: Grid, directions: Directions) -> u64 {
    for direction in directions.iter() {
        let string_grid: String = grid.clone().into();
        println!("{}", string_grid);
        grid.move_robot(*direction);
    }

    let mut sum: u64 = 0;
    for (y_index, row) in grid.coordinates.iter().enumerate() {
        for (x_index, coordinate) in row.iter().enumerate() {
            if *coordinate == Coordinate::LeftBox {
                sum += (y_index as u64 * 100) + x_index as u64;
            }
        }
    }
    sum
}

fn main() {
    let input_start = Instant::now();
    let (grid, directions) = load_input("input_simple_simple.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(grid.clone(), directions.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let doubled_grid = Grid::double(grid.clone());

    let solution_2_start = Instant::now();
    let output_2 = solution_2(doubled_grid.clone(), directions.clone());

    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
