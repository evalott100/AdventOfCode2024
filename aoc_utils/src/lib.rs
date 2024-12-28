use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Eq, PartialEq, Debug, Copy, Ord, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn is_adjacent(&self, other: &Point) -> bool {
        (self.x == other.x && (self.y as isize - other.y as isize).abs() == 1)
            || (self.y == other.y && (self.x as isize - other.x as isize).abs() == 1)
    }

    pub fn up(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }
    pub fn down(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }
    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn adjacent(&self, direction: Direction) -> Point {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Hash)]
pub struct Grid<T> {
    pub rows: VecDeque<VecDeque<T>>,
}

impl<T> Grid<T> {
    pub fn new(rows: VecDeque<VecDeque<T>>) -> Grid<T> {
        Grid { rows }
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        self.rows.get(point.y).and_then(|row| row.get(point.x))
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut T> {
        self.rows
            .get_mut(point.y)
            .and_then(|row| row.get_mut(point.x))
    }

    pub fn width(&self) -> usize {
        self.rows.front().map_or(0, |row| row.len())
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.rows[point.y][point.x]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.rows[point.y][point.x]
    }
}
