use crate::Position;
use crate::Direction;

#[derive(Debug)]
pub struct Neighbor {
    pub pos: Position,
    pub dir: Direction,
}

impl Neighbor {
    pub fn new(pos: Position, dir: Direction) -> Self {
        Self {
            pos,
            dir,
        }
    }
}
