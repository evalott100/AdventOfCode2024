#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn is_adjacent(&self, other: &Point) -> bool {
        (self.x == other.x && (self.y as isize - other.y as isize).abs() == 1)
            || (self.y == other.y && (self.x as isize - other.x as isize).abs() == 1)
    }

    pub fn adjacent_points(&self) -> Vec<Point> {
        Vec::from([
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ])
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
}
