use std::fs;

pub fn run() {
    println!("\n--Day 02------");
    println!("Part 1: {}", &part1(&get_file("input")));
    println!("Part 2: {}", &part2(&get_file("input")));
    println!("--------------");
}

fn part1(input: &str) -> u32 {
    let guide_score: u32 = input.lines().fold(0, |acc, line| {
        let shapes = line.split(" ").collect::<Vec<&str>>();
        let (opp, you) = (shapes[0], shapes[1]);
        acc + play_game(opp, you)
    });
    return guide_score;
}

fn part2(input: &str) -> u32 {
    let guide_score: u32 = input.lines().fold(0, |acc, line| {
        let shapes = line.split(" ").collect::<Vec<&str>>();
        let (opp, game_res) = (shapes[0], shapes[1]);
        acc + play_game_p2(opp, game_res)
    });
    return guide_score;
}

fn play_game(opp: &str, you: &str) -> u32 {
    let mut score = 0;
    
    let win = 6;
    let draw = 3;
    // score game results
    match (opp, you) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => score += win,
        ("A", "X") | ("B", "Y") | ("C", "Z") => score += draw,
        _ => ()
    }
    
    // add selected shape score
    match you {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => ()
    }
    
    // println!("Input {opp}, {you} => {score}");
    
    return score;
}

fn play_game_p2(opp: &str, game_result: &str) -> u32 {
    let mut score = 0;
    let win = 6;
    let draw = 3;
    
    match game_result {
        "X" => {
            match opp {
                "A" => score += 3, // need sisccors
                "B" => score += 1, // need rock
                "C" => score += 2, // need paper
                _ => ()
            }
        }, // lose
        
        "Y" => {
            match opp {
                "A" => score += 1, // need rock
                "B" => score += 2, // need paper
                "C" => score += 3, // need scissors
                _ => ()
            }
            score += draw;
        }, // draw
        
        "Z" => {
            match opp {
                "A" => score += 2, // need paper
                "B" => score += 3, // need scissors
                "C" => score += 1, // need rock
                _ => ()
            }
            score += win;
        }, // win
        
        _ => ()
    }
    
    // println!("Input {opp}, {game_result} => {score}");
    
    return score;
}

fn get_file(file: &str) -> String {
    return fs::read_to_string(format!("src\\day_02\\{file}.txt")).expect("File not found!");
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