fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let mut lines = input.lines();

    let time: Vec<&str> = lines.nth(0).unwrap().split(":").collect();
    let time: i64 = time[1].replace(" ", "").parse().unwrap();
    println!("{:?}", time);

    let distance: Vec<&str> = lines.nth(0).unwrap().split(":").collect();
    let distance: i64 = distance[1].replace(" ", "").parse().unwrap();
    println!("{:?}", distance);

    let mut win_count: i64 = 0;
    for t in 0..time {
        let score = (time - t) * t;
        if score > distance {
            win_count += 1;
        }
    }

    return win_count;
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn part2_sample() {
        let input_file: &str = include_str!("sample.txt");
        assert_eq!(71503, process(&input_file));
    }
}