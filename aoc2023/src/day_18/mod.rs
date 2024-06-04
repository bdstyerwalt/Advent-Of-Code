mod part1;
mod part2;
mod integer_polygons;

use std::fs;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_18\\input.txt").expect("File not found!");

    println!("\n--Day 18------");
    println!("Part 1: {}", &part1::process(&input_file));
    println!("Part 2: {}", &part2::process(&input_file));
    println!("--------------");
}