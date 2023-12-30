fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input);
    let p2 = part2(input);
    println!("Part 1: {} | Part 2: {}", p1, p2);
}

fn parse(input: &str) -> Puzzle { 
    todo!();
}

fn part1(input: &str) -> u32 {
    todo!();
}

fn part2(input: &str) -> u32 {
    todo!();
}


struct Puzzle {
    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(51, process(input));
    }
}