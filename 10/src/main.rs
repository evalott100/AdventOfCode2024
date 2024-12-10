use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type Map = Vec<Vec<usize>>;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    fn boundary(&self, map: &Map) -> HashSet<Position> {
        let mut boundary = HashSet::new();
        if self.x > 0 {
            boundary.insert(Position::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            boundary.insert(Position::new(self.x, self.y - 1));
        }
        if self.x < map[self.y].len() - 1 {
            boundary.insert(Position::new(self.x + 1, self.y));
        }
        if self.y < map.len() - 1 {
            boundary.insert(Position::new(self.x, self.y + 1));
        }

        boundary
    }
}

#[derive(Clone, Debug)]
struct Node {
    value: usize,
    position: Position,
    sub_nodes: Vec<Node>,
}
impl Node {
    fn new(value: usize, position: Position) -> Node {
        Node {
            value,
            position,
            sub_nodes: Vec::new(),
        }
    }
    fn add_node(&mut self, node: Node) {
        self.sub_nodes.push(node);
    }

    fn is_leaf(&self) -> bool {
        self.sub_nodes.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    fn new(value: usize, position: Position) -> Tree {
        Tree {
            root: Node::new(value, position),
        }
    }
}

fn load_input(path: &str) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = Vec::new();

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        let mut line: Vec<usize> = Vec::new();

        let char_heights: Vec<char> = raw_line.unwrap().chars().collect();
        for char in char_heights {
            line.push((char as u32 - '0' as u32) as usize);
        }
        map.push(line);
    }

    map
}

fn get_paths(map: &Map, previous: &mut Node) {
    let value = previous.value;

    for boundary in previous.position.boundary(map).iter() {
        if map[boundary.y][boundary.x] == value + 1 {
            let mut node: Node = Node::new(value + 1, *boundary);
            get_paths(map, &mut node);
            previous.add_node(node)
        }
    }
}

fn parse_map_into_tree(map: &Map) -> Vec<Tree> {
    let mut trees: Vec<Tree> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 0 {
                continue;
            }

            let mut new_tree = Tree::new(0, Position::new(x, y));
            get_paths(map, &mut new_tree.root);
            trees.push(new_tree);
        }
    }
    trees
}

fn trail_ends(node: &Node, ends: &mut HashSet<Position>) {
    if node.is_leaf() && node.value == 9 {
        ends.insert(node.position);
    }
    for sub_node in node.sub_nodes.iter() {
        trail_ends(sub_node, ends);
    }
}

fn count_trail_ends(path_trees: &[Tree]) -> usize {
    let mut count = 0;
    for path_tree in path_trees.iter() {
        let mut ends = HashSet::new();
        trail_ends(&path_tree.root, &mut ends);
        count += ends.len();
    }
    count
}

fn number_of_trails(node: &Node) -> usize {
    if node.is_leaf() && node.value == 9 {
        return 1;
    }
    let mut count = 0;
    for sub_node in node.sub_nodes.iter() {
        count += number_of_trails(sub_node);
    }
    count
}

fn count_distinct_trails(path_trees: &[Tree]) -> usize {
    let mut count = 0;
    for path_tree in path_trees.iter() {
        count += number_of_trails(&path_tree.root);
    }
    count
}

fn solution_1(path_trees: &[Tree]) -> u64 {
    count_trail_ends(path_trees) as u64
}

fn solution_2(path_trees: &[Tree]) -> u64 {
    count_distinct_trails(path_trees) as u64
}

fn main() {
    let input_start = Instant::now();
    let map: Map = load_input("input.dat");
    let path_trees = parse_map_into_tree(&map);
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(&path_trees);
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(&path_trees);
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
