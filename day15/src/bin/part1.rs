
fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let steps = parse(input);
    return steps.iter().map(|s| {
        println!("{s}");
        hash_step(s)
    }).sum::<u32>();
}

fn parse(input: &str) -> Vec<&str> {
    let steps: Vec<&str> = input.split(",").collect();
    return steps;
}

fn hash_step(step: &str) -> u32 {
    let val = step.chars().fold(0, |acc, c| {
        ((acc + c as u32) * 17) % 256
    });
    return val;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(1320, process(input));
    }
}
