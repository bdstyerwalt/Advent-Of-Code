use std::{fs, collections::{HashSet, HashMap}};

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_04\\input.txt").expect("File not found!");

    let (p1_res, game_cards) = part1(&input_file);
    
    println!("\n--Day 04------");
    println!("Part 1: {}", p1_res);
    println!("Part 2: {}", &part2(game_cards));
    println!("--------------");
}

fn part1(input_file: &String) -> (i32, HashMap<i32, i32>) {
    let mut game_map: HashMap<i32, i32> = HashMap::new();
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
        
        // find the intersection
        let inter = have_nums.intersection(&wins_nums);
        let winners: Vec<&i32> = inter.collect();

        // get the games score
        //print!("Game {}: Winning numbers -> {:?}", game_id, winners);
        let mut score: i32 = 0;
        if winners.len() > 0 { 
            let power: u32 = (winners.len() as u32) - 1;
            score = 2_i32.pow(power);
        }
        //println!(" -- Score {}", score);
        total_score += score;
        game_map.insert(game_id.parse().unwrap(), winners.len() as i32);
    }
    //println!("{:?}", game_map);
    return (total_score, game_map);
}

fn part2(game_map: HashMap<i32, i32>) -> i32 {
    let mut game_vec: Vec<i32> = vec![];
    for (id, _score) in &game_map {
        game_vec.push(*id);
    }
    
    let mut total_cards: i32 = 0;
    while game_vec.len() > 0 {
        total_cards += 1;
        let id = game_vec.pop().unwrap();
        let (_id, score) = game_map.get_key_value(&id).unwrap();
        //println!("Game {} had {} winners", id, score);
        if *score > 0 {
            //print!(" -> Adding games: ");
            for cnt in 1..=*score {
                let id = id + cnt;
                //print!("{id}");
                game_vec.push(id);
            }
            //println!("")
        }
    }

    return total_cards;
}

fn find_numbers(nums: Vec<&str>) -> HashSet<i32> {
    let mut num_set: HashSet<i32> = HashSet::new();
    for num in nums {
        num_set.insert(num.parse().unwrap());
    }
    return num_set;
}