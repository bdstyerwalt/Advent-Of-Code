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
    after_rules: HashMap<u32, Vec<u32>>,
    before_rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
    middle_pages: Vec<u32>,
    incorrect_updates: Vec<Vec<u32>>,
    corrected_updates: Vec<Vec<u32>>,
}

impl Puzzle {
    fn evaluate_page_correctness(&mut self) {
        for (_idx, update) in self.updates.iter().enumerate() {
            let mut update_pass = true;
            for (i, page) in update.iter().enumerate() {
                let after_pages = &update[i+1..];
                let before_pages = &update[..i];
                // Validate that before pages don't violate after Rules
                let (after_pass, _) = Puzzle::check_rules(&self.after_rules, before_pages, page);
                // Validate that after pages don't violate before Rules
                let (before_pass, _) = Puzzle::check_rules(&self.before_rules, after_pages, page);
                
                update_pass = update_pass && after_pass && before_pass;
            }
            
            if update_pass {
                let mid = update.len()/2;
                self.middle_pages.push(update[mid]);
            } else {
                self.incorrect_updates.push(update.clone());
            }
        }
    }

    fn check_rules(rule_set: &HashMap<u32, Vec<u32>>, pages: &[u32], curr_page: &u32) -> (bool, u32) {
        let rules = rule_set.get(curr_page);
        match rules {
            Some(rules) => {
                for rule in rules {
                    // if pages contain the rule, the rule is volated 
                    if pages.contains(rule) {
                        return (false, *rule);
                    }
                }
            },
            None => return (true, 0),
        }
        return (true, 0);
    }

    fn swap_pages(mut page_vec: Vec<u32>, p1: u32, p2: u32) -> Vec<u32> {
        let idx1 = page_vec.iter().position(|&p| p==p1).unwrap();
        let idx2 = page_vec.iter().position(|&p| p==p2).unwrap();
        page_vec.swap(idx1, idx2);
        return page_vec;
    }

    fn rectify_updates(&mut self) {
        for (_idx, update) in self.incorrect_updates.iter().enumerate() {
            let mut i = 0;
            let mut temp_update = update.clone();
            
            // println!("\n________________________________________");
            // println!("Update: {:?}", update);
            while let Some(page) = temp_update.get(i) {
                let after_pages = &temp_update[i+1..];
                let before_pages = &temp_update[..i];
                let (after_rules_pass, vi) = Puzzle::check_rules(&self.after_rules, &before_pages, page);
                if !after_rules_pass {
                    // println!("--After Rule Violated: {vi} should be after {page} -> swapping...");
                    temp_update = Puzzle::swap_pages(temp_update.clone(), vi, *page);
                    continue;
                }
                let (before_rules_pass, vi) = Puzzle::check_rules(&self.before_rules, after_pages, page);
                if !before_rules_pass {
                    // println!("--Before Rule Violated: {vi} should be before {page} -> swapping...");
                    temp_update = Puzzle::swap_pages(temp_update.clone(), vi, *page);
                }

                if after_rules_pass && before_rules_pass {
                    // println!("Fixed Update: {:?}", temp_update);
                    i += 1;                
                }
            }
            self.corrected_updates.push(temp_update.clone());
        }
    }
}

fn parse(input: &str) -> Puzzle {
    let mut after: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];
    
    for line in input.lines() {
        if line.contains("|") {
            let nums: Vec<u32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            let (left, right) = (nums[0], nums[1]);
            after.entry(left).and_modify(|x_vec| x_vec.push(right)).or_insert(vec![right]);
            before.entry(right).and_modify(|x_vec| x_vec.push(left)).or_insert(vec![left]);
        } else if line.contains(",") {
            let nums: Vec<u32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            updates.push(nums);
        }
    }
    
    let puzzle = Puzzle {
        after_rules: after,
        before_rules: before,
        updates: updates,
        middle_pages: vec![],
        incorrect_updates: vec![],
        corrected_updates: vec![],
    };
    //println!("{:?}", puzzle);
    return puzzle;
}

fn part1(input_file: &str) -> u32 {
    let mut puzzle = parse(input_file);
    puzzle.evaluate_page_correctness();
    // println!("{:?}", puzzle.middle_pages);
    return puzzle.middle_pages.iter().sum();
}

fn part2(input_file: &str) -> u32 {
    let mut puzzle = parse(input_file);
    puzzle.evaluate_page_correctness();
    puzzle.rectify_updates();
    let res = puzzle.corrected_updates.iter().fold(0, |acc, v| acc + v[v.len()/2]);
    return res;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        assert_eq!(143, part1(include_str!("sample.txt")));
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!(123, part2(include_str!("sample.txt")));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(4135, part1(input));
        assert_eq!(5285, part2(input));
    }
}