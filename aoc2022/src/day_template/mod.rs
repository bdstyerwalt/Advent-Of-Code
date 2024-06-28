use std::fs;

pub fn run() {
    println!("\n--Day {}------", get_day());
    println!("Part 1: {}", &part1(&get_file("input")));
    println!("Part 2: {}", &part2(&get_file("input")));
    println!("--------------");
}

fn part1(input: &str) -> u32 {
   
}

fn part2(input: &str) -> u32 {
    
}

fn get_file(file: &str) -> String {
    return fs::read_to_string(format!("src\\day_{}\\{file}.txt", get_day())).expect("File not found!");
}

fn get_day() -> &str {
    return "03";
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1_sample() {
        let result = part1(&get_file("sample"));
        assert_eq!(result, 15)
    }
    
    #[test]
    fn p1_input() {
        let result = part1(&get_file("input"));
        assert_eq!(result, 12276)
    }
    
    #[test]
    fn p2_sample() {
        let result = part2(&get_file("sample"));
        assert_eq!(result, 12)
    }
    
    #[test]
    fn p2_input() {
        let result = part2(&get_file("input"));
        assert_eq!(result, 9975)
    }
}