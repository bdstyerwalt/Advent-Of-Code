use std::{collections::{HashSet, HashMap}, usize};
use Direction::{Up, Down, Left, Right};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(input: &str) -> Puzzle { 
    let lines_meta_data = input.clone().lines();
    let goal = Position::new(lines_meta_data.count()-1, lines_meta_data.next().unwrap().len()-1);
    let city_map = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().map(move |(col, ch)| {
            return (Position::new(row, col), ch.to_string().trim().parse().expect("Should be valid number"))
        })
    }).collect::<HashMap<Position, u32>>();

    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    for (pos, _v) in &city_map {
        g_score.insert(pos, u32::MAX);
        f_score.insert(pos, u32::MAX);
    }

    return Puzzle::new(city_map, goal, g_score, f_score);
}

fn process(input: &str) -> u32 {
    let mut puzzle: Puzzle = parse(input);
    puzzle.a_star_3_step_lim();
    return puzzle.min_heat_loss;
}


struct Puzzle {
    city_map: HashMap<Position, u32>,
    start: Position,
    goal: Position,
    g_score: HashMap<Position, u32>,
    f_score: HashMap<Position, u32>,
    open_set: HashSet<Position>,
    came_from: HashMap<Position, Position>,
    min_heat_loss: u32,
}

impl Puzzle {
    fn new(city_map: HashMap<Position, u32>, goal: Position,
            g_score: HashMap<&Position, u32>, 
            f_score: HashMap<&Position, u32>) -> Self {
        let start = Position::new(0, 0);
        let mut open_set = HashSet::new();
        open_set.insert(start);
        Self {
            city_map: city_map,
            g_score: g_score, 
            f_score: f_score,
            start: start,
            goal: goal,
            open_set: open_set,
            came_from: HashMap::new(),
            min_heat_loss: u32::MAX,
        }
    }

    fn a_star_3_step_lim(&mut self) {
        self.g_score.insert(self.start, *self.city_map.get(&self.start).expect("coords should exist"));
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
                println!("Score: {curr_val}");
                self.calculate_path_score(&curr_pos);
                return;
            }

            let neighbors = Puzzle::get_neighbors(&curr_pos, &self.goal);
            for n_pos in neighbors {
                let tent_g = *self.g_score.get(&curr_pos).unwrap();
                print!("| N {},{} = {tent_g} ", n_pos.0, n_pos.1);
                if &tent_g < self.g_score.get(&n_pos).unwrap() {
                    self.came_from.insert(n_pos, curr_pos);
                    self.g_score.insert(n_pos, tent_g);
                    self.f_score.insert(n_pos, tent_g + self.city_map.get(&n_pos).unwrap());
                    self.open_set.insert(n_pos);
                    println!("\n*****FScore {},{} = {}", n_pos.0, n_pos.1, self.f_score.get(&n_pos).unwrap());
                }
            }
            println!("|\n");
        }
        println!("*****Ran out of nodes*****");
    }

    fn calculate_path_score(&mut self, curr_pos: &Position) {
        let mut curr_pos = curr_pos;
        let mut score = *self.city_map.get(curr_pos).unwrap();
        let cnt = 0;
        while self.came_from.contains_key(curr_pos) {
            print!("Step {cnt}: curr: {},{} | ", curr_pos.0, curr_pos.1);
            curr_pos = self.came_from.get(curr_pos).unwrap();
            score += self.city_map.get(curr_pos).unwrap();
            println!("new curr {},{} with score {}", curr_pos.0, curr_pos.1, self.city_map.get(curr_pos).unwrap())
        }
        println!("Score is {score}");
        self.min_heat_loss = score;
    }
}

fn get_neighbors(pos: &Position, max_pos: &Position) -> Vec<Neighbor> {
    let mut neighbors: Vec<Neighbor> = vec![];
    let mut new_pos: Position = Position::new(pos.row, pos.col);
    if new_pos.row > 0 {
        new_pos.row -= 1;
        neighbors.push(Neighbor::new(new_pos, Up))
    }

    if new_pos.row < max_pos.row {
        new_pos.row += 1;
        neighbors.push(Neighbor::new(new_pos, Down))
    }

    if new_pos.col > 0 {
        new_pos.col -= 1;
        neighbors.push(Neighbor::new(new_pos, Left))
    }

    if new_pos.col < max_pos.col {
        new_pos.col += 1;
        neighbors.push(Neighbor::new(new_pos, Right))
    }

    return neighbors;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
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