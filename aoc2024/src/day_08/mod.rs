use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn run() {
    let day_idx = file!().find("day_").expect("Couldn't find `day_` in file path") + 4;
    let day = file!().get(day_idx..day_idx+2).unwrap();
    let input_file = include_str!("input.txt");

    println!("\n--Day {day}------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn get_distance_to_point(&self, pt: Point) -> isize {
        let dx = self.x - pt.x;
        let dy = self.y - pt.y;
        // println!("{:?}, {:?} -> {x}, {y} -> {:?}, {:?}", self, pt, x*x, y*y);
        return (dx*dx + dy*dy).isqrt();
    }
    
    // fn get_slope(&self, pt: Point) -> isize {
    //     let x = self.x - pt.x;
    //     let y = self.y - pt.y;
    //     return y / x;
    // }
}

#[derive(Debug)]
struct Puzzle {
    init_map: HashMap<Point, char>,
    antennas: HashMap<String, Vec<Point>>,
    antinodes: HashMap<String, Vec<Point>>,
    anti_locs: HashSet<Point>,
    width: isize,
    height: isize,
}

impl Puzzle {
    fn print(&self) {
        println!("KEYS: {:?}", self.antennas.keys());
        for row in 0..self.height {
            for col in 0..self.width {
                let pt = Point{x:col, y:row};
                let val = self.init_map.get(&pt).unwrap().to_string();
                print!("{val}");
            }
            
            for key in self.antennas.keys() {
                // print!("  ->  ");
                let nodes = match self.antinodes.get(&key.to_string()) {
                    Some(antinode_vec) => &antinode_vec,
                    None => &vec![],
                };
            
                for col in 0..self.width {
                    let pt = Point{x:col, y:row};
                    let val = self.init_map.get(&pt).unwrap().to_string();
                   
                    if nodes.contains(&pt) {
                        print!("#");
                    } else if &val == key {
                        print!("{val}");
                    } else {
                        print!(".");
                    }
                }
            }
            
            print!("  ->  ");
            for col in 0..self.width {
                let pt = Point{x:col, y:row};
                let val = self.init_map.get(&pt).unwrap().to_string();
               
                if self.anti_locs.contains(&pt) {
                    print!("#");
                } else {
                    print!("{val}");
                }
            }
            println!();
        }
    }

    fn is_point_in_bounds(&self, pt: Point) -> bool {
        return pt.x >= 0 && pt.x < self.width && pt.y >= 0 && pt.y < self.height;
    }
}

fn parse(input: &str) -> Puzzle {
    let mut map: HashMap<Point, char> = HashMap::new();
    let mut antennas: HashMap<String, Vec<Point>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let pt = Point{x:col as isize, y:row as isize};
            let node = ch.to_string();
            map.insert(pt, ch);
            if ch != '.' {
                antennas.entry(node).and_modify(|val| val.push(pt)).or_insert(vec![pt]);
            }
            width = col;
        }
        height = row;
    }
    
    
    let puzzle = Puzzle { 
        init_map: map,
        antennas: antennas,
        antinodes: HashMap::new(),
        anti_locs: HashSet::new(),
        width: (width + 1) as isize,
        height: (height + 1) as isize,
    };
    // println!("{:?}", puzzle);
    return puzzle;
}

fn part1(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);

    for ant in puzzle.antennas.keys().clone() {
        let locs = puzzle.antennas.get(ant).unwrap().into_iter().combinations(2);
        // println!("\n{ant}");
        for points in locs {
            let (anti1, anti2) = project_antinodes(*points[0], *points[1]);
            let dist = anti1.get_distance_to_point(anti2);
            // println!("    Antinodes: {:?},{:?} and {:?},{:?} -> dist: {dist}", anti1.x, anti1.y, anti2.x, anti2.y);
            
            if puzzle.is_point_in_bounds(anti1) {
                puzzle.anti_locs.insert(anti1);
                puzzle.antinodes.entry(ant.to_string()).and_modify(|val| val.push(anti1)).or_insert(vec![anti1]);
            }
            
            if puzzle.is_point_in_bounds(anti2) {
                puzzle.anti_locs.insert(anti2);
                puzzle.antinodes.entry(ant.to_string()).and_modify(|val| val.push(anti2)).or_insert(vec![anti2]);
            }
        }
    }
    // println!("{:?}", puzzle.antinodes);
    // puzzle.print();
    return puzzle.anti_locs.len();
}

fn part2(input_file: &str) -> usize {
    let _puzzle = parse(input_file);

    return 0;
}

fn project_antinodes(mut pt1: Point, mut pt2: Point) -> (Point, Point) {
    let dist = pt1.get_distance_to_point(pt2);
    let dx = pt1.x - pt2.x;
    let dy = pt1.y - pt2.y;
    // print!("{:?},{:?} to {:?},{:?} -> dist: {dist}, dx: {dx}, dy: {dy}", pt1.x, pt1.y, pt2.x, pt2.y);
    
    pt1.x += dx;
    pt1.y += dy;
    
    pt2.x -= dx;
    pt2.y -= dy;
    
    return (pt1, pt2)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(14, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(0000, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        // assert_eq!(0000, part1(input));
        // assert_eq!(0000, part2(input));
    }
}