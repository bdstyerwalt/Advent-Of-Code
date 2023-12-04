use std::{fs, error::Error, collections::{HashSet}};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file: String = fs::read_to_string("input.txt")?;

    println!("\n\n---------Day 3---------");
    println!("Part 1: {}", &part1(&input_file));
    //println!("Part 2: {}", &part2(&input_file));
    println!("----------------------");
    
    return Ok(())
}


fn part1(input_file: &String) -> i32 {
    let mut total_score: i32 = 0;
    for line in input_file.lines() {
        // split line on game id ':'
        let mut line_vec: Vec<&str> = line.split(":").collect();

        // After Split, pop game numbers and separate into vectors
        let mut game_nums: Vec<&str> = line_vec.pop().unwrap().split("|").collect();
        let wins_line: Vec<&str> = game_nums.pop().unwrap().trim().split_whitespace().collect();
        let have_line: Vec<&str> = game_nums.pop().unwrap().trim().split_whitespace().collect();

        // last item in line vec is the game label, find id after split
        let mut game_id: Vec<&str> = line_vec.pop().unwrap().split(" ").collect();
        let game_id: &str = game_id.pop().unwrap();
        
        // create sets of each numbers
        let have_nums = find_numbers(have_line);
        let wins_nums = find_numbers(wins_line);
        let inter = have_nums.intersection(&wins_nums);
        let winners: Vec<&i32> = inter.collect();

        let mut score: i32 = 0;
        if winners.len() > 0 { 
            let power: u32 = (winners.len() as u32) - 1;
            score = 2_i32.pow(power);
        }
        println!("Game {}: Score {} -- Winning numbers -> {:?}", game_id, score, winners);
        total_score += score;
    }
    return total_score;
}


fn find_numbers(nums: Vec<&str>) -> HashSet<i32> {
    let mut num_set: HashSet<i32> = HashSet::new();
    for num in nums {
        num_set.insert(num.parse().unwrap());
    }
    return num_set;
}
