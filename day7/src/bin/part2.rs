fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn part2_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(6440, process(&input));
    }
}