use std::{fs, error::Error, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file: String = fs::read_to_string("input.txt")?;

    let p1_res: i64 = part1(&input_file);

    println!("\n\n---------Day 5---------");
    println!("Part 1: {}", p1_res);
    //println!("Part 2: {}", part2(game_cards));
    println!("-----------------------");
    
    return Ok(())
}

fn part1(input_lines: &String) -> i64 {
    let mut lines = input_lines.lines();

    // Find Seeds
    let mut seeds = lines.next().unwrap();
    seeds = seeds.split(": ").nth(1).unwrap();
    let mut seeds: Vec<i64> = seeds.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    
    let mut seed_map: HashMap<i64, i64> = HashMap::new();
    for seed in &seeds {
        seed_map.insert(*seed, *seed);
    }
    
    let mut building: bool = false;
    let mut tmp_map: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    for line in lines {
        if line.contains("map:") {
            println!("\n------\n{}", line);
            building = true;
            continue;
        }

        if building && line.is_empty() {
            building = false;
        }
        
        if building {
            let mut map_input: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            let m2 = build_map(&mut map_input);
            tmp_map.extend(m2)
        }
        
        if !building && !tmp_map.is_empty() {
            seed_map = compare_maps(seed_map, tmp_map.clone());
            tmp_map.clear();
        }
    }
    seed_map = compare_maps(seed_map, tmp_map.clone());
    return *seed_map.values().min().unwrap();
}

fn build_map(nums: &mut Vec<i64>) -> HashMap<(i64, i64), (i64, i64)> {
    //print!("--Building!");
    if nums.len() < 2 { panic!() }
    let range: i64 = nums[2];
    let source: i64 = nums[1];
    let destn: i64 = nums[0];

    let mut map: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    map.insert((source, source+range), (destn, destn+range));
    //println!("{:?}", map);
    return map;
}

fn compare_maps(source_map: HashMap<i64, i64>, dest_map: HashMap<(i64, i64), (i64, i64)>) -> HashMap<i64, i64> {
    //println!("--Comparing!");
    //println!("src: {:?}", source_map);
    //println!("dst {:?}", dest_map);
    let mut map: HashMap<i64, i64> = HashMap::new();
    for (seed, value) in &source_map {
        let mut found: bool = false;
        for ((source_min, source_max), (dest_min, _dest_max)) in &dest_map {
            if value >= &source_min && value < &source_max {
                //println!("sMin {}, dMin {}", source_min, dest_min);
                let diff = dest_min-source_min ;
                //println!("Found Seed[{}] {}-->{} ({})", seed, value, value+diff, diff);
                map.insert(*seed, value+diff);
                found = true;
                break;
            } 
        }
        if  !found {
            map.insert(*seed, *value);
        }
    }
    //println!("{:?}", map);
    return map;
}

