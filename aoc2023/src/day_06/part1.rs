pub fn process(input: &str) -> i32 {
    let mut lines = input.lines();

    let times: Vec<&str> = lines.nth(0).unwrap().split(":").collect();
    let times: Vec<i32> = times[1].split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    // println!("{:?}", times);

    let distances: Vec<&str> = lines.nth(0).unwrap().split(":").collect();
    let distances: Vec<i32> = distances[1].split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    // println!("{:?}", distances);

    let mut win_count: i32 = 1;
    for i in 0..times.len() {
        let mut game_score: i32 = 0;
        let time = times[i];
        let dist = distances[i];

        for t in 0..time {
            let score = (time - t) * t;
            if score > dist {
                game_score += 1;
            }
        }
        win_count = win_count * game_score;
    }

    return win_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(288, process(&input));
    }
}