use std::{fs, error::Error, collections::HashMap, str::Lines, ops::Range, vec};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file: String = fs::read_to_string("input.txt")?;

    println!("\n\n---------Day 5---------");
    let p1_res: u64 = part1(&input_file);
    println!("Part 1: {}", p1_res);
    println!("-----------------------");
    let p2_res: u64 = part2(&input_file);
    println!("Part 2: {}", p2_res);
    println!("-----------------------");
    
    return Ok(())
}

fn part1(input_lines: &String) -> u64 {
    let mut lines = input_lines.lines();

    // Find Seeds
    let mut seeds = lines.next().unwrap();
    seeds = seeds.split(": ").nth(1).unwrap();
    let seeds: Vec<u64> = seeds.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    
    let mut seed_map: HashMap<u64, u64> = HashMap::new();
    for seed in &seeds {
        seed_map.insert(*seed, *seed);
    }
    
    return process(lines, seed_map);
}

fn part2(input_lines: &String) -> u64 {
    let mut lines = input_lines.lines();

    // Find Seeds from first line
    let seeds = lines.next().unwrap().split(": ").nth(1).unwrap();
    let seeds: Vec<u64> = seeds.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    
    // send the rest of the input to map builder
    let seed_to_loc_maps: Vec<Vec<(Range<u64>, Range<u64>)>> = build_part2_maps(lines);
    
    let mut min_val = u64::MAX;
    let mut locations: Vec<u64> = vec![];
    for seed_range in seeds.chunks(2) {
        println!("{}->{}", seed_range[0], seed_range[0]+seed_range[1]);
        for mut seed in seed_range[0]..seed_range[0]+seed_range[1] {
            //print!("\nSeed {} | ", seed);
            for map in &seed_to_loc_maps {
                for (source, destin) in map {
                    let old_seed = seed;
                    if source.contains(&seed) {
                        seed = seed + destin.start-source.start;
                        //print!("{}->{}, ", old_seed, seed);                        
                        break
                    }
                    //print!("{}->{}, ", old_seed, seed);                        
                }
            }
            locations.push(seed)
            //min_val = min_val.min(seed);
        }
        //println!()
    }
    println!("{:?}", locations);
    return *locations.iter().min().unwrap();
}


fn build_map(nums: &mut Vec<u64>) -> HashMap<(u64, u64), (u64, u64)> {
    //print!("--Building!");
    if nums.len() < 2 { panic!() }
    let range: u64 = nums[2];
    let source: u64 = nums[1];
    let destn: u64 = nums[0];

    let mut map: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
    map.insert((source, source+range), (destn, destn+range));
    //println!("{:?}", map);
    return map;
}

fn compare_maps(source_map: HashMap<u64, u64>, dest_map: HashMap<(u64, u64), (u64, u64)>) -> HashMap<u64, u64> {
    //println!("--Comparing!");
    //println!("src: {:?}", source_map);
    //println!("dst {:?}", dest_map);
    let mut map: HashMap<u64, u64> = HashMap::new();
    for (seed, value) in &source_map {
        let mut found: bool = false;
        for ((source_min, source_max), (dest_min, _dest_max)) in &dest_map {
            if (source_min..source_max).contains(&value) {
                //println!("sMin {}, dMin {}", source_min, dest_min);
                let diff = value-source_min ;
                //println!("Found Seed[{}] {}-->{} ({})", seed, value, value+diff, diff);
                map.insert(*seed, dest_min+diff);
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

fn process(lines: Lines, seed_map: HashMap<u64, u64>) -> u64 {
    let mut seed_map: HashMap<u64, u64> = seed_map;
    let mut building: bool = false;
    let mut tmp_map: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
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
            let mut map_input: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
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

fn build_part2_maps(lines: Lines) -> Vec<Vec<(Range<u64>, Range<u64>)>> {
    let mut building: bool = false;
    let mut tmp_map: Vec<(Range<u64>, Range<u64>)> = vec![];
    let mut all_maps: Vec<Vec<(Range<u64>, Range<u64>)>> = vec![];

    for line in lines {
        if line.contains("map:") {
            println!("{}", line);
            building = true;
            continue;
        }

        if building && line.is_empty() {
            all_maps.push(tmp_map.clone());
            tmp_map.clear();
            building = false;
        }
        
        if building {
            // Each line within a map contains three numbers: 
            // [Destination range start], [Source range start], [Range length]
            let map_input: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            let source_range: Range<u64> = map_input[1]..map_input[1]+map_input[2];
            //println!("Source Range: {}->{}", map_input[1], map_input[1]+map_input[2]);
            let destin_range: Range<u64> = map_input[0]..map_input[0]+map_input[2];
            //println!("Destin Range: {}->{}", map_input[0], map_input[0]+map_input[2]);
            tmp_map.push((source_range, destin_range));
        }
    }
    
    return all_maps;
}


#[cfg(test)]
mod tests {
    use std::{fs};
    use crate::part1;
    use crate::part2;

    #[test]
    fn part1_sample() {
        let input_file: String = fs::read_to_string("sample.txt").expect("Couldn't read file");
        assert_eq!(35, part1(&input_file));
    }

    #[test]
    fn part2_sample() {
        let input_file: String = fs::read_to_string("sample.txt").expect("Couldn't read file");
        assert_eq!(46, part2(&input_file));
    }

    #[test]
    fn part1_input() {
        let input_file: String = fs::read_to_string("input.txt").expect("Couldn't read file");
        assert_eq!(486613012, part1(&input_file));
    }

    #[test]
    fn part2_input() {
        let input_file: String = fs::read_to_string("input.txt").expect("Couldn't read file");
        assert_eq!(56931769, part2(&input_file));
    }
}