use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;

mod direction;
use direction::Direction;
use Direction::*;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
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

    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    for ((k1, k2), _v) in &city_map {
        g_score.insert((k1.clone(), k2.clone()), u32::MAX);
        f_score.insert((k1.clone(), k2.clone()), u32::MAX);
    }

    return Puzzle::new(city_map, goal, g_score, f_score);
}

fn process(input: &str) -> u32 {
    let mut puzzle: Puzzle = parse(input);
    puzzle.dijkstra();
    return puzzle.min_heat_loss;
}


struct Puzzle {
    city_map: HashMap<(usize, usize), u32>,
    start: (usize, usize),
    goal: (usize, usize),
    g_score: HashMap<(usize, usize), u32>,
    f_score: HashMap<(usize, usize), u32>,
    open_set: HashSet<(usize, usize)>,
    came_from: HashMap<(usize, usize), ((usize, usize), Vec<Direction>)>,
    min_heat_loss: u32,
}

impl Puzzle {
    fn new(city_map: HashMap<(usize, usize), u32>, goal: (usize, usize),
            g_score: HashMap<(usize, usize), u32>, 
            f_score: HashMap<(usize, usize), u32>) -> Self {
        let mut open_set = HashSet::new();
        open_set.insert((0, 0));
        Self {
            city_map: city_map,
            g_score: g_score, 
            f_score: f_score,
            start: (0, 0),
            goal: goal,
            open_set: open_set,
            came_from: HashMap::new(),
            min_heat_loss: u32::MAX,
        }
    }

    fn h(curr_pos: &(usize, usize), goal: (usize, usize)) -> u32 {
        return ((goal.0 - curr_pos.0) + (goal.1 - curr_pos.1) ) as u32
    }

    #[allow(dead_code)]
    fn a_star_3_step_lim(&mut self) {
        self.g_score.insert(self.start, *self.city_map.get(&self.start).expect("coords should exist"));
        self.f_score.insert(self.start, *self.city_map.get(&self.start).expect("coords should exist"));

        let mut dirs: Vec<Direction> = vec![];
        while !self.open_set.is_empty() {
            //TODO: Update current logic to grab lowest f_score from open set keys

            let (curr_pos, curr_val) = self.f_score.iter().filter(|((r, c), _v)| self.open_set.contains(&(*r, *c)))
                                                    .min_by_key(|((_r, _c), v)| *v).expect("Should find f");
            let (curr_pos, curr_val) = (*curr_pos, *curr_val);
            self.open_set.remove(&curr_pos);
            if let Some(node) = self.came_from.get(&curr_pos) {
                dirs = node.1.to_vec();
            };

            println!("Exploring {},{}", curr_pos.0, curr_pos.1);
            if curr_pos == self.goal {
                println!("*****CALCULATING*****");
                println!("Score: {curr_val}");
                let new_score = self.calculate_path_score(&curr_pos);
                if self.min_heat_loss > new_score {
                    self.min_heat_loss = new_score;
                }
            }

            let neighbors = Puzzle::get_neighbors(&curr_pos, &self.goal, &dirs);
            for (n_pos, n_dir) in neighbors {
                let mut dirs = dirs.clone();
                let tent_g = *self.g_score.get(&curr_pos).unwrap() + self.city_map.get(&n_pos).unwrap();
                print!("| N {},{} = {tent_g} ", n_pos.0, n_pos.1);
                if &tent_g < self.g_score.get(&n_pos).expect("Should find g at") {
                    if dirs.len() == 3 {
                        dirs.remove(0);
                    }
                    dirs.push(n_dir);
                    self.came_from.insert(n_pos, (curr_pos, dirs));
                    self.g_score.insert(n_pos, tent_g);
                    self.f_score.insert(n_pos, tent_g + Puzzle::h(&n_pos, self.goal));
                    self.open_set.insert(n_pos);
                    println!("\n*****FScore {},{} = {}", n_pos.0, n_pos.1, self.f_score.get(&n_pos).expect("Should find coords"));
                }
            }
            println!("|\n");
        }
        println!("*****Ran out of nodes*****");
    }

    fn get_neighbors(pos: &(usize, usize), max_pos: &(usize, usize), dirs: &Vec<Direction>) -> Vec<((usize, usize), Direction)> {
        let (row, col) = *pos;
        let mut neighbors: Vec<((usize, usize), Direction)> = vec![];

        let mut last_dir = &Undefined;
        if !dirs.is_empty() {
            last_dir = &dirs.last().unwrap();
        }

        if row > 0 && last_dir != &Down {
            neighbors.push(((row-1, col), Up))
        }

        if row < max_pos.0 && last_dir != &Up   {
            neighbors.push(((row+1, col), Down))
        }

        if col > 0 && last_dir != &Right  {
            neighbors.push(((row, col-1), Left))
        }

        if col < max_pos.1 && last_dir != &Left  {
            neighbors.push(((row, col+1), Right))
        }

        return neighbors;
    }

    fn calculate_path_score(&mut self, curr_pos: &(usize, usize)) -> u32 {
        let mut curr_pos = curr_pos;
        let mut score = *self.city_map.get(curr_pos).unwrap();
        let cnt = 0;

        let mut debug_pos: Vec<(usize, usize)> = vec![];
        while self.came_from.contains_key(curr_pos) {
            debug_pos.push((curr_pos.0, curr_pos.1));
            print!("Step {cnt}: curr: {},{} | ", curr_pos.0, curr_pos.1);
            let node = self.came_from.get(curr_pos).unwrap();
            curr_pos = &node.0;
            score += self.city_map.get(curr_pos).unwrap();
            println!("new curr {},{} with score {}", curr_pos.0, curr_pos.1, self.city_map.get(curr_pos).unwrap())
        }
        debug_pos.push((curr_pos.0, curr_pos.1));

        println!("Score is {score}");

        for i in 0..=self.goal.0 {
            for j in 0..=self.goal.1{
                if debug_pos.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        return score as u32;
    }

    fn dijkstra(&mut self) {
        let max_walk = 3;
        let min_walk = 1;

        println!("Max Row {}, Max Col {}", self.goal.0, self.goal.1);

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
                for step in min_walk..=max_walk {
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
                    queue.push(State { pos: new_pos, dir: new_dir, cost: cost })
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
    fn test_sample() {
        let input = include_str!("sample.txt");
        let result = process(input);
        dbg!(result);
        assert_eq!(102, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = process(input);
        dbg!(result);
        assert_eq!(953, result);
    }
}