use std::collections::HashMap;

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
    println!("{:?}", puzzle.equations);
    let mut calibration_res = 0;
    for (total, mut nums) in puzzle.equations {
        let acc = nums[0];
        print!("\nSearching for {total} -> {acc}");
        nums = nums[1..].to_vec();
        if total == evaluate_mult(total, acc.clone(), nums.clone()) {
            print!(" -> SUCCESS");
            calibration_res += total;
        } else if total == evaluate_add(total, acc, nums) {
            print!(" -> SUCCESS");
            calibration_res += total;
        }
    }
    return calibration_res;
}

fn part2(input_file: &str) -> usize {
    let _puzzle = parse(input_file);

    return 0;
}

fn evaluate_mult(total: u64, acc: u64, nums: Vec<u64>) -> u64 {
    let res = acc * nums[0];
    if res == total {
        if nums.len() > 1 {
            return evaluate_add(total, acc, nums);
        }

        print!(" * {}", nums[0]);
        print!(" = {res}");
        return res;
    } else if res > total {
        return evaluate_add(total, acc, nums);
    } else if nums.len() > 1 {
        print!(" * {}", nums[0]);
        return evaluate_mult(total, res, nums[1..].to_vec());
    }
    return 0;
}

fn evaluate_add(total: u64, acc: u64, nums: Vec<u64>) -> u64 {
    print!(" + {}", nums[0]);
    let res = acc + nums[0];
    if res == total {
        if nums.len() > 1 {
            return 0;
        }

        print!(" = {res}");
        return res;
    } else if res < total && nums.len() > 1 {
        return evaluate_mult(total, res, nums[1..].to_vec());
    }
    // greater than total
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        // assert_eq!(3749, part1(include_str!("sample.txt")));
        assert_eq!(37741, part1(include_str!("sample2.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(0, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        // assert_eq!(9999, part1(include_str!("input.txt")));
        assert!(21498048347537 < part1(include_str!("input.txt")));
        
        assert_eq!(0, part2(include_str!("input.txt")));
    }
}