use std::fs;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_02\\input.txt").expect("File not found!");

    println!("\n--Day 02------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    
    for line in input.lines() {
        let levels = line.trim().split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        reports.push(levels);
    }
    
    // println!("{:?}", reports);
    return reports;
}

fn part1(input_file: &str) -> i32 {
    let reports = parse(&input_file);
    
    let mut safe_report_count = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_report_count += 1
        }
    }
    
    return safe_report_count;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    // A report only counts as safe if both of the following are true:
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three
    let res = report.iter()
                    .zip(report.iter().skip(1))
                    //.inspect(|(a, b)| println!("a: {}, b: {}, diff: {}", a, b, *b-*a))
                    .map(|(&a, &b)| b-a)
                    .collect::<Vec<_>>();
    let all_inc = res.iter().all(|&item| item > 0);
    let all_dec = res.iter().all(|&item| item < 0);
    
    let res: Vec<i32> = res.iter().map(|&a| a.abs()).collect();
    let all_lt3 = res.iter().all(|&item| item > 0 && item <= 3);
    
    return (all_inc || all_dec) && all_lt3;
}

fn part2(input_file: &str) -> i32 {
    let reports = parse(&input_file);
    
    let mut safe_report_count = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_report_count += 1;
            continue;
        }

        // Check if removing one item fixes the report
        for i in 0..report.len() {
            let mut partial: Vec<i32> = report.clone();
            partial.remove(i);
            if is_report_safe(&partial) {
                safe_report_count += 1;
                break;
            }
        }
    }
    
    return safe_report_count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(2, p1);
    }

    #[test]
    fn test_sample_p2() {
        let input = include_str!("sample.txt");
        let p2 = part2(input);
        dbg!(p2);
        assert_eq!(4, p2);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let (p1, p2) = (part1(input), part2(input));
        dbg!(p1, p2);
        assert_eq!(306, p1);
        assert_eq!(366, p2);
    }
}