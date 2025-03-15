use std::collections::{HashMap, HashSet};

pub fn run() {
    let day_idx = file!().find("day_").expect("Couldn't find `day_` in file path") + 4;
    let day = file!().get(day_idx..day_idx+2).unwrap();
    let input_file = include_str!("input.txt");

    println!("\n--Day {day}------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

fn walk(p: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::U => return Point {x: p.x, y: p.y - 1},
        Direction::D => return Point {x: p.x, y: p.y + 1},
        Direction::L => return Point {x: p.x - 1, y: p.y},
        Direction::R => return Point {x: p.x + 1, y: p.y},
    }
}

fn turn(dir: Direction) -> Direction {
    match dir {
        Direction::U => return Direction::R,
        Direction::R => return Direction::D,
        Direction::D => return Direction::L,
        Direction::L => return Direction::U,
    }
}

#[derive(Debug)]
struct Puzzle {
    init_map: HashMap<Point, String>,
    final_map: HashMap<Point, String>,
    width: isize,
    height: isize,
    start_pt: Point,
    start_dir: Direction,
    visited: HashSet<Point>,
}

#[allow(dead_code)]
impl Puzzle {
    fn print_map(&self) {
        // print!("\n  ");
        // for c in 0..self.width {
            // print!("{c}");
        // }
        println!();

        for r in 0..self.height {
            // print!("{} ", r);
            for c in 0..self.width {
                print!("{}", self.final_map.get(&Point{x:c, y:r}).unwrap());
            }
            println!();
        }
    }
    
    fn print_init_map(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                print!("{}", self.final_map.get(&Point{x:c, y:r}).unwrap());
            }
            println!();
        }
    }
    
    fn count_x_per_row(&self) {
        let mut tot_cnt = 0;
        for r in 0..self.height {
            let mut cnt = 0;
            for c in 0..self.width {
                if self.final_map.get(&Point{x:c, y:r}).unwrap() == &String::from("X") {
                    cnt += 1;
                }
            }
            println!("Row {r}: {cnt} X's");
            tot_cnt += cnt;
        }

        println!("@@@ {tot_cnt}");
    }
}

fn parse(input: &str) -> Puzzle {
    let mut init_map: HashMap<Point, String> = HashMap::new();
    let mut start: Point = Point {x:0, y:0};
    let mut start_dir: Direction = Direction::U;
    let mut w = 0;
    let mut h = 0;
    
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let p = Point {x: col as isize, y: row as isize};
            init_map.insert(p.clone(), ch.to_string());
            
            if ch == '^' {
                start = p;
                start_dir = Direction::U;
            }
            w = col as isize;
        }
        h = row as isize;
    }
    
    
    let puzzle = Puzzle {
        init_map: init_map.clone(),
        final_map: init_map,
        width: w+1,
        height: h+1,
        start_pt: start,
        start_dir: start_dir,
        visited: HashSet::new(),
    };
    //println!("{:?}", puzzle);
    return puzzle;
}

fn part1(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    let mut pt = puzzle.start_pt;
    let mut dir = puzzle.start_dir;
    
    loop {
        puzzle.visited.insert(pt.clone());
        puzzle.final_map.insert(pt.clone(), "X".to_string());

        let new_pt = walk(&pt, &dir);
        if new_pt.x < 0 || new_pt.x >= puzzle.width || new_pt.y < 0 || new_pt.y >= puzzle.height {
            break;
        }
        
        if puzzle.init_map.get(&new_pt) == Some(&"#".to_string()) {
            dir = turn(dir);
            continue
        }
        pt = new_pt;
    }
    
    return puzzle.visited.len();
}

fn part2(input_file: &str) -> usize {
    let _puzzle = parse(input_file);

    return 0;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(5, part1(include_str!("small.txt")));
        assert_eq!(41, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(0000, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(5331, part1(input));
        // assert_eq!(0000, part2(input));
    }
}