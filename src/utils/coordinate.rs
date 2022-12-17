#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }
    pub fn is_adjacent_to(&self, other: &Coordinate, diagonally: bool) -> bool {
        if self.x == other.x && self.y.abs_diff(other.y) == 1 {
            return true;
        }
        if self.y == other.y && self.x.abs_diff(other.x) == 1 {
            return true;
        }
        if diagonally && self.x.abs_diff(other.x) == 1 && self.y.abs_diff(other.y) == 1 {
            return true;
        }
        false
    }
}
