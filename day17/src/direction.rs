#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Undefined,
}

impl Direction {
    pub fn are_all_equal(v_dirs: &Vec<Direction>) -> bool {
        let first_dir = v_dirs.get(0).unwrap();
        return v_dirs.iter().all(|dir| dir == first_dir);
    }
}
