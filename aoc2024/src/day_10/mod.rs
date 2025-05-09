use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

pub fn run() {
    let day_idx = file!().find("day_").expect("Couldn't find `day_` in file path") + 4;
    let day = file!().get(day_idx..day_idx+2).unwrap();
    let input_file = include_str!("input.txt");

    println!("\n--Day {day}------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Puzzle {
    trailheads: HashMap<Point, usize>,
    trail_maps: HashMap<Point, HashSet<Point>>,
    topo_map: HashMap<Point, usize>,
    width: isize,
    height: isize,
    target: usize,
}

#[allow(dead_code)]
impl Puzzle {
    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let pt = Point{x:col, y:row};
                let val = self.topo_map.get(&pt).unwrap();
                print!("{val}");
            }
            println!("");
        }
        println!("");
    }
    
    fn print_trail_maps(&self) {
        for th in self.trailheads.keys() {
            let trail_set = self.trail_maps.get(&th).unwrap();
            println!("Printing for TH {:?} -> {:?}", th, trail_set);
            
            for row in 0..self.height {
                for col in 0..self.width {
                    let pt = Point{x:col, y:row};
                    
                    if trail_set.contains(&pt) {
                        let val = self.topo_map.get(&pt).unwrap();
                        print!("{val}");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            println!("");
        }
    }
    
    fn print_trail_scores(&self) {
        for (th, score) in &self.trailheads {
            println!("Trail starting at ({},{}) has score {}", th.x, th.y, score);
        }
    }
}

fn parse(input: &str) -> Puzzle {
    let mut trailheads: HashMap<Point, usize> = HashMap::new();
    let mut topo_map: HashMap<Point, usize> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let num: usize = ch.to_string().parse().unwrap();
            if num == 0 {
                trailheads.insert(Point{x:col as isize, y:row as isize}, 0);
            }
            topo_map.insert(Point{x:col as isize, y:row as isize}, num);
        }
        width = line.len();
        height = row+1;
    }
    
    let puzzle = Puzzle { 
        trailheads: trailheads,
        trail_maps: HashMap::new(),
        topo_map: topo_map,
        width: width as isize,
        height: height as isize,
        target: 9,
    };
    
    // println!("{:?}", puzzle);
    // puzzle.print();
    return puzzle;
}

fn find_next_steps(topo_map: &HashMap<Point, usize>, pt: Point, step: usize) -> Vec<Point> {
    let mut dirs = vec![];
    let max_loc = topo_map.keys().max().unwrap();
    
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = pt.x + dx;
        let y = pt.y + dy;
        
        if x < 0 || x > max_loc.x || y < 0 || y > max_loc.y {
            continue
        }
        
        let new_pt = Point{x,y};
        if topo_map.get(&new_pt).unwrap() == &step {
            dirs.push(new_pt);
        }
    }
    return dirs;
}

fn part1(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    let mut deque: VecDeque<(Point, Point, usize, HashSet<Point>)> = VecDeque::new();
    // Add all Trailheads to the deque
    for th in puzzle.trailheads.keys() {
        deque.push_back((*th, *th, 0, HashSet::new()));
    }

    while let Some((head, loc, step, mut pt_set)) = deque.pop_front() {
        pt_set.insert(loc.clone());
        
        if step == puzzle.target {
            match puzzle.trail_maps.get(&head) {
                Some(tmap) => {
                    if !tmap.contains(&loc) {
                        puzzle.trailheads.entry(head).and_modify(|v| *v+=1).or_insert(1);
                    }
                },
                None => { puzzle.trailheads.entry(head).and_modify(|v| *v+=1).or_insert(1); }
            };
            puzzle.trail_maps.entry(head).and_modify(|v| v.extend(pt_set.clone())).or_insert(pt_set.clone());
            continue
        }
        
        let next_step = step+1;
        for dir in find_next_steps(&puzzle.topo_map, loc, next_step) {
            deque.push_back((head, dir, next_step, pt_set.clone()));
        }
    }
    // puzzle.print_trail_maps();
    // puzzle.print_trail_scores();
    return puzzle.trailheads.values().sum();
}

fn part2(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    let mut deque: VecDeque<(Point, Point, usize, HashSet<Point>)> = VecDeque::new();
    // Add all Trailheads to the deque
    for th in puzzle.trailheads.keys() {
        deque.push_back((*th, *th, 0, HashSet::new()));
    }

    while let Some((head, loc, step, mut pt_set)) = deque.pop_front() {
        // println!("{:?} - {:?}", head, pt_set);
        pt_set.insert(loc.clone());
        
        if step == puzzle.target {
            puzzle.trailheads.entry(head).and_modify(|v| *v+=1).or_insert(1);
            puzzle.trail_maps.entry(head).and_modify(|v| v.extend(pt_set.clone())).or_insert(pt_set.clone());
            continue
        }
        
        let next_step = step+1;
        for dir in find_next_steps(&puzzle.topo_map, loc, next_step) {
            deque.push_back((head, dir, next_step, pt_set.clone()));
        }
    }
    // puzzle.print_trail_maps();
    // puzzle.print_trail_scores();
    return puzzle.trailheads.values().sum();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(36, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(81, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(798, part1(input));
        assert_eq!(1816, part2(input));
    }
}