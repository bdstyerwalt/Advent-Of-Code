use std::collections::HashMap;
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

#[derive(Debug)]
struct Puzzle {
    equations: HashMap<u64, Vec<u64>>,
}

impl Puzzle {
    fn calibration(&self, operators: &Vec<Operators>) -> u64 {
        let calibration_result = self.equations.iter().filter_map(|(target, numbers)| {
            let op_cnt = numbers.len() - 1;
            // Create every sequence of operator possible for numbers sequence
            (0..op_cnt).map(|_| operators).multi_cartesian_product().any(|seq| { 
                let mut s = seq.iter(); 
                // Copy numbers and try sequnce of operators
                let result = numbers.iter().copied().reduce(|acc, next_number| { 
                    let op = s.next().unwrap();
                    return op.eval(acc, next_number);
                }).unwrap();
                // if the target matches the result, return Some(target) for the filter
                return *target == result
            }).then_some(target)
        // sum the Some("passing equation's target") results 
        }).sum();

        return calibration_result;
    }
}

enum Operators {
    ADD,
    MULT,
    CONCAT,
}

impl Operators {
    fn eval(&self, left: u64, right: u64) -> u64 {
        match self {
            Operators::ADD => return left + right,
            Operators::MULT => return left * right,
            Operators::CONCAT => {
                let err_message = format!("Failed to parse {left}{right}");
                let temp = format!("{left}{right}").parse().expect(err_message.as_str());
                return temp;
            }
        }
    }
}

fn parse(input: &str) -> Puzzle {
    let mut equations: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in input.lines() {
        let mut nums = line.split(":");
        let k: u64 = nums.nth(0).unwrap().parse().unwrap();
        
        let vals: Vec<u64> = nums.nth(0).unwrap().trim().split_whitespace()
                                 .map(|x| x.trim().parse().unwrap()).collect();
        equations.insert(k, vals);
    }
    
    let puzzle = Puzzle { equations: equations };
    return puzzle;
}

fn part1(input_file: &str) -> u64 {
    let puzzle = parse(input_file);
    let ops = vec![Operators::ADD, Operators::MULT];
    let calibration_res = puzzle.calibration(&ops);

    return calibration_res;
}

fn part2(input_file: &str) -> u64 {
    let puzzle = parse(input_file);
    let ops = vec![Operators::ADD, Operators::MULT, Operators::CONCAT];
    let calibration_res = puzzle.calibration(&ops);

    return calibration_res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(3749, part1(include_str!("sample.txt")));
        assert_eq!(3774, part1(include_str!("sample2.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(11387, part2(include_str!("sample.txt")));
        assert_eq!(11412, part2(include_str!("sample2.txt")));
    }

    #[test]
    fn test_input() {
        assert_eq!(21572148763543, part1(include_str!("input.txt")));
        assert_eq!(581941094529163, part2(include_str!("input.txt")));
    }
}