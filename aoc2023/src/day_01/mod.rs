/*  The newly-improved calibration document consists of lines of text; 
    Each line originally contained a specific calibration value that 
    the Elves now need to recover. On each line, the calibration value
    can be found by combining the first digit and the last digit (in that order)
    to form a single two-digit number.
*/

use std::{fs, collections::HashMap};
use regex::Regex;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_01\\input.txt").expect("File not found!");

    println!("\n--Day 01------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}


fn part1(input_file: &String) -> i32 {
    let re = Regex::new(r"(\D*)(?<first>\d)(\w*\d*)(?<last>\d)(\w*)").unwrap();
    let single_num = Regex::new(r"(\D*)(?<first>\d)").unwrap();

    let mut acc: i32 = 0;
    for line in input_file.lines() {
        let first: i32;
        let last: i32;
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            first = caps["first"].parse().unwrap();
            last = caps["last"].parse().unwrap();
        } else if single_num.is_match(line) {
            let caps = single_num.captures(line).unwrap();
            first = caps["first"].parse().unwrap();
            last = first;
        } else {
            todo!()
        }
        acc += 10*first + last;
        //println!("{}{} {}\n--- ", first, last, acc);
    }
    return acc;
    
    
}

fn part2(input_file: &String) -> i32 {
    let numbers: HashMap<&str, i32> = HashMap::from([("one", 1), ("two", 2), ("three", 3), ("four", 4), 
                                                     ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), 
                                                     ("nine", 9), ("1", 1), ("2", 2), ("3", 3), ("4", 4), 
                                                     ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)]); 

    let mut acc: i32 = 0;

    for line in input_file.lines() {
        let mut first: usize = line.len();
        let mut last: usize = 0;

        let mut first_digit: i32 = 0; 
        let mut last_digit: i32 = 0; 
        for (key, val) in &numbers {
            let byte_loc: usize = match line.find(key) {
                Some(f_byte) => f_byte,
                None => continue,
            };
            if byte_loc <= first { 
                first = byte_loc;
                first_digit = *val;
            }
            let byte_loc: usize = match line.rfind(key) {
                Some(f_byte) => f_byte,
                None => continue,
            };
            if byte_loc >= last { 
                last = byte_loc;
                last_digit = *val;
            }
        }
        acc += 10*first_digit + last_digit;
        //println!("{}{}: {}", first_digit, last_digit, acc);
    }
    return acc;
}