use std::{fs, error::Error, collections::HashMap};
use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>> {
    let input_file: String = fs::read_to_string("input.txt")?;

    println!("\n\n----------------------");
    println!("Day 2 - Part 1: {}", &part1(&input_file));
    println!("Day 2 - Part 2: {}", &part2(&input_file));
    println!("----------------------");
    
    return Ok(())
}

fn part1(input_file: &String) -> i32 {
    // hash map of games with game# as key and rgb tuples as values
    let game_map: HashMap<i32, HashMap<String, i32>> = build_games_table(input_file);
    let game_soln: HashMap<String, i32> = HashMap::from([(String::from("red"), 12), (String::from("green"), 13), (String::from("blue"), 14)]);
    let mut acc = 0;
    for (id, scores) in game_map {
        let mut pass: bool = true;
        for (color, value) in scores {
            let Some((_key, max_val)) = game_soln.get_key_value(&color) else { continue; };
            if value > *max_val { pass = false }
        }
        if pass { 
            acc += id ;
            //println!("Game {} passed!", id)
        }
    }
    return acc;
}

fn part2(input_file: &String) -> i32 {
    let game_map: HashMap<i32, HashMap<String, i32>> = build_games_table(input_file);

    let mut acc: i32 = 0;
    for (_id, scores) in game_map {
        let mut inner_acc: i32 = 1;
        for (_color, value) in scores {
            inner_acc *= value;
        }
        acc += inner_acc;
    }
    return acc;
}

fn build_games_table(input_file: &String) -> HashMap<i32, HashMap<String, i32>> {
    // hash map of games with game# as key and rgb tuples as values
    let mut game_map: HashMap<i32, HashMap<String, i32>> = HashMap::new();
    let game_id = Regex::new(r"(Game )(?<gameID>\d{1,})(:)(?<values>.*)").unwrap();

    // loop over lines, get game id, check color combos
    for line in input_file.lines() { 
        // get game id and values
        if !game_id.is_match(line) { panic!() }
        let caps = game_id.captures(line).unwrap();
        let id: i32 = caps["gameID"].parse().unwrap();
        let values: String = String::from(&caps["values"]).replace(";", ",");
        
        // check color combos
        let mut game_values: HashMap<String, i32> = HashMap::from([(String::from("red"), 0), (String::from("green"), 0), (String::from("blue"), 0)]);
        for byte in values.split(", ") {
            let mut data = byte.trim().split_whitespace();
            let mut val: i32 = data.next().unwrap().parse().unwrap();
            let color: &str = data.next().unwrap();
            
            //println!("Color: {}, Value: {}", &color, val);

            let prev:i32 = match game_values.get_key_value(color) {
                Some((_key, prev)) => *prev,
                None => 0,
            };
            if val < prev {
                val = prev;
            }
            
            game_values.insert(String::from(color), val);
        }
        //println!("Game {} {:?}", id, &game_values);
        game_map.insert(id, game_values);    
    }
    return game_map;
}