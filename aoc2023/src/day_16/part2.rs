use std::{collections::{HashMap, HashSet}, hash::{Hasher, Hash}};
use Direction::{North, South, East, West};
use Mirror::{Pipe, Dash, Forward, Backward};

fn parse(input: &str) -> Puzzle {
    let mut width: usize = 0;
    let lines = input.lines();
    let height: usize = lines.clone().count();

    let mirror_map = lines.enumerate().flat_map(|(row, line)| {
        width = line.len();
        line.chars().enumerate().filter_map(move |(col, ch)| {
            if ch == '.' { None } else { Some(((row, col), Mirror::from_ch(ch))) }
        })
    }).collect::<HashMap<(usize, usize), Mirror>>();  
    return Puzzle::new(mirror_map, HashSet::new(), HashSet::new(), width, height);
}

// fn print_answser(puzzle: &mut Puzzle) {
//     let (start_row, start_col, start_dir) = puzzle.best_pos;
//     println!("BEST START POS: row {}, col {}, dir {:?}", start_row, start_col, start_dir);
//     puzzle.evaluate(start_row, start_col, start_dir);
//     for row in 0..puzzle.height {
//         for col in 0..puzzle.width {
//             if puzzle.visited_set.contains(&(row, col)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

pub fn process(input: &str) -> usize {
    let mut puzzle: Puzzle = parse(input);
    for row in 0..puzzle.height {
        for col in 0..puzzle.width {
            if row == 0 {
                puzzle.evaluate(row, col, South)
            }

            if col == 0 {
                puzzle.evaluate(row, col, East);
            }

            if row == puzzle.height {
                puzzle.evaluate(row, col, North)
            }

            if col == puzzle.width {
                puzzle.evaluate(row, col, West);
            }
        }
    }
    // print_answser(&mut puzzle);
    return puzzle.max_energized;
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
    max_energized: usize,
    best_pos: (usize, usize, Direction)
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
            max_energized: 0,
            best_pos: (0, 0, North)
        }
    }

    fn evaluate(&mut self, start_row: usize, start_col: usize, start_dir: Direction) {
        self.visited_pipes.clear();
        self.visited_set.clear();
        
        let mut new_dirs = vec![];
        match self.mirror_map.get(&(start_row, start_col)) {
            Some(mirror) => {
                let temp_dirs = eval_mirror(mirror.clone(), &start_dir);
                for dir in temp_dirs {
                    // println!("NEW STARTING DIR {:?}", dir);
                    new_dirs.push((start_row, start_col, dir));
                }
            }
            None => new_dirs = walk(self, start_row, start_col, &start_dir),
        }
        
        while !new_dirs.is_empty() && !self.mirror_map.is_empty(){
            let (row, col, dir) = new_dirs.remove(0);

            match dir {
                North => {
                    if row == 0 { continue; }
                },
                South => {
                    if row == self.height-1 { continue; }
                },
                East => {
                    if col == self.width-1 { continue; }
                },
                West => {
                    if col == 0 { continue; }
                },
            }
            // println!("Now walking {:?} from point ({row},{col})", &dir);
            if !self.visited_pipes.contains(&(row, col, dir)) {
                let next_dirs = walk(self, row, col, &dir);
                new_dirs.extend(next_dirs);
                self.visited_pipes.insert((row, col, dir));
            }
        }
        
        self.check_max(start_row, start_col, start_dir)
    }

    fn check_max(&mut self, row: usize, col: usize, init_dir: Direction) {
        let count = self.visited_set.iter().count();
        if count > self.max_energized {
            self.max_energized = count;
            self.best_pos = (row, col, init_dir);
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
        assert_eq!(51, process(input));
    }
}