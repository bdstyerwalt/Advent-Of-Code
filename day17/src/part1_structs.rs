use std::collections::{HashMap, BinaryHeap};

mod node;
use node::Node;

mod position;
use position::Position;

mod neighbor;
use neighbor::Neighbor;

mod direction;
use direction::Direction;
use Direction::*;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(input: &str) -> Puzzle { 
    let mut meta_data = input.clone().lines();
    let last_row = meta_data.clone().count();
    let last_col = meta_data.nth(0).unwrap().chars().count();
    let last: Position = Position::new(last_row, last_col);

    let city_map = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().map(move |(col, ch)| {
            println!("{:?}", Position::new(row, col));
            (Node::new(Position::new(row, col), usize::MAX, usize::MAX,
                          ch.to_string().trim().parse().expect("Should be valid number")),
                          vec![])
        })
    }).collect::<HashMap<Node, Vec<Direction>>>();
    return Puzzle::new(city_map, Position::new(0, 0), last);
}

fn process(input: &str) -> u32 {
    let puzzle: Puzzle = parse(input);
    let min_heat_loss = a_star_3_step_lim(puzzle);
    return min_heat_loss;
}

fn a_star_3_step_lim(mut puzzle: Puzzle) -> u32 {
    let mut open_nodes: BinaryHeap<Position> = BinaryHeap::new();

    let (mut init_node, init_dirs) = puzzle.retrieve_node(puzzle.start);
    puzzle.city_map.remove(&init_node);
    init_node.g = init_node.h;
    init_node.f = init_node.h;

    open_nodes.push(init_node.pos);
    puzzle.city_map.insert(init_node, init_dirs);
    assert_eq!(puzzle.goal.row*puzzle.goal.col, puzzle.city_map.iter().count());


    let mut came_from: HashMap<Position, Position> = HashMap::new();
    while !open_nodes.is_empty() {
        let curr_pos = open_nodes.pop().unwrap();
        let (curr_node, curr_dirs) = puzzle.retrieve_node(curr_pos);
        println!("Exploring {},{} with {} -- {:?}", curr_node.pos.row, curr_node.pos.col, curr_node.g, curr_dirs);
        if curr_node.pos == puzzle.goal {
            println!("*****CALCULATING*****");
            puzzle.min_heat_loss = puzzle.calculate_path_score(&came_from, &curr_node);
            if puzzle.min_heat_loss > curr_node.g as u32 {
                puzzle.min_heat_loss = curr_node.g as u32;
            }
        }

        let neighbors = get_neighbors(&curr_node.pos, &puzzle.goal, &curr_dirs);
        for nb in neighbors {
            let (mut nb_node, _) = puzzle.retrieve_node(nb.pos);
            let mut nb_dirs = curr_dirs.to_vec();
            puzzle.city_map.remove(&nb_node);

            let tent_g = curr_node.g;

            print!("| N {},{} = {tent_g} ", nb.pos.row, nb.pos.col);
            if tent_g < nb_node.g {
                nb_node.g = tent_g;
                nb_node.f = tent_g + nb_node.h;

                came_from.insert(nb.pos, curr_node.pos);
                if nb_dirs.len() >= 3 {
                    nb_dirs.remove(0);
                }
                nb_dirs.push(nb.dir);
                print!("{:?}", nb.dir);
                
                if open_nodes.
                open_nodes.push(nb.pos);
                println!("\n*****FSCORE: {}", nb_node.f);
            }
            puzzle.city_map.insert(nb_node, nb_dirs);
            assert_eq!(puzzle.goal.row*puzzle.goal.col, puzzle.city_map.iter().count());
        }
        println!("|\n");
    }
    println!("*****Ran out of nodes*****");
    return puzzle.min_heat_loss;
}

#[derive(Debug)]
struct Puzzle {
    city_map: HashMap<Node, Vec<Direction>>,
    start: Position,
    goal: Position,
    min_heat_loss: u32,
}

impl Puzzle {
    fn new(city_map: HashMap<Node, Vec<Direction>>, start: Position, goal: Position) -> Self {
        Self {
            city_map: city_map,
            start: start,
            goal: goal,
            min_heat_loss: u32::MAX,
        }
    }

    fn retrieve_node(&self, pos: Position) -> (Node, Vec<Direction>) {
        let (node, dirs) = self.city_map.iter().filter(|(node, _)| node.pos == pos).next().unwrap();
        let node = *node;
        return (node, dirs.to_vec());
    }

    fn calculate_path_score(&self, came_from: &HashMap<Position, Position>, curr: &Node) -> u32 {
        let mut curr_pos = curr.pos;
        let mut score = curr.h;
        let cnt = 0;

        let mut debug_pos: Vec<Position> = vec![];
        while came_from.contains_key(&curr_pos) {
            debug_pos.push(curr_pos);
            print!("Step {cnt}: {},{} <- ", curr_pos.row, curr_pos.col);
            curr_pos = *came_from.get(&curr_pos).unwrap();
            let (curr_node, _) = self.city_map.iter().filter(|(node, _)| node.pos == curr_pos).next().unwrap(); 
            score += curr_node.h;
            println!("{},{} with score {}", curr_pos.row, curr_pos.col, curr_node.h)
        }
        println!("Score is {score}");

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

        return score as u32;
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

    if pos.row < max_pos.row-1 && skip_dir != &Down {
        let new_pos = Position::new(pos.row + 1, pos.col);
        neighbors.push(Neighbor::new(new_pos, Down))
    }

    if pos.col > 0 && skip_dir != &Left {
        let new_pos = Position::new(pos.row, pos.col - 1);

        neighbors.push(Neighbor::new(new_pos, Left))
    }

    if pos.col < max_pos.col-1 && skip_dir != &Right {
        let new_pos = Position::new(pos.row, pos.col + 1);
        neighbors.push(Neighbor::new(new_pos, Right))
    }
    // println!("NNN: {:?}", neighbors);
    return neighbors;
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