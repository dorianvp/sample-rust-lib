use arithmetic::{add, subtract};

pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> u64 {
        add(
            subtract(self.x.max(other.x), self.x.min(other.x)),
            subtract(self.y.max(other.y), self.y.min(other.y)),
        )
    }
}
