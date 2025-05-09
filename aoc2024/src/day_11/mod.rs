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
    rocks: Vec<u64>,
    //              num  blinks num rocks
    memory: HashMap<(u64, usize), u64>,
}

impl Puzzle {
    fn new(rocks: Vec<u64>) -> Self {
        Puzzle {
            rocks,
            memory: HashMap::new(),
        }
    }
    
    fn get_next_rock_vec(rock: u64) -> Vec<u64> {
        let mut rock_vec = vec![];
        if rock == 0_u64 {
            rock_vec.push(1_u64);
        } else if rock.ilog10() % 2 != 0 {
            let mid = (rock.ilog10() + 1) / 2;
            let div = 10_u64.pow(mid as u32);
            rock_vec.push(rock / div);
            rock_vec.push(rock % div);
        } else {
            rock_vec.push(rock * 2024);
        }
        return rock_vec;
    }
    
    #[allow(unused_variables)]
    fn apply_ruleset(&mut self, cnt: usize) {
        for blink in 1..=cnt {
            // println!("Blink {blink}");
            let mut new_rocks: Vec<u64> = vec![];
            for (i, rk) in self.rocks.iter().enumerate() {
                new_rocks.extend(Puzzle::get_next_rock_vec(*rk));
            }
            self.rocks = new_rocks;
        }
    }
    
    #[allow(unused_variables)]
    fn apply_ruleset_with_mem(&mut self, cnt: usize) -> u64 {
        let mut rock_count: u64 = 0;
        for rock in self.rocks.clone().iter() {
            rock_count += self.compute_rocks_recursive(*rock, cnt);
        }
        return rock_count;
    }
    
    fn compute_rocks_recursive(&mut self, rock: u64, steps: usize) -> u64 {
        if self.memory.contains_key(&(rock, steps)) {
            return self.memory[&(rock, steps)];
        }
        
        if steps == 0 {
            return 1;
        }
        
        let mut res: u64 = 0;
        let next_rocks = Puzzle::get_next_rock_vec(rock);
        for v in next_rocks {
            res += self.compute_rocks_recursive(v, steps-1);
        }
        self.memory.insert((rock, steps), res);
        
        return res;
    }
}

fn parse(input: &str) -> Puzzle {
    let rocks = input.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect();
    return Puzzle::new(rocks);
}


fn part1(input_file: &str) -> usize {
    let mut puzzle = parse(input_file);
    let blinks = 25;
    puzzle.apply_ruleset(blinks);
    return puzzle.rocks.len();
}

fn part2(input_file: &str) -> u64 {
    let mut puzzle = parse(input_file);
    let blinks = 75;
    let result = puzzle.apply_ruleset_with_mem(blinks);
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(125681, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(149161030616311, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(193899, part1(input));
        assert_eq!(229682160383225, part2(input));
    }
}