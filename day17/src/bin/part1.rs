use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use Direction::*;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(input: &str) -> Puzzle { 
    let city_map = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().map(move |(col, ch)| {
            Node::new(Position::new(row, col), usize::MAX, usize::MAX,
                          ch.to_string().trim().parse().expect("Should be valid number"))
        })
    }).collect::<HashSet<Node>>();

    let last_pos = city_map.iter().last().unwrap().pos;
    return Puzzle::new(city_map, Position::new(0, 0), last_pos);
}

fn process(input: &str) -> u32 {
    let mut puzzle: Puzzle = parse(input);
    puzzle.a_star_3_step_lim();
    return puzzle.min_heat_loss;
}


struct Puzzle<'a> {
    city_map: HashSet<Node>,
    start: Position,
    goal: Position,
    open_nodes: BinaryHeap<&'a mut Node>,
    came_from: HashMap<Position, Position>,
    min_heat_loss: u32,
}

impl<'a> Puzzle<'a> {
    fn new(city_map: HashSet<Node>, start: Position, goal: Position) -> Self {
        let mut open_nodes = BinaryHeap::new();
        Self {
            city_map: city_map,
            start: start,
            goal: goal,
            open_nodes: open_nodes,
            came_from: HashMap::new(),
            min_heat_loss: u32::MAX,
        }
    }

    fn a_star_3_step_lim(&mut self) {
        let mut init_pos = &self.city_map.iter().filter(|node| node.pos == self.start);
        let init_pos = init_pos.next().expect("Should contain start node");
        init_pos.g = 0;
        init_pos.f = init_pos.h;
        self.open_nodes.push(&mut init_pos);

        while !self.open_nodes.is_empty() {
            let curr = self.open_nodes.pop().unwrap();
            println!("Exploring {},{} -- {:?}", curr.pos.row, curr.pos.col, curr.prev_dirs);
            if curr.pos == self.goal {
                println!("*****CALCULATING*****");
                self.calculate_path_score(&curr);
                return;
            }

            let neighbors = get_neighbors(&curr.pos, &self.goal, &curr.prev_dirs);
            for nb in neighbors {
                let mut nb_node: &mut Node = self.city_map.iter().filter(|node| node.pos == nb.pos).next().unwrap();
                let mut n_dirs = curr.prev_dirs.clone();
                let tent_g = curr.g;
                print!("| N {},{} = {tent_g} ", nb.pos.row, nb.pos.col);
                if tent_g < nb_node.g {
                    nb_node.g = tent_g;
                    nb_node.f = tent_g + nb_node.h;

                    self.came_from.insert(nb.pos, curr.pos);
                    if n_dirs.len() >= 3 { 
                        n_dirs.remove(0);
                    }
                    n_dirs.push(nb.dir);
                    nb_node.prev_dirs = n_dirs;

                    self.open_nodes.push(&mut nb_node);
                    // println!("\n*****FScore {},{} = {}", nb.pos.row, nb.pos.col, self.f_score.get(&nb.pos).unwrap());
                    // println!("PrevDirs: {:?}", n_dirs);
                    // println!("OpenPos: {:?}", self.open_nodes);
                }
            }
            println!("|\n");
        }
        println!("*****Ran out of nodes*****");
    }

    fn calculate_path_score(&mut self, curr: &Node) {
        let mut curr_pos = curr.pos;
        let mut score = curr.h;
        let cnt = 0;

        let mut debug_pos: Vec<&Position> = vec![];
        debug_pos.push(&curr_pos);

        while self.came_from.contains_key(&curr_pos) {
            print!("Step {cnt}: {},{} <- ", curr_pos.row, curr_pos.col);
            curr_pos = *self.came_from.get(&curr_pos).unwrap();
            let curr_node = self.city_map.iter().filter(|node| node.pos == curr_pos).next().unwrap(); 
            score += curr_node.h;
            debug_pos.push(&curr_pos);
            println!("{},{} with score {}", curr_pos.row, curr_pos.col, curr_node.h)
        }
        println!("Score is {score}");
        self.min_heat_loss = score as u32;

        for i in 0..=self.goal.row {
            for j in 0..=self.goal.col{
                if debug_pos.contains(&&Position::new(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn get_neighbors(pos: &Position, max_pos: &Position, prev_dirs: &Vec<Direction>) -> Vec<Neighbor> {
    let mut neighbors: Vec<Neighbor> = vec![];

    let mut skip_dir = &Undefined;
    if prev_dirs.len() == 3 {
        let first_dir = prev_dirs.get(0).unwrap();
        if Direction::are_all_equal(prev_dirs) {
            skip_dir = first_dir;
            println!("SKIPDIRSSSSS: {:?}", skip_dir);
        }
    }

    if pos.row > 0 && skip_dir != &Up {
        let new_pos = Position::new(pos.row - 1, pos.col);
        neighbors.push(Neighbor::new(new_pos, Up))
    }

    if pos.row < max_pos.row && skip_dir != &Down {
        let new_pos = Position::new(pos.row + 1, pos.col);
        neighbors.push(Neighbor::new(new_pos, Down))
    }

    if pos.col > 0 && skip_dir != &Left {
        let new_pos = Position::new(pos.row, pos.col - 1);

        neighbors.push(Neighbor::new(new_pos, Left))
    }

    if pos.col < max_pos.col && skip_dir != &Right {
        let new_pos = Position::new(pos.row, pos.col + 1);
        neighbors.push(Neighbor::new(new_pos, Right))
    }
    // println!("NNN: {:?}", neighbors);
    return neighbors;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    pos: Position,
    f: usize,
    g: usize,
    h: usize,
    prev_dirs: Vec<Direction>

}

impl Node {
    fn new(pos: Position, f: usize, g: usize, h: usize) -> Self {
        Self {
            pos,
            f,
            g,
            h,
            prev_dirs: vec![],
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }
}

#[derive(Debug)]
struct Neighbor {
    pos: Position,
    dir: Direction,
}

impl Neighbor {
    fn new(pos: Position, dir: Direction) -> Self {
        Self {
            pos,
            dir,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Undefined,
}

impl Direction {
    fn are_all_equal(v_dirs: &Vec<Direction>) -> bool {
        let first_dir = v_dirs.get(0).unwrap();
        return v_dirs.iter().all(|dir| dir == first_dir);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(102, process(input));
    }
}