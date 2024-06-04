use std::{fs, collections::HashSet};

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_03\\input.txt").expect("File not found!");

    println!("\n--Day 03------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn part1(input_file: &String) -> i32 {
    
    let symbol_locs: HashSet<(i32, i32)> = find_symbols(input_file);
    let number_locs: Vec<(i32, i32, i32, i32)> = find_numbers(input_file);

    let mut acc: i32 = 0;
    for (val, row, left, right) in number_locs {
        let mut border_points: HashSet<(i32, i32)> = HashSet::new();
        let (row, left, right) = (row as i32, left as i32, right as i32);
        //println!("----------\n{}: {}, ({},{})", val, row, left, right);
        for y in row-1..=row+1 {
            for x in left-1..=right+1 {
                //println!("{},{}", y, x);
                border_points.insert((y, x));
            }
        }
        let intersect = border_points.intersection(&symbol_locs);
        if intersect.count() > 0 {
            acc += val;
            //println!("INTERSECT: {}->{}", acc-val, acc);
        }
    }
    return acc;
}

fn find_symbols(input_file: &String) -> HashSet<(i32, i32)> {
    let mut symbol_locs: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in input_file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                //println!("{} {},{}", &c.to_string(), row, col);
                symbol_locs.insert((row as i32, col as i32));
            }
        }
    }
    return symbol_locs;
}

fn find_numbers(input_file: &String) -> Vec<(i32, i32, i32, i32)> {
    let mut number_locs: Vec<(i32, i32, i32, i32)> = Vec::new();
    
    for (row, line) in input_file.lines().enumerate() {
        let mut active_num: bool = false;
        let mut val: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = 0;
        for (col, c) in line.chars().enumerate() {
            if !active_num && !c.is_numeric() {
                continue
                
            } else if !active_num && c.is_numeric() {
                active_num = true;
                (val, left, right) = (c.to_string().parse().unwrap(), col as i32, col as i32);
                if col == line.len()-1 {
                    number_locs.push((val, row as i32, left, right));
                }

            } else if active_num && c.is_numeric() {
                (val, right) = (val*10 + c.to_string().parse::<i32>().unwrap(), col as i32);
                //println!("END OF LINE: {}, {}", col, line.len()-1);
                if col == line.len()-1 { // EOL condition
                    //println!("{}: {}, ({},{})", val, row, left, right);
                    number_locs.push((val, row as i32, left, right));
                }
                
            } else if active_num && !c.is_numeric() {
                active_num = false;
                //println!("{}: {}, ({},{})", val, row, left, right);
                number_locs.push((val, row as i32, left, right))
            }
        }
    }
    return number_locs;

}

/* ---------------------------------------------------------------- */

fn part2(input_file: &String) -> i32 { 
    let gear_locs: HashSet<(i32, i32)> = find_gears(input_file);
    let number_locs: Vec<(i32, i32, i32, i32)> = find_numbers(input_file);

    let mut acc: i32 = 0;
    for (gear_row, gear_col) in gear_locs {
        let mut touching: i32 = 0;
        let mut gear_ratio: i32 = 1;
        for (val, row, left, right) in &number_locs {
            let mut border_points: HashSet<(i32, i32)> = HashSet::new();
            let (row, left, right) = (*row as i32, *left as i32, *right as i32);
            //println!("----------\n{}: {}, ({},{})", val, row, left, right);
            for y in row-1..=row+1 {
                for x in left-1..=right+1 {
                    //println!("{},{}", y, x);
                    border_points.insert((y, x));
                }
            }
        
            if border_points.contains(&(gear_row, gear_col)) {
                // println!("* at ({},{}) touches {}", gear_row, gear_col, val);
                touching += 1;
                gear_ratio *= val;
            }
            if touching == 2 {
                // println!("Gear ratio is -> {}", gear_ratio);
                acc += gear_ratio;
                break;
            }
        }
    }
    return acc;
}

fn find_gears(input_file: &String) -> HashSet<(i32, i32)> {
    let mut gear_locs: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in input_file.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '*' {
                //println!("{} {},{}", &c.to_string(), row, col);
                gear_locs.insert((row as i32, col as i32));
            }
        }
    }
    return gear_locs;
}