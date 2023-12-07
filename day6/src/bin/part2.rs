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
    use std::fs;
    use crate::process;

    #[test]
    fn part2_sample() {
        let input_file: String = fs::read_to_string("sample.txt").expect("Couldn't read file");
        assert_eq!(35, process(&input_file));
    }
}