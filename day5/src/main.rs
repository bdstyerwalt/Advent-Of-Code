use std::{fs, error::Error, collections::HashMap, vec};
use std::cmp::min;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file: String = fs::read_to_string("sample.txt")?;

    let p1_res: i64 = part1(&input_file);

    println!("\n\n---------Day 5---------");
    println!("Part 1: {}", p1_res);
    //println!("Part 2: {}", part2(game_cards));
    println!("----------------------");
    
    return Ok(())
}

fn part1(input_lines: &String) -> i64 {
    let mut lines = input_lines.lines();

    // Find Seeds
    let mut seeds = lines.next().unwrap();
    seeds = seeds.split(": ").nth(1).unwrap();
    let mut seeds: Vec<i64> = seeds.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    
    let mut tmp_map: HashMap<i64, i64> = HashMap::new();
    for seed in &seeds {
        tmp_map.insert(*seed, *seed);
    }
    let mut map_vec: Vec<HashMap<i64, i64>> = vec![tmp_map.clone()];
    tmp_map.clear();
    
    let mut building: bool = false;
    for line in lines {
        if line.contains("map:") {
            println!("\n------\n{}", line);
            building = true;
            continue;
        }
        
        if building && line.is_empty() {
            println!("--Complete!");
            building = false;
            map_vec.push(tmp_map.clone());
            tmp_map.clear();
        }
        
        if building {
            let mut map_input: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            let m2 = build_map(&mut map_input);
            tmp_map.extend(m2)
        }

        if !building && map_vec.len() > 1 {
            let mut pass_vec: Vec<HashMap<i64, i64>> = map_vec.clone();
            map_vec.clear();
            map_vec.push(compare_maps(pass_vec.pop().unwrap(), pass_vec.pop().unwrap()));
        }
    }

    let final_map: HashMap<i64, i64> = map_vec.pop().unwrap();
    let mut result: i64 = *final_map.get(&seeds.pop().unwrap()).unwrap();
    for seed in seeds.into_iter() {
        let tmp = final_map.get(&seed).unwrap();
        result = min(result, *tmp);
    }
    return result;
}


fn build_map(nums: &mut Vec<i64>) -> HashMap<i64, i64> {
    println!("--Building!");
    if nums.len() < 2 { panic!() }
    let range: i64 = nums.pop().unwrap();
    let source: i64 = nums.pop().unwrap();
    let destn: i64 = nums.pop().unwrap();

    let map: HashMap<i64, i64> = (0..range).map(|x| (source+x, destn+x)).collect();
    return map;
}

fn compare_maps(dest_map: HashMap<i64, i64>, source_map: HashMap<i64, i64>) -> HashMap<i64, i64> {
    println!("--Comparing!");
    let mut map: HashMap<i64, i64> = HashMap::new();
    for (seed, value) in &source_map {
        if dest_map.contains_key(&value) {
            let (key, val) = dest_map.get_key_value(&value).unwrap();
            map.insert(*seed, *val);
        } else {
            map.insert(*seed, *value);
        }
    }
    return map;
}

