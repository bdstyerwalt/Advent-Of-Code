use std::{collections::{HashMap, VecDeque}, string, u64};

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
    fn calibrate_equations(&self, operators: &Vec<Operators>) -> u64 {
        let mut calibration_res = 0;
        for (total, nums) in &self.equations {
            let total = *total;
            println!("\nSearching for {total} -> {:?}", nums);
            
            let mut deq: VecDeque<(u64, Vec<u64>, Vec<String>)> = VecDeque::new();
            deq.push_front((nums[0].clone(), nums[1..].to_vec().clone(), vec![]));
            
            let mut eval = true;
            while let Some((acc, num_vec, mut op_vec)) = deq.pop_front() {
                if num_vec.get(0).is_none() || !eval {
                    break;
                }
                println!("{acc} __ {:?} __ {:?} ___ DEQ Size: {}", num_vec, op_vec, deq.len());
    
                let n = num_vec.get(0).unwrap();
                let rem_num = num_vec[1..].to_vec();
                let is_last_val = rem_num.len() == 0;
                for op in operators {
                    let (status, val) = Puzzle::test_operator(&total, acc, n.clone(), op, is_last_val);
                    match status {
                        Status::MATCH => {
                            op_vec.push(op.get_string()); 
                            println!("MATCH: {}\n", create_equation_string(total, nums.clone(), &op_vec));
                            calibration_res += val;
                            eval = false;
                            break;
                        },
                        Status::EVALUATING => {
                            let mut temp_op_vec = op_vec.clone();
                            temp_op_vec.push(op.get_string());
                            deq.push_back((val, rem_num.clone(), temp_op_vec))
                        },
                        Status::OVERFLOW => {},
                    }
                }
            }
        }
        return calibration_res;
    }

    fn test_operator(total: &u64, acc: u64, n: u64, op: &Operators, is_last_val: bool) -> (Status, u64) {
        let val = op.eval(acc, n);
        if val == *total && is_last_val {
            return (Status::MATCH, val);
        } else if val < *total {
            return (Status::EVALUATING, val);
        } else {
            return (Status::OVERFLOW, 0);
        }
    }
}

enum Status {
    MATCH,
    EVALUATING,
    OVERFLOW,
}
enum Operators {
    ADD,
    MULT,
    CONCAT,
}

impl Operators {
    fn get_string(&self) -> String {
        match self {
            Operators::ADD => return String::from("+"),
            Operators::MULT => return String::from("*"),
            Operators::CONCAT => return String::from("||"),
        }
    }

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
    let calibration_res = puzzle.calibrate_equations(&ops);

    return calibration_res;
}

fn part2(input_file: &str) -> usize {
    let _puzzle = parse(input_file);

    return 0;
}

fn create_equation_string(total: u64, nums: Vec<u64>, operators: &Vec<String>) -> String {
    let mut res = total.to_string() + " = " ;
    let num_strings: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    let space = " ".to_string();
    let it = num_strings.iter().interleave(operators).intersperse(&space);
    for c in it {
        res.push_str(c);
    }
    return res;
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
        assert_eq!(0, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        assert_eq!(21572148763543, part1(include_str!("input.txt")));
        assert_eq!(0, part2(include_str!("input.txt")));
    }
}