use std::collections::{HashMap, VecDeque};
use std::ops::Range;

// TODO: Refactor part fields to hashmap


use std::fs;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_19\\input.txt").expect("File not found!");

    println!("\n--Day 19------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn parse(input: &str) -> Puzzle { 
    let lines = input.lines();
    let workflow_lines = lines.clone().filter(|line| !line.is_empty() && line.chars().nth(0).unwrap() != '{');
    let parts_lines = lines.filter(|line| !line.is_empty() && line.chars().nth(0).unwrap() == '{');

    let mut name = "";
    let workflow_map = workflow_lines.into_iter().map(|line| {
        let mut line = line.split("{");
        name = line.next().unwrap();
        let all_conds = line.next().unwrap().split(",").collect::<Vec<&str>>();
        let cond_vec: Vec<Condition> = all_conds.iter().map(|pat| {
            let cond: Condition;
            if pat.contains(":") {
                let (field, pat) = pat.split_at(1);
                let (cmp_type, pat) = pat.split_at(1);
                let mut pat = pat.split(":");
                let value: u128 = pat.next().unwrap().trim().parse().unwrap();
                let dest = pat.next().unwrap().to_string();
                cond = Condition { field: field.to_string(), cmp_type: cmp_type.to_string(), value, dest };
            } else {
                let (field, cmp_type, value, dest) = ("ELSE".to_string(), "".to_string(), 0u128, pat.replace("}", "").to_string());
                cond = Condition { field, cmp_type, value, dest };
            }
            return cond;

        }).collect::<Vec<Condition>>();
        (name.to_string(), cond_vec)
    }).collect::<HashMap<String, Vec<Condition>>>();

    let parts_vec = parts_lines.into_iter().map(|line| {
        let line = line.replace("{", "").replace("}", "");
        let parts = line.split(",").collect::<Vec<&str>>();

        let mut part = Part::new();
        for pattern in parts {
            let mut pat = pattern.split("=");
            let field = pat.next().unwrap();
            let value = pat.next().unwrap().trim().parse().expect("Should be a number");
            match field {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => (),
            }
        }
        return part;
    }).collect::<Vec<Part>>();
    
    return Puzzle { workflow_map, parts_vec};
}

fn part1(input: &str) -> u128 {
    let mut puzzle = parse(input);
    return puzzle.evaluate_parts();
}

fn part2(input: &str) -> u128 {
    let puzzle = parse(input);
    let workflows = puzzle.workflow_map;
    return find_distinct_combinations(workflows);
}

fn find_distinct_combinations(workflows: HashMap<String, Vec<Condition>>) -> u128 {
    let mut pass_rngs: VecDeque<PartRanges> = VecDeque::new();
    let mut open_rngs: VecDeque<PartRanges> = VecDeque::new();
    open_rngs.push_front(PartRanges::new());
    
    while let Some(mut curr_range) = open_rngs.pop_front() {
        let conditions = workflows.get(&curr_range.wf).unwrap(); 
        for cond in conditions {
            // println!("{:?}", cond);
            // print!("{} ", cond.field);

            curr_range.wf = cond.dest.clone();
            match cond.field.as_str() {
                "x" | "m" | "a" | "s" => {
                    let fail_range = cond.find_passing_range(&mut curr_range, cond.field.clone());
                    match cond.dest.as_str() {
                        "A" => {
                            pass_rngs.push_back(curr_range.clone());
                            curr_range = fail_range;
                        },
                        "R" => {
                            curr_range = fail_range;
                            continue;
                        },
                        _ => {
                            open_rngs.push_back(curr_range.clone());
                            curr_range = fail_range;
                        }
                    }
                },
                _ => {
                    match cond.dest.as_str() {
                        "A" => {
                            pass_rngs.push_back(curr_range.clone());
                            break;
                        },
                        "R" => break,
                        _ => open_rngs.push_back(curr_range.clone()),
                    }
                },
            };       
        }
    }

    let result = pass_rngs.iter().map(|part| {
        part.combinations()
    }).collect::<Vec<u128>>().iter().sum();
    
    return result;
}

struct Puzzle {
    workflow_map: HashMap<String, Vec<Condition>>,
    parts_vec: Vec<Part>,
}

impl Puzzle {
    fn evaluate_parts(&mut self) -> u128 {
        let starting_map = "in".to_string();
        let res = self.parts_vec.iter().filter_map(|part| {
            // println!("\n\n{:?}", part);
            let mut curr_map = &starting_map;
            while self.workflow_map.contains_key(curr_map) {
                let conditions = self.workflow_map.get(curr_map).unwrap(); 
                let mut found: bool;
                for cond in conditions {
                    // println!("{:?}", cond);
                    // print!("{} ", cond.field);
                    found = match cond.field.as_str() {
                        "x" => cond.passes_field_check(part.x),
                        "m" => cond.passes_field_check(part.m),
                        "a" => cond.passes_field_check(part.a),
                        "s" => cond.passes_field_check(part.s),
                        _ => true,
                    };
                    
                    match (found, cond.dest.as_str()) {
                        (true, "A") => return Some(part.sum()),
                        (true, "R") => {
                            // println!("REJECTED");
                            return None
                        },
                        (true, _) => {
                            curr_map = &cond.dest;
                            // println!("NEW DEST: {}", curr_map);
                            break;
                        },
                        (false, _) => {
                            curr_map = &cond.dest;
                            continue
                        },
                    }                    
                }
            }
            return None;
        }).collect::<Vec<u128>>();

        return res.iter().sum();
    }
}

#[derive(Debug, Clone)]
struct Condition {
    field: String,
    cmp_type: String,
    value: u128,
    dest: String,
}

impl Condition {
    fn passes_field_check(&self, value: u128) -> bool {
        // print!("{} {} = ", self.cmp_type, self.value);
        let res = match self.cmp_type.as_str() {
            ">" => value > self.value,
            "<" => value < self.value,
            _ => panic!(),
        };
        // println!("{res}");
        return res;
    }

    fn find_passing_range(&self, curr: &mut PartRanges, fld: String) -> PartRanges {
        let mut new_part_range = curr.clone();
        let range = curr.fields.get(&fld).unwrap().clone();
        
        match self.cmp_type.as_str() {
            ">" => { 
                curr.fields.insert(fld.clone(), self.value+1..range.end);
                new_part_range.fields.insert(fld, range.start..self.value);
            },
            "<" => { 
                curr.fields.insert(fld.clone(), range.start..self.value-1);
                new_part_range.fields.insert(fld, self.value..range.end);
            }
            _ => panic!(),
        };
        return new_part_range;
    }
}

#[derive(Debug, Clone)]
struct PartRanges {
    fields: HashMap<String, Range<u128>>,
    wf: String,
}

impl PartRanges {
    fn new() -> Self {
        let start = 1..4000;
        let mut map = HashMap::new();
        map.insert("x".to_string(), start.clone());
        map.insert("m".to_string(), start.clone());
        map.insert("a".to_string(), start.clone());
        map.insert("s".to_string(), start);
        Self {
            fields: map,
            wf: "in".to_string()
        }
    }

    fn combinations(&self) -> u128 {
        let x = self.fields.get("x").unwrap();
        let m = self.fields.get("m").unwrap();
        let a = self.fields.get("a").unwrap();
        let s = self.fields.get("s").unwrap();
        
        let x = x.end - x.start + 1;
        let m = m.end - m.start + 1;
        let a = a.end - a.start + 1;
        let s = s.end - s.start + 1;
               
        return x * m * a * s;
    }
}

#[derive(Debug)]
struct Part {
    x: u128,
    m: u128,
    a: u128,
    s: u128,
}

impl Part {
    fn new() -> Self {
        Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        }
    }

    fn sum(&self) -> u128 {
        let result = self.x + self.m + self.a + self.s;
        // dbg!(result);
        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(19114, p1);
    }

    #[test]
    fn test_sample_p2() {
        let input = include_str!("sample.txt");
        let p2 = part2(input);
        dbg!(p2);
        assert_eq!(167409079868000, p2);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let p1 = part1(input);
        let p2 = part2(input);
        dbg!(p1, p2);
        assert_eq!(395382, p1);
        assert_eq!(103557657654583, p2)
    }
}