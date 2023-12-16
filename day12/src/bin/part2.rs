use std::collections::HashMap;


fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let puzzles = parse_spring_groups(&input);

    let mut memo: HashMap<(&[char], &[usize]), usize> = HashMap::new();
    let result: usize = puzzles.iter().enumerate().map(|(i, puz)| {
        print!("{i} -> ");
        let res = SpringGrouping::check_score(&puz.pattern, &puz.numbers, &mut memo);
        println!("RESULT: {res}");
        return res;
    }).sum();
    
    return result;
}

fn parse_spring_groups(input: &str) -> Vec<SpringGrouping> {
    let mut puzzles: Vec<SpringGrouping> = vec![];

    let line_cnt = input.clone().lines().count();
    for (i, line) in input.lines().enumerate() {
        // println!("{} out of {}", i+1, line_cnt);
        let (springs, numbers) = line.split_once(" ").unwrap();

        let mut springs: Vec<char> = springs.chars().collect();
        springs.push('?');
        springs = springs.repeat(5);
        let _ = springs.pop();
        // println!("{:?}", springs);
        
        let mut numbers: Vec<usize> = numbers.split(",").map(|x| x.trim().parse().unwrap()).collect();
        numbers = numbers.repeat(5);
        // println!("{:?}", numbers);

        puzzles.push(SpringGrouping::new(springs, numbers));
    }
    return puzzles;
}

#[derive(Debug)]
struct SpringGrouping {
    pattern: Vec<char>,
    numbers: Vec<usize>,
}

impl SpringGrouping {
    fn new(pattern: Vec<char>, numbers: Vec<usize>) -> Self {
        Self {
            pattern: pattern,
            numbers: numbers
        }
    }

    fn check_score<'a>(pat: &'a [char], nums: &'a [usize], memo: &mut HashMap<(&'a [char], &'a [usize]), usize>) -> usize {
        match memo.get(&(pat, nums)) {
            Some(val) => return *val,
            None => (),
        }

        if nums.is_empty() {
            return (!pat.contains(&'#')) as usize;
        }

        if pat.len() < (nums.iter().sum::<usize>() + nums.len() - 1) {
            return 0;
        }

        let result = match pat[0] {
            '.' => Self::check_score(&pat[1..], nums, memo),
            '#' => Self::pound_town(pat, nums, memo),
            '?' => Self::check_score(&pat[1..], nums, memo) + Self::pound_town(pat, nums, memo),
            _ => panic!("Invalid char in check score")
        };
        memo.insert((pat, nums), result);
        return result;
    }

    fn pound_town<'a>(pat: &'a [char], nums: &'a [usize], memo: &mut HashMap<(&'a [char], &'a [usize]), usize>) -> usize {
        // if there isn't a chance to fit the number in the patter or
        // there is a '.' where the number would match, fail the check
        if pat.len() < nums[0] || pat[0..nums[0]].contains(&'.') {
            return 0;
        }

        // if we have reached the last pound group in recurrsion
        if pat.len() == nums[0] {
            let res = (nums.len() == 1) as usize;
            return res;
        }

        // if the pound group extends beyond the size of the number
        if pat[nums[0]] == '#' {
            return 0;
        }

        // if the number matched the pound group but wasn't 
        // the end of the pattern, continue recurrsive check
        return Self::check_score(&pat[nums[0]+1..], &nums[1..], memo);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(525152, process(input));
    }
}