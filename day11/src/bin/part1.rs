fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    todo!()
}

mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = include_str!("sample.txt");
        todo!()
        // assert_eq!(4, process(input));
    }

    #[test]
    fn test_part1_soln() {
        let input = include_str!("input.txt");
        todo!()
        // assert_eq!(579, process(input));
    }
}