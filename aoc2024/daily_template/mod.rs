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

}

fn parse(input: &str) -> Puzzle {

    //println!("{:?}", puzzle);
    return Puzzle { };
}

fn part1(input_file: &str) -> usize {
    let puzzle = parse(input_file);

    return 0;
}

fn part2(input_file: &str) -> usize {
    let _puzzle = parse(input_file);

    return 0;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(0000, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(0000, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(0000, part1(input));
        assert_eq!(0000, part2(input));
    }
}