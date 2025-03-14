use std::fs;
use std::collections::HashMap;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_04\\input.txt").expect("File not found!");

    println!("\n--Day 04------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Puzzle {
    width: i32,
    height: i32,
    grid: HashMap<Point, String>,
    search_locations: Vec<Point>,
    matches: Vec<Vec<Point>>,
}

fn parse(input: &str, first_letter: char) -> Puzzle {
    let mut grid: HashMap<Point, String> = HashMap::new();
    let mut search: Vec<Point> = vec![];
    let mut y = 0;
    let mut x = 0;
    
    for line in input.lines() {
        x = 0;
        for letter in line.chars() {
            let pt = Point {x, y};
            grid.insert(pt.clone(), letter.to_string());
            if letter == first_letter {
                search.push(pt)
            }
            x += 1;
        }
        y += 1;
    }
    
    let puzzle = Puzzle {
        width: x, 
        height: y, 
        grid: grid,
        search_locations: search,
        matches: vec![],
    };
    // println!("{:?}", puzzle);
    return puzzle;
}

fn part1(input_file: &str) -> usize {
    let dirs: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    
    let search_pattern = "XMAS";
    let mut puzzle = parse(input_file, search_pattern.chars().nth(0).unwrap());
    //println!("Search Locs: {:?}\n", puzzle.search_locations);
    
    for start_pt in puzzle.search_locations.iter() {
        // println!("Start {:?}\n", start_pt);
        for (x_dir, y_dir) in &dirs {
            let mut pt = start_pt.clone();
            let mut word = puzzle.grid.get(&pt).unwrap().clone();
            let mut word_pt_vec = vec![pt.clone()];
            
            // println!("Start  x:{}, y:{} = {}", pt.x, pt.y, word);
            for i in 1..search_pattern.len() {
                pt.x = pt.x + x_dir;
                pt.y = pt.y + y_dir;
                if pt.x < 0||pt.x >= puzzle.width || pt.y < 0 || pt.y >= puzzle.height {
                    // println!("FAIL BOUNDS\n-----\n");
                    break;
                }
                let new_lt = puzzle.grid.get(&pt).unwrap();
                
                word.push_str(new_lt);
                word_pt_vec.push(pt.clone());
                // println!("Search x:{}, y:{} = {} => {}", pt.x, pt.y, new_lt, word);
                
                //println!("{}) pattern: {:?}", i+1, puzzle.pattern[0..i+1].to_string());
                if word != search_pattern[0..i+1] {
                    // println!("FAIL WORD\n-----\n");
                    break;
                }
            }
            
            if word == search_pattern {
                puzzle.matches.push(word_pt_vec);
                // println!("new word={:?}", word);
                // println!("SUCCESS\n-----\n");
            }
        }
    }
    
    return puzzle.matches.len();
}

fn part2(input_file: &str) -> usize {
    //                               (TpLeft/BtmRight) (TpRight/BtmLeft)
    let dirs: Vec<(i32, i32)> = vec![(-1, -1), (1, 1), (-1, 1), (1, -1)];
    
    let mut puzzle = parse(input_file, 'A');
    // println!("Search Locs: {:?}\n", puzzle.search_locations);
    for start_pt in puzzle.search_locations.iter() {
        if start_pt.x == 0 || start_pt.x == puzzle.width-1 || start_pt.y == 0 || start_pt.y == puzzle.height-1 {
            // if pt is on edge, skip because we can't form a full 3x3 grid
            // println!("Skipping {:?}\n", start_pt);
            continue;
        }
        // println!("Start {:?}\n", start_pt);
        
        let mut grid_string: String = "".to_string(); 
        for (dx, dy) in dirs.iter() {
            let x = start_pt.x + dx;
            let y = start_pt.y + dy;
            let pt = Point { x, y };
            grid_string.push_str(puzzle.grid.get(&pt).unwrap());
        }
        
        match grid_string.as_str() {
            "MSMS" | "MSSM" | "SMMS" | "SMSM" => {
                puzzle.matches.push(vec![start_pt.clone()]);
            },
            _ => {
                //println!("{grid_string} did not match!")
            },
        }
    }
    return puzzle.matches.len();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(18, p1);
    }

    #[test]
    fn test_sample_p2() {
        let input = include_str!("sample.txt");
        let p1 = part2(input);
        dbg!(p1);
        assert_eq!(9, p1);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let (p1, p2) = (part1(input), part2(input));
        dbg!(p1, p2);
        assert_eq!(2543, p1);
        assert_eq!(62098619, p2);
    }
}