use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::usize;
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x: x, y: y }
    }

    fn same_x(p1: &Point, p2: &Point) -> bool {
        return p1.x == p2.x;
    }

    fn same_y(p1: &Point, p2: &Point) -> bool {
        return p1.y == p2.y;
    }

    fn complete_the_square(p1: &Point, p2: &Point, p3: &Point) -> Option<Self> {
        if Point::same_x(p1, p2) && Point::same_y(p1, p3) {
            return Some(Point { x: p3.x, y: p2.y });
        }
        return None;
    }
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

    fn print_map_dbg_idx(&self) {
        print!("\n  ");
        for c in 0..self.width {
            print!("{c}");
        }
        println!();

        for r in 0..self.height {
            print!("{} ", r);
            for c in 0..self.width {
                print!("{}", self.final_map.get(&Point{x:c, y:r}).unwrap());
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
            self.final_map.entry(*c).and_modify(|v| *v="+".to_string());
        }
    }

    fn print_obs_dbg(&self, corners: Vec<Point>, obstacle: Point) {
        let mut obs_map = self.final_map.clone();
        let xs: Vec<isize> = corners.iter().map(|p| p.x).collect();
        let ys: Vec<isize> = corners.iter().map(|p| p.y).collect();

        let min_x = *xs.iter().min().unwrap();
        let max_x = *xs.iter().max().unwrap();
        
        let min_y = *ys.iter().min().unwrap();
        let max_y = *ys.iter().max().unwrap();

        for x in min_x+1..max_x {
            obs_map.entry(Point::new(x, min_y)).and_modify(|v| *v="-".to_string());
            obs_map.entry(Point::new(x, max_y)).and_modify(|v| *v="-".to_string());
        }

        for y in min_y+1..max_y {
            obs_map.entry(Point::new(min_x, y)).and_modify(|v| *v="|".to_string());
            obs_map.entry(Point::new(max_x, y)).and_modify(|v| *v="|".to_string());
        }

        for c in corners.iter().chain(vec![&obstacle]) {
            obs_map.entry(*c).and_modify(|v| *v="+".to_string());
        }

        obs_map.entry(obstacle).and_modify(|v| *v="O".to_string());
        self.print_custom_map(&obs_map);
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
    puzzle.print_map();

    // for v in &puzzle.corners {
        // println!("{:?}", v);
    // }

    for (p, d) in &puzzle.path {
        let new_d = turn(*d);
        let mut potential = walk(&p, &new_d);
        // println!("\npot: {:?}", potential);

        while puzzle.is_point_in_bounds(&potential) {
            print!("Walking {:?}: ", new_d);
            println!("{:?}", potential);

            if puzzle.path.contains(&(potential, new_d)){
                let obs = walk(&p, &d);
                
                if puzzle.walls.contains(&obs) {
                    println!();
                    break;
                }
                
                println!("obs: {:?}\n", obs);
                // let mut map = puzzle.final_map.clone();
                // map.entry(*p).and_modify(|v| *v="+".to_string());
                // map.entry(obs).and_modify(|v| *v="O".to_string());
                puzzle.final_map.entry(obs).and_modify(|v| *v="O".to_string());
                // puzzle.print_custom_map(&map);
                puzzle.obstructions.insert(obs);
                break;
            }
        
            // println!("Walking {:?}", new_d);
            potential = walk(&potential, &new_d);
            // println!("pot: {:?}", potential);
        }
    }
    puzzle.print_map_dbg_idx();

    return puzzle.obstructions.len();
}

#[allow(dead_code)]
fn junk(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    puzzle.run_guard_patrol();
    puzzle.draw_corners();


    // Add the starting location since it is invalid
    puzzle.walls.insert(puzzle.start_pt);

    let mut obstructions: HashSet<Point> = HashSet::new();
    let cv: Vec<Point> = puzzle.corners.clone().into_iter().collect();
    for mut combo in cv.iter().combinations(3) {
        combo.sort();
        for p in combo.iter().permutations(3) {
            let (p1, p2, p3) = (p[0], p[1], p[2]);
            
            if let Some(pt) = Point::complete_the_square(p1, p2, p3) {
                let corner_pts = vec![**p1, **p2, **p3];
                let obs = find_corner_location(corner_pts.clone(), pt);
                if puzzle.walls.contains(&obs) {//&& puzzle.visited.contains(&pt) {
                    continue;
                }
                println!("\n@@@({},{})", obs.x, obs.y);
                
                // puzzle.print_obs_dbg(corner_pts, pt);
                
                puzzle.final_map.entry(obs).and_modify(|v| *v="O".to_string());
                obstructions.insert(obs);
                break;
            }
        }
    }

    puzzle.print_map();

    return obstructions.len();
}

fn find_corner_location(corner_points: Vec<Point>, pt: Point) -> Point {
    let xs: Vec<isize> = corner_points.iter().map(|p| p.x).collect();
    let ys: Vec<isize> = corner_points.iter().map(|p| p.y).collect();

    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();

    if pt.x == min_x && pt.y == min_y {
        // top left
        return Point::new(pt.x, pt.y-1);
    } else if pt.x == max_x && pt.y == max_y {
        // bottom right
        return Point::new(pt.x, pt.y+1);
    } else if pt.x == min_x && pt.y == max_y {
        // bottom left
        return Point::new(pt.x-1, pt.y);
    } else {
        // top right
        return Point::new(pt.x+1, pt.y);
    }

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
        // assert_eq!(0, part2(input));
    }
}