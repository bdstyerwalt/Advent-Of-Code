use crate::Position;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};


#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub pos: Position,
    pub f: usize,
    pub g: usize,
    pub h: usize,
}

impl Node {
    pub fn new(pos: Position, f: usize, g: usize, h: usize) -> Self {
        Self {
            pos,
            f,
            g,
            h,
        }
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
        // Add other fields for equality check if needed
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.row.hash(state);
        self.pos.col.hash(state);
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.f.cmp(&self.f)
            .then_with(|| self.pos.row.cmp(&other.pos.row))
            .then_with(|| self.pos.col.cmp(&other.pos.col))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}