use std::{fs, error::Error, collections::HashMap, str::Lines};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file: String = fs::read_to_string("sample.txt")?;

    println!("\n\n---------Day 5---------");
    let p1_res: i128 = part1(&input_file);
    println!("Part 1: {}", p1_res);
    let p2_res: i128 = part2(&input_file);
    println!("Part 2: {}", p2_res);
    println!("-----------------------");
    
    return Ok(())
}

fn part1(input_lines: &String) -> i128 {
    let mut lines = input_lines.lines();

    // Find Seeds
    let mut seeds = lines.next().unwrap();
    seeds = seeds.split(": ").nth(1).unwrap();
    let seeds: Vec<i128> = seeds.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    
    let mut seed_map: HashMap<i128, i128> = HashMap::new();
    for seed in &seeds {
        seed_map.insert(*seed, *seed);
    }
    
    return process(lines, seed_map);
}

fn part2(input_lines: &String) -> i128 {
    let mut lines = input_lines.lines();

    // Find Seeds
    let mut seeds = lines.next().unwrap();
    seeds = seeds.split(": ").nth(1).unwrap();
    let seeds: Vec<i128> = seeds.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    println!("seeds {:?}", seeds);
    let starters: Vec<i128> = seeds.clone().into_iter()
                        .step_by(2)
                        .collect();
    let ranges: Vec<i128> = seeds.into_iter()
                        .skip(1)
                        .step_by(2)
                        .collect();

    println!("starts {:?}", starters);
    println!("ranges {:?}", ranges);
    

    let mut seed_map: HashMap<i128, i128> = HashMap::new();
    for i in 0..starters.len() {
        let start = starters[i];
        println!("{} -> {:?}", start, ranges[i]);
        for r in 0..ranges[i] {
            seed_map.insert(start+r, start+r);
        }
    }
    
    return process(lines, seed_map);
}


fn build_map(nums: &mut Vec<i128>) -> HashMap<(i128, i128), (i128, i128)> {
    //print!("--Building!");
    if nums.len() < 2 { panic!() }
    let range: i128 = nums[2];
    let source: i128 = nums[1];
    let destn: i128 = nums[0];

    let mut map: HashMap<(i128, i128), (i128, i128)> = HashMap::new();
    map.insert((source, source+range), (destn, destn+range));
    //println!("{:?}", map);
    return map;
}

fn compare_maps(source_map: HashMap<i128, i128>, dest_map: HashMap<(i128, i128), (i128, i128)>) -> HashMap<i128, i128> {
    //println!("--Comparing!");
    //println!("src: {:?}", source_map);
    //println!("dst {:?}", dest_map);
    let mut map: HashMap<i128, i128> = HashMap::new();
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

fn process(lines: Lines, seed_map: HashMap<i128, i128>) -> i128 {
    let mut seed_map: HashMap<i128, i128> = seed_map;
    let mut building: bool = false;
    let mut tmp_map: HashMap<(i128, i128), (i128, i128)> = HashMap::new();
    for line in lines {
        if line.contains("map:") {
            println!("{}", line);
            building = true;
            continue;
        }

        if building && line.is_empty() {
            building = false;
        }
        
        if building {
            let mut map_input: Vec<i128> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
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
