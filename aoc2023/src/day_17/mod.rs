use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::fs;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_17\\input.txt").expect("File not found!");

    println!("\n--Day 17------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn parse(input: &str) -> Puzzle { 
    let lines = input.lines();
    let mut goal = (lines.clone().count()-1, 0);
    let city_map = lines.enumerate().flat_map(|(row, line)| {
        goal.1 = line.len()-1;
        line.chars().enumerate().map(move |(col, ch)| {
            ((row, col), ch.to_string().trim().parse().expect("Should be valid number"))
        })
    }).collect::<HashMap<(usize, usize), u32>>();
    return Puzzle::new(city_map, goal);
}

fn part1(input: &str) -> u32 {
    let mut puzzle: Puzzle = parse(input);
    puzzle.dijkstra(1, 3);
    return puzzle.min_heat_loss;
}

fn part2(input: &str) -> u32 {
    let mut puzzle: Puzzle = parse(input);
    puzzle.dijkstra(4, 10);
    return puzzle.min_heat_loss;
}

struct Puzzle {
    city_map: HashMap<(usize, usize), u32>,
    // start: (usize, usize),
    goal: (usize, usize),
    min_heat_loss: u32,
}

impl Puzzle {
    fn new(city_map: HashMap<(usize, usize), u32>, goal: (usize, usize)) -> Self {
        Self {
            city_map: city_map,
            // start: (0, 0),
            goal: goal,
            min_heat_loss: u32::MAX,
        }
    }

    fn dijkstra(&mut self, min_walk: isize, max_walk: isize) {
        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        let mut visited = HashSet::new();

        queue.push(State { pos: (0, 0), dir: 0, cost: 0 });
        queue.push(State { pos: (0, 0), dir: 1, cost: 0 });
        
        while let Some(State { pos, dir, cost }) = queue.pop() {
            let orig_cost = cost;
            let mut cost = cost;
            
            if pos == self.goal { 
                // println!("REACHED GOALs {},{} with {cost} going {:?}", pos.0, pos.1, dir);
                self.min_heat_loss = cost;
                return;
            }
            
            if visited.contains(&(pos, dir)) {
                continue;
            }
            // println!("Exploring {},{} with {cost} going {:?}", pos.0, pos.1, dir);
            visited.insert((pos, dir));

            let (row, col) = (pos.0 as isize, pos.1 as isize);
            for n in [-1, 1] {
                cost = orig_cost;
                let (mut new_row, mut new_col) = (row, col);
                for step in 1..=max_walk {
                    if dir == 1 {
                        new_col = col + (step * n);
                    } else {
                        new_row = row + (step * n);
                    }

                    if new_row < 0 || new_row > self.goal.0 as isize || 
                        new_col < 0 || new_col > self.goal.1 as isize {
                        break;
                    }

                    let new_pos = (new_row as usize, new_col as usize);
                    let new_dir = 1 - dir;
                    cost += *self.city_map.get(&new_pos).unwrap();
                    
                    if visited.contains(&(new_pos, new_dir)) {
                        continue;
                    }

                    if step >= min_walk {
                        queue.push(State { pos: new_pos, dir: new_dir, cost: cost })
                    }
                }
            }
            self.min_heat_loss = cost;
        }
    }

}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: (usize, usize),
    dir: usize,
    cost: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            // .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(102, p1);
    }

    #[test]
    fn test_sample_p2() {
        let input = include_str!("sample.txt");
        let p2 = part2(input);
        dbg!(p2);
        assert_eq!(94, p2);
    }

    #[test]
    fn test_unfortunate_p2() {
        let input = include_str!("unfortunate.txt");
        let p2 = part2(input);
        dbg!(p2);
        assert_eq!(71, p2);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let (p1, p2) = (part1(input), part2(input));
        dbg!(p1, p2);
        assert_eq!(953, p1);
        assert_eq!(1180, p2);
    }
}