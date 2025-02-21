use std::fs;
use regex::Regex;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_03\\input.txt").expect("File not found!");

    println!("\n--Day 03------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn parse(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap(); // \d means digit
    let res: i32 = re.find_iter(input).fold(0, |acc, mat| {
        let mut mul_str = input[mat.start()..mat.end()].to_string();
        mul_str = mul_str.replace("mul(", "");
        mul_str = mul_str.replace(")", "");
        let nums: Vec<i32> = mul_str.split(",").map(|x| x.parse().unwrap()).collect();
        //println!("{:?} -> {:?}", nums, nums[0]*nums[1]);
        
        return acc + nums[0]*nums[1];
    });

    return res;
}

fn part1(input_file: &str) -> i32 {
    let res = parse(input_file);
    
    // println!("{:?}", res);
    return res;
}

fn part2(input_file: &str) -> i32 {    
    let _puzzle = parse(&input_file);
    let p2 = 0;
    return p2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(161, p1);
    }

    #[test]
    fn test_sample_p2() {
        let input = include_str!("sample.txt");
        let p2 = part2(input);
        dbg!(p2);
        assert_eq!(48, p2);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let (p1, p2) = (part1(input), part2(input));
        dbg!(p1, p2);
        assert_eq!(183788984, p1);
        assert_eq!(0, p2);
    }
}