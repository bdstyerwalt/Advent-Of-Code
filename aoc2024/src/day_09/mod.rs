use std::collections::HashMap;

pub fn run() {
    let day_idx = file!().find("day_").expect("Couldn't find `day_` in file path") + 4;
    let day = file!().get(day_idx..day_idx+2).unwrap();
    let input_file = include_str!("input.txt");

    println!("\n--Day {day}------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

#[derive(Debug)]
struct Puzzle {
    disk: Vec<String>,
    nums: HashMap<u64, (u64, usize)>, // <id, (idx, length)>
    gaps: HashMap<u64, usize>, // <idx, length>
    checksum: u64,
}

impl Puzzle {
    fn new(disk: Vec<String>, nums: HashMap<u64, (u64, usize)>, gaps: HashMap<u64, usize>) -> Self {
        Puzzle {
            disk: disk,
            nums: nums,
            gaps: gaps,
            checksum: 0,
        }
    }

    fn update_disk(&mut self) {
        self.checksum = 0;
        let mut new_disk: Vec<String> = vec![String::from("."); self.disk.len()];
        for (id, (idx, cnt)) in &self.nums {
            // println!("id: {id}, idx: {idx}, cnt: {cnt}");
            for i in 0..*cnt {
                let k = (*idx as usize) + i;
                new_disk[k] = id.to_string();
                self.checksum += id * (k as u64);
            }
        }
        
        // for v in &new_disk {
        //     print!("{v}");
        // }
        // println!("");
        self.disk = new_disk;
    }
}

fn parse(input: &str) -> Puzzle {
    let mut disk: Vec<String> = vec![];
    let mut nums: HashMap<u64, (u64, usize)> = HashMap::new();
    let mut gaps: HashMap<u64, usize> = HashMap::new();
    let mut id: u32 = 0;
    
    let mut real_idx = 0;
    for (idx, ch) in input.chars().enumerate() {
        let size = ch.to_string().parse().unwrap();
        for _i in 0..size {
            if idx % 2 == 0 {
                disk.push(id.to_string());
                nums.insert(id as u64, (real_idx as u64, size));
            } else {
                disk.push('.'.to_string());
                gaps.insert(real_idx as u64, size);
            }
        }
        real_idx += size;
        if idx % 2 == 0 {
            id += 1;
        }
    }
    
    let puzzle = Puzzle::new(disk, nums, gaps);
    
    // println!("NUMS: {:?}", puzzle.nums);
    // println!("GAPS: {:?}", puzzle.gaps);
    return puzzle;
}

fn part1(input_file: &str) -> u64 {
    let puzzle = parse(input_file);
    let mut numbers: Vec<u64> = vec![];
    let segments = puzzle.disk.clone();
    let mut pt1 = 0;
    let mut pt2 = puzzle.disk.len()-1;

    while pt1 <= pt2 {
        let left = segments.get(pt1).unwrap();
        let right = segments.get(pt2).unwrap();
        if left != &String::from('.') {
            let val = left.parse::<u64>().expect("Should be number");
            numbers.push(val);
            pt1 += 1;
        } else if right != &String::from('.') {
            let val = right.parse::<u64>().expect("Should be number");
            numbers.push(val);
            pt1 += 1;
            pt2 -= 1;
        } else {
            pt2 -= 1;
        }
    } 
    
    let checksum: u64 = numbers.iter().enumerate().fold(0, |mut acc, (i, num)| {
        let val = i as u64 * num;
        acc += val;
        // println!("{i}*{num}={val} -> {acc}");
        return acc;
    });
    
    return checksum;
}

fn part2(input_file: &str) -> u64 {
    let mut puzzle = parse(input_file);
    let max_num = *puzzle.nums.keys().clone().max().unwrap();

    for id in (0..=max_num).rev() {
        if id % 100 == 0 {
            println!("Remaining: Nums={id} - Gaps={}", puzzle.gaps.len());
        }
        
        let mut gap_keys: Vec<u64> = puzzle.gaps.clone().into_keys().collect();
        gap_keys.sort_unstable();
        let mut found = false;
        for k in gap_keys {
            let (idx, num_size) = puzzle.nums.remove(&id).unwrap();
            let gap_size = puzzle.gaps.remove(&k).unwrap();
            
            if k > idx {
                puzzle.nums.insert(id, (idx, num_size));
                break;
            } else if num_size == gap_size {
                puzzle.nums.insert(id, (k, num_size));
                found = true;
                break;
            } else if num_size < gap_size {
                puzzle.nums.insert(id, (k, num_size));
                puzzle.gaps.insert(k + num_size as u64, gap_size - num_size);
                found = true;
                break;
            } else {
                puzzle.gaps.insert(k, gap_size);
                puzzle.nums.insert(id, (idx, num_size));
            }
        }
        if found {
            puzzle.update_disk();
        }
        
    }
    puzzle.update_disk();
    // 4029946121348 is too low
    // 6321896265143
    // 18446744073709551615 max u64
    return puzzle.checksum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(1928, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(2858, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(6288599492129, part1(input));
        assert_eq!(6321896265143, part2(input));
    }
}