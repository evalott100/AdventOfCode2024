use aoc_utils::Point;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::time::Instant;

fn load_input(path: &str) -> Vec<Vec<char>> {
    let mut garden_map: Vec<Vec<char>> = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        garden_map.push(raw_line.unwrap().chars().collect());
    }
    garden_map
}

fn point_in_bounds(point: Point, x_bound: usize, y_bound: usize) -> bool {
    point.x <= x_bound && point.y <= y_bound
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .cloned()
    }
}

fn region_from_point(
    garden_map: &Vec<Vec<char>>,
    point: Point,
    region_so_far: &mut BTreeSet<Point>,
    perimeter_so_far: &mut BTreeMap<Point, Vec<Direction>>,
) {
    let plant = garden_map[point.y][point.x];

    for (adjacent_point, direction) in zip(point.adjacent_points().iter(), Direction::iter()) {
        if !point_in_bounds(
            *adjacent_point,
            garden_map[0].len() - 1,
            garden_map.len() - 1,
        ) || garden_map[adjacent_point.y][adjacent_point.x] != plant
        {
            perimeter_so_far
                .entry(point)
                .or_insert_with(Vec::new)
                .push(direction)
        } else if !region_so_far.contains(adjacent_point) {
            region_so_far.insert(*adjacent_point);
            region_from_point(garden_map, *adjacent_point, region_so_far, perimeter_so_far);
        }
    }
}

#[derive(Clone)]
struct Region {
    contents: BTreeSet<Point>,
    perimeter: BTreeMap<Point, Vec<Direction>>,
    character: char,
}

impl Region {
    fn new(garden_map: &Vec<Vec<char>>, point: Point) -> Region {
        // Has to be the a top left index.
        let mut contents: BTreeSet<Point> = BTreeSet::from([point]);
        let mut perimeter: BTreeMap<Point, Vec<Direction>> = BTreeMap::new();
        region_from_point(garden_map, point, &mut contents, &mut perimeter);
        Region {
            contents,
            perimeter,
            character: garden_map[point.y][point.x],
        }
    }

    fn contains(&self, point: Point) -> bool {
        self.contents.contains(&point)
    }

    fn area(&self) -> u32 {
        self.contents.len() as u32
    }

    fn fencing(&self) -> u32 {
        let mut sum = 0;
        for (_, fencing) in self.perimeter.iter() {
            sum += fencing.len() as u32;
        }
        sum
    }

    fn sides(&self) -> u32 {
        let mut sum = 0;
        // TODO
        for (point, directions) in self.perimeter.iter() {
            println!("{:?} directions {:?}", point, directions);
            for direction in directions {
                sum += 1;
            }
        }
        sum
    }
}
fn find_regions(garden_map: &Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    for y in 0..garden_map.len() {
        for x in 0..garden_map[y].len() {
            let point = Point::new(x, y);
            if !regions.iter().any(|region| region.contains(point)) {
                regions.push(Region::new(garden_map, point));
            }
        }
    }
    regions
}

fn solution_1(regions: Vec<Region>) -> u32 {
    let mut sum = 0;
    for region in regions {
        let area = region.area();
        let fencing = region.fencing();
        println!(
            "region {:?}, area {:?} * perimeter {:?} = {:?}",
            region.character,
            area,
            fencing,
            area * fencing
        );
        sum += area * fencing;
    }
    sum
}

fn solution_2(regions: Vec<Region>) -> u32 {
    let mut sum = 0;
    for region in regions {
        let area = region.area();
        let sides = region.sides();
        println!(
            "region {:?}, area {:?} * sides {:?} = {:?}",
            region.character,
            area,
            sides,
            area * sides
        );
        sum += area * sides;
    }
    sum
}

fn main() {
    let input_start = Instant::now();
    let garden_map: Vec<Vec<char>> = load_input("input_simple.dat");
    let regions = find_regions(&garden_map);
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(regions.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );
    let solution_2_start = Instant::now();
    let output_2 = solution_2(regions.clone());
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
