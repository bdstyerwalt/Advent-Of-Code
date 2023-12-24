use std::{collections::{HashSet, HashMap}, usize};

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
    puzzle.a_star_3_step_lim();
    return puzzle.min_heat_loss;
}


struct Puzzle {
    city_map: HashMap<(usize, usize), u32>,
    start: (usize, usize),
    goal: (usize, usize),
    g_score: HashMap<(usize, usize), u32>,
    f_score: HashMap<(usize, usize), u32>,
    open_set: HashSet<(usize, usize)>,
    came_from: HashMap<(usize, usize), (usize, usize)>,
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

    fn a_star_3_step_lim(&mut self) {
        self.g_score.insert(self.start, 0);
        self.f_score.insert(self.start, *self.city_map.get(&self.start).expect("coords should exist"));

        while !self.open_set.is_empty() {
            //TODO: Update current logic to grab lowest f_score from open set keys

            let (curr_pos, curr_val) = self.f_score.iter().filter(|((r, c), _v)| self.open_set.contains(&(*r, *c)))
                                                    .min_by_key(|((_r, _c), v)| *v).expect("Should find f");
            let (curr_pos, curr_val) = (*curr_pos, *curr_val);
            self.open_set.remove(&curr_pos);

            println!("Exploring {},{}", curr_pos.0, curr_pos.1);
            if curr_pos == self.goal {
                println!("*****CALCULATING*****");
                self.calculate_path_score(curr_val);
                return;
            }

            let neighbors = Puzzle::get_neighbors(&curr_pos, &self.goal);
            for n_pos in neighbors {
                let tent_g = *self.g_score.get(&curr_pos).expect("Shold find g");
                // println!("Neighbor {},{}", n_pos.0, n_pos.1);
                if &tent_g < self.g_score.get(&n_pos).expect("Should find g at") {
                    self.came_from.insert(n_pos, curr_pos);
                    self.g_score.insert(n_pos, tent_g);
                    self.f_score.insert(n_pos, tent_g + self.city_map.get(&n_pos).expect("Should find coords"));
                    self.open_set.insert(n_pos);
                }
            }
        }
        println!("*****Ran out of nodes*****");
        dbg!(&self.goal);
    }

    fn get_neighbors(pos: &(usize, usize), max_pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let (start, stop) = *pos;
        let mut neighbors: Vec<(usize, usize)> = vec![];

        if start > 0 {
            neighbors.push((start-1, stop))
        }

        if start < max_pos.0 {
            neighbors.push((start+1, stop))
        }

        if stop > 0 {
            neighbors.push((start, stop-1))
        }

        if stop < max_pos.1 {
            neighbors.push((start, stop+1))
        }

        return neighbors;
    }

    fn calculate_path_score(&mut self, current: u32) {
        let score = self.came_from.iter().fold(current, |acc, (keys, _values)| {
            let (k1, k2) = (keys.0, keys.1);
            match self.city_map.get(&(k1, k2)) {
                Some(val) => {
                    println!("value at ({k1},{k2}) {val}");
                    acc + val
                },
                None => acc,
            }
        });
        println!("Score is {score}");
        self.min_heat_loss = score;
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