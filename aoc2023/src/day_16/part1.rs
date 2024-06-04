use std::{collections::{HashMap, HashSet}, hash::{Hasher, Hash}};
use Direction::{North, South, East, West};
use Mirror::{Pipe, Dash, Forward, Backward};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> usize {
    let mut width: usize = 0;
    let lines = input.lines();
    let height: usize = lines.clone().count();

    let mirror_map = lines.enumerate().flat_map(|(row, line)| {
        width = line.len();
        line.chars().enumerate().filter_map(move |(col, ch)| {
            if ch == '.' { None } else { Some(((row, col), Mirror::from_ch(ch))) }
        })
    }).collect::<HashMap<(usize, usize), Mirror>>();  
    
    let mut puzzle = Puzzle::new(mirror_map, HashSet::new(), HashSet::new(), width, height);
    // dbg!(&puzzle);

    let mut new_dirs = vec![];
    match puzzle.mirror_map.get(&(0, 0)) {
        Some(mirror) => {
            let temp_dirs = eval_mirror(mirror.clone(), &East);
            for dir in temp_dirs {
                // println!("NEW STARTING DIR {:?}", dir);
                new_dirs.push((0, 0, dir));
            }
        }
        None => new_dirs = walk(&mut puzzle, 0, 0, &East),
    }
    
    while !new_dirs.is_empty() && !puzzle.mirror_map.is_empty(){
        let (row, col, dir) = new_dirs.remove(0);

        match dir {
            North => {
                if row == 0 { continue; }
            },
            South => {
                if row == puzzle.height-1 { continue; }
            },
            East => {
                if col == puzzle.width-1 { continue; }
            },
            West => {
                if col == 0 { continue; }
            },
        }
        // println!("Now walking {:?} from point ({row},{col})", &dir);
        if !puzzle.visited_pipes.contains(&(row, col, dir)) {
            let next_dirs = walk(&mut puzzle, row, col, &dir);
            new_dirs.extend(next_dirs);
            puzzle.visited_pipes.insert((row, col, dir));
        }
    }
    // dbg!(&puzzle.visited_set);

    for row in 0..puzzle.height {
        for col in 0..puzzle.width {
            if puzzle.visited_set.contains(&(row, col)) {
                // print!("#");
            } else {
                // print!(".");
            }
        }
        // println!();
    }
    // dbg!(puzzle.width);
    // dbg!(puzzle.height);
    return puzzle.visited_set.iter().count();
}

fn walk(puzzle: &mut Puzzle, curr_row: usize, curr_col: usize, curr_dir: &Direction) -> Vec<(usize, usize, Direction)> {
    let mut new_dirs: Vec<(usize, usize, Direction)> = vec![];
    let mut points: Vec<(usize, usize)> = vec![];
    let (row, col): (usize, usize);
    let mirror: &Mirror;
    // println!("------ Current ------");
    // println!("--- Row {curr_row} | Col {curr_col} ---");
    // println!("------- {:?} -------", curr_dir);
    // println!("---------------------");
    match curr_dir {
        North => {
            match puzzle.mirror_map.iter()
                    .filter(|((row, col), _mir)| *row < curr_row && *col == curr_col)
                    .max_by_key(|((row, _col), _mir)| row) {
                Some(map) => {
                    (row, col, mirror) = (map.0.0, map.0.1, map.1);
                    create_energized_points(&mut points, col, row, curr_row, false)
                },
                None => {
                    create_energized_points(&mut points, curr_col, 0, curr_row, false);
                    puzzle.visited_set.extend(points.iter());
                    return new_dirs
                },
            }
        },
        South => {
            match puzzle.mirror_map.iter()
                    .filter(|((row, col), _mir)| *row > curr_row && *col == curr_col)
                    .min_by_key(|((row, _col), _mir)| row) {
                Some(map) => {
                    (row, col, mirror) = (map.0.0, map.0.1, map.1);
                    create_energized_points(&mut points, col, curr_row, row, false)
                },
                None => {
                    create_energized_points(&mut points, curr_col, curr_row, puzzle.height-1, false);
                    puzzle.visited_set.extend(points.iter());
                    return new_dirs
                },
            }
        },
        East => {
            match puzzle.mirror_map.iter()
                    .filter(|((row, col), _mir)| *col > curr_col && *row == curr_row)
                    .min_by_key(|((_row, col), _mir)| col) {
                Some(map) => {
                    (row, col, mirror) = (map.0.0, map.0.1, map.1);    
                    create_energized_points(&mut points, row, curr_col, col, true)
                },
                None => {
                    create_energized_points(&mut points, curr_row, curr_col, puzzle.width-1, true);
                    puzzle.visited_set.extend(points.iter());
                    return new_dirs
                },
            }
        },
        West => {
            match puzzle.mirror_map.iter()
                    .filter(|((row, col), _mir)| *col < curr_col && *row == curr_row)
                    .max_by_key(|((_row, col), _mir)| col) {
                Some(map) => {
                    (row, col, mirror) = (map.0.0, map.0.1, map.1);
                    create_energized_points(&mut points, row, col, curr_col, true)
                },
                None => {
                    create_energized_points(&mut points, curr_row, 0, curr_col, true);
                    puzzle.visited_set.extend(points.iter());
                    return new_dirs
                },
            }
        },
    }
    puzzle.visited_set.extend(points.iter());
    let temp_dirs = eval_mirror(mirror.clone(), &curr_dir);
    for dir in temp_dirs {
        new_dirs.push((row, col, dir));
    }
    return new_dirs;
}

fn create_energized_points(points: &mut Vec<(usize, usize)>, unchanged: usize, a: usize, b: usize, is_col: bool) {
    for i in a..b+1 {
        if is_col {
            // println!("Adding Points from row {unchanged} on cols {a} to {}", b);
            // print!("({unchanged},{i}) |");
            points.push((unchanged, i))       
        } else {
            // println!("Adding Points from col {unchanged} on rows {a} to {}", b);
            // print!("({i},{unchanged}) |");
            points.push((i, unchanged))
        }
    }
    // println!();
} 

fn eval_mirror(mirror: Mirror, curr_dir: &Direction) -> Vec<Direction> {
    let mut dirs: Vec<Direction> = vec![];

    match (curr_dir, mirror) {
        // North
        (North, Pipe) => dirs.push(North),
        (North, Dash) => {
            dirs.push(East);
            dirs.push(West);
        },
        (North, Forward) => dirs.push(East),
        (North, Backward) => dirs.push(West),

        // South
        (South, Pipe) => dirs.push(South),
        (South, Dash) => {
            dirs.push(East);
            dirs.push(West);
        },
        (South, Forward) => dirs.push(West),
        (South, Backward) => dirs.push(East),

        // East
        (East, Pipe) => {
            dirs.push(North);
            dirs.push(South);
        },
        (East, Dash) => dirs.push(East), 
        (East, Forward) => dirs.push(North),
        (East, Backward) => dirs.push(South),

        // West
        (West, Pipe) => {
            dirs.push(North);
            dirs.push(South);
        },
        (West, Dash) => dirs.push(West),
        (West, Forward) => dirs.push(South),
        (West, Backward) => dirs.push(North),
    }
    return dirs;
}

#[derive(Debug)]
struct Puzzle {
    mirror_map: HashMap<(usize, usize), Mirror>,
    visited_pipes: HashSet<(usize, usize, Direction)>,
    visited_set: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn new(mirrors: HashMap<(usize, usize), Mirror>, visited_locs: HashSet<(usize, usize)>,
            visited_pipes: HashSet<(usize, usize, Direction)>, width: usize, height: usize,) -> Self {
        Self {
            mirror_map: mirrors,
            visited_set: visited_locs,
            visited_pipes: visited_pipes,
            width: width,
            height: height,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            North => 1.hash(state),
            South => 2.hash(state),
            East => 3.hash(state),
            West => 4.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
enum Mirror {
    Pipe,
    Dash,
    Forward,
    Backward,
}

impl Mirror {
    fn from_ch(ch: char) -> Mirror {
        return match ch {
            '|' => Pipe,
            '-' => Dash,
            '/' => Forward,
            '\\' => Backward,
            _ => panic!("Not a valid mirror character")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(46, process(input));
    }
}