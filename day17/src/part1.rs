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
        let MAX_WALK = 3;

        let mut dist = HashMap::new();
        let mut prev = HashMap::new();
        let mut dirs: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        for (key, _score) in &self.city_map {
            dist.insert(key.clone(), u32::MAX);
            prev.insert(key.clone(), None);
            dirs.insert(key.clone(), vec![]);
        }
        dist.insert(self.start.clone(), 0);
        queue.push(State { pos: (0, 0), cost: 0 });
        

        while let Some(State { pos, cost }) = queue.pop() {
            let curr_dirs = dirs.get(&pos).unwrap().to_vec();
            println!("Exploring {},{} with {cost}", pos.0, pos.1);

            // if pos == self.goal { 
            //     self.min_heat_loss = Puzzle::calculate_path_score_dijkstra(&self.city_map, &prev, &self.goal.clone());
            //     return;
            // }

            if &cost > dist.get(&pos).unwrap() {
                continue;
            }


            let neighbors = Puzzle::get_neighbors(&pos, &self.goal, &curr_dirs);
            // println!("{:?}", &neighbors);
            for (n_pos, ndir) in neighbors {
                let mut ndirs = curr_dirs.to_vec();
                let next = State { pos: n_pos, cost: cost + self.city_map.get(&n_pos).unwrap() };
                
                let mut skip_dir = &Undefined;
                if ndirs.len() == MAX_WALK && Direction::are_all_equal(&ndirs) {
                    skip_dir = ndirs.get(0).unwrap();
                }

                if skip_dir == &ndir {
                    continue;
                }
                queue.push(next);
                
                if &next.cost <= dist.get(&next.pos).unwrap() {
                    print!("| N {},{} = {} ", n_pos.0, n_pos.1, next.cost);
                    
                    // Handle the consecutive dirs
                    if ndirs.len() == MAX_WALK {
                        // println!("-------------{:?}", ndirs);
                        ndirs.remove(0);
                    }
                    ndirs.push(ndir);
                    print!("{:?}", ndirs);
                    
                    
                    dirs.insert(n_pos.clone(), ndirs);
                    dist.insert(n_pos.clone(), next.cost);
                    prev.insert(n_pos.clone(), Some(pos.clone()));
                }
            }
            println!("\n")
        }
        self.min_heat_loss = Puzzle::calculate_path_score_dijkstra(&self.city_map, &prev, &self.goal.clone());
    }

    fn calculate_path_score_dijkstra(city_map: &HashMap<(usize, usize), u32>, prev: &HashMap<(usize, usize), Option<(usize, usize)>>, curr_pos: &(usize, usize)) -> u32 {
        let mut score: Vec<u32> = vec![];
        let mut path: Vec<(usize, usize)> = vec![];
        let mut u = prev.get(&curr_pos).unwrap();
        path.insert(0, *curr_pos);
        score.insert(0, *city_map.get(&curr_pos).unwrap());
        dbg!(&path, &score);

        while u.is_some() && u.unwrap() != (0, 0) {
            let pos = u.unwrap();
            println!("{:?} {}", pos, *city_map.get(&pos).unwrap());
            path.insert(0, pos);
            score.insert(0, *city_map.get(&pos).unwrap());
            u = prev.get(&pos).unwrap();
        }


        for i in 0..=curr_pos.0 {
            for j in 0..=curr_pos.1{

                if path.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        return score.iter().sum();
    }

}



#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: (usize, usize),
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
            .then_with(|| self.pos.cmp(&other.pos))
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
        assert_eq!(102, process(input));
    }
}