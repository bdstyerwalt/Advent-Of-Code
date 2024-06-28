use std::fs;
use std::collections::BTreeSet;

pub fn run() {
    println!("\n--Day {}------", get_day());
    println!("Part 1: {}", &part1(&get_file("input")));
    println!("Part 2: {}", &part2(&get_file("input")));
    println!("--------------");
}

fn part1(input: &str) -> usize {
    let result = input.lines().fold(0, |acc, rucksack| {
        let num_items = rucksack.len();
        let (items1, items2) = rucksack.split_at(num_items/2);
        
        let mut set1 = BTreeSet::from_iter(items1.split(""));
        set1.remove("");
        let mut set2 = BTreeSet::from_iter(items2.split(""));
        set2.remove("");
        
        let wrong_item = set1.intersection(&set2).nth(0).unwrap();
        
        acc + get_priority(wrong_item)
    });
    
    return result;
}

fn part2(input: &str) -> usize {
    let mut priority_total: usize = 0;
    let mut ruck_set: BTreeSet<&str> = BTreeSet::new();

    let mut cnt = 1;
    for line in input.lines() {
        let mut temp_set = BTreeSet::from_iter(line.split(""));
        temp_set.remove("");
        
        match cnt {
            1 => {
                ruck_set = temp_set.clone();
                ruck_set.remove("");
            },

            2 => {
                let int_vec: Vec<_> = ruck_set.intersection(&temp_set).cloned().collect();
                ruck_set = BTreeSet::from_iter(int_vec);
            },

            3 => {
                let int_vec: Vec<_> = ruck_set.intersection(&temp_set).cloned().collect();
                ruck_set = BTreeSet::from_iter(int_vec);
                priority_total += get_priority(ruck_set.iter().nth(0).unwrap());
                cnt = 0;
            },
            _ => ()
        }       
        cnt += 1;
        
    }
    
    return priority_total;
}

fn get_priority(item: &str) -> usize {
    let mut letters: Vec<char> = ('a'..='z').collect();
    let mut upper: Vec<char> = ('A'..='Z').collect();
    letters.append(&mut upper);
    
    let pri = letters.iter().position(|x| x.to_string() == item.to_string()).expect("Should have found a letter");
    
    return pri + 1;
}

fn get_file(file: &str) -> String {
    return fs::read_to_string(format!("src\\day_{}\\{file}.txt", get_day())).expect("File not found!");
}

fn get_day() -> String {
    return "03".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1_sample() {
        let result = part1(&get_file("sample"));
        assert_eq!(result, 157)
    }
    
    #[test]
    fn p1_input() {
        let result = part1(&get_file("input"));
        assert_eq!(result, 7811)
    }
    
    #[test]
    fn p2_sample() {
        let result = part2(&get_file("sample"));
        assert_eq!(result, 70)
    }
    
    #[test]
    fn p2_input() {
        let result = part2(&get_file("input"));
        assert_eq!(result, 2639)
    }
}