use std::collections::{HashMap, HashSet};
use std::usize;

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
    start_pt: Point,
    start_dir: Direction,
    width: isize,
    height: isize,
    init_map: HashMap<Point, String>,
    final_map: HashMap<Point, String>,
    visited: HashSet<Point>,
    path: Vec<(Point, Direction)>,
    corners: HashSet<Point>,
    walls: HashSet<Point>,
    obstructions: HashSet<Point>,
}

#[allow(dead_code)]
impl Puzzle {
    fn print_map(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                print!("{}", self.final_map.get(&Point{x:c, y:r}).unwrap());
            }
            println!();
        }
    }

    fn print_map_dbg_idx(&self, map: HashMap<Point, String>) {
        print!("\n  ");
        for c in 0..self.width {
            print!("{c}");
        }
        println!();

        for r in 0..self.height {
            print!("{} ", r);
            for c in 0..self.width {
                print!("{}", map.get(&Point{x:c, y:r}).unwrap());
            }
            println!();
        }
    }
    
    fn print_init_map(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                print!("{}", self.init_map.get(&Point{x:c, y:r}).unwrap());
            }
            println!();
        }
    }

    fn print_custom_map(&self, map: &HashMap<Point, String>) {
        for r in 0..self.height {
            for c in 0..self.width {
                print!("{}", map.get(&Point{x:c, y:r}).unwrap());
            }
            println!();
        }
    }
    
    fn is_point_in_bounds(&self, pt: &Point) -> bool {
        return pt.x >= 0 && pt.x < self.width && pt.y >= 0 && pt.y < self.height;
    }

    fn run_guard_patrol(&mut self) {
        let mut pt = self.start_pt;
        let mut dir = self.start_dir;
        let mut path_iter = ["|", "-"].iter().cycle();
        let mut path_val = path_iter.next().unwrap();
        loop {
            self.visited.insert(pt.clone());
            self.path.push((pt.clone(), dir));
            // self.final_map.insert(pt.clone(), "X".to_string());
            self.final_map.insert(pt.clone(), path_val.to_string());

            let new_pt = walk(&pt, &dir);
            if !self.is_point_in_bounds(&new_pt) {
                break;
            }
            
            if self.init_map.get(&new_pt) == Some(&"#".to_string()) {
                self.corners.insert(pt.clone());
                path_val = path_iter.next().unwrap();
                dir = turn(dir);
                continue
            }
            pt = new_pt;
        }
        self.draw_corners();
    }

    fn draw_corners(&mut self) {
        for c in &self.corners {
            if *c != self.start_pt {
                self.final_map.entry(*c).and_modify(|v| *v="+".to_string());
            }
        }
        self.final_map.entry(self.start_pt).and_modify(|v| *v="^".to_string());
    }

}

fn parse(input: &str) -> Puzzle {
    let mut init_map: HashMap<Point, String> = HashMap::new();
    let mut walls: HashSet<Point> = HashSet::new();
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
            } else if ch == '#' {
                walls.insert(p.clone());
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
        path: vec![],
        corners: HashSet::new(),
        walls: walls,
        obstructions: HashSet::new(),
    };
    //println!("{:?}", puzzle);
    return puzzle;
}

fn part1(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    puzzle.run_guard_patrol();
    
    return puzzle.visited.len();
}

fn part2(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    puzzle.run_guard_patrol();

    puzzle.visited.remove(&puzzle.start_pt);
    for obs in &puzzle.visited {
        let mut cycle_path: HashSet<(Point, Direction)> = HashSet::new();

        let mut guard_pos = puzzle.start_pt;
        let mut guard_dir = puzzle.start_dir;
        loop {
            let next_pt = walk(&guard_pos, &guard_dir);
            if puzzle.walls.contains(&next_pt) || next_pt == *obs {
                guard_dir = turn(guard_dir);
            } else if cycle_path.contains(&(next_pt, guard_dir)) {
                puzzle.obstructions.insert(*obs);
                break;
            } else if puzzle.is_point_in_bounds(&guard_pos) {
                guard_pos = next_pt;
                cycle_path.insert((guard_pos, guard_dir));
            } else {
                break;
            }
        }
    }
    return puzzle.obstructions.len();
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
        assert_eq!(1, part2(include_str!("small2.txt")));
        assert_eq!(6, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(5331, part1(input));
        assert_eq!(1812, part2(input));
    }
}