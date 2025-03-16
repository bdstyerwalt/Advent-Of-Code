use std::{collections::{HashMap, VecDeque}, u64};

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
    fn calibrate_equations(&self, operators: &Vec<String>) -> u64 {
        let mut calibration_res = 0;
        for (total, nums) in &self.equations {
            let total = *total;
            // println!("\nSearching for {total} ->");
            
            let mut deq: VecDeque<(u64, Vec<u64>, Vec<String>)> = VecDeque::new();
            deq.push_front((nums[0].clone(), nums[1..].to_vec().clone(), vec![]));
            // println!("{:?}", deq);
            
            while let Some((acc, num_vec, mut op_vec)) = deq.pop_front() {
                // println!("{acc} __ {:?} __ {:?} ___ DEQ Size: {}", num_vec, op_vec, deq.len());
    
                if let None = num_vec.get(0) {
                    // println!("RAN OUT OF NUMBERS...\n");
                    break;
                }
    
                let n = num_vec.get(0).unwrap();
                let rem_num = num_vec[1..].to_vec();
                // println!("REM: {:?}", rem_num);
    
                let mul = acc * n;
                if mul == total  && rem_num.len() == 0 {
                    op_vec.push("*".to_string());
                    println!("{}", create_equation_string(total, nums.clone(), &op_vec));
                    calibration_res += mul;
                    break;
                } else if mul < total {
                    // clone so we don't affect the op string for the add branch 
                    let mut mul_op_vec = op_vec.clone(); 
                    mul_op_vec.push("*".to_string());
                    deq.push_back((mul, rem_num.clone(), mul_op_vec));
                }
    
                let add = acc + n;
                if add == total && rem_num.len() == 0 {
                    op_vec.push("+".to_string());
                    println!("{}", create_equation_string(total, nums.clone(), &op_vec));
                    calibration_res += add;
                    break;
                } else if add < total {
                    op_vec.push("+".to_string());
                    deq.push_back((add, rem_num, op_vec));
                }
            }
        }
        return calibration_res;
    }
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
    let calibration_res = puzzle.calibrate_equations(&vec![]);

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