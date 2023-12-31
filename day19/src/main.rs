use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input);
    let p2 = part2(input);
    println!("Part 1: {} | Part 2: {}", p1, p2);
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
                let value: u32 = pat.next().unwrap().trim().parse().unwrap();
                let dest = pat.next().unwrap().to_string();
                cond = Condition { field: field.to_string(), cmp_type: cmp_type.to_string(), value, dest };
            } else {
                let (field, cmp_type, value, dest) = ("ELSE".to_string(), "".to_string(), 0u32, pat.replace("}", "").to_string());
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

fn part1(input: &str) -> u32 {
    let mut puzzle = parse(input);
    return puzzle.evaluate_parts();
}

fn part2(_input: &str) -> u32 {
    return 0;
}

struct Puzzle {
    workflow_map: HashMap<String, Vec<Condition>>,
    parts_vec: Vec<Part>,
}

impl Puzzle {
    fn evaluate_parts(&mut self) -> u32 {
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
        }).collect::<Vec<u32>>();

        return res.iter().sum();
    }
}

#[derive(Debug)]
struct Condition {
    field: String,
    cmp_type: String,
    value: u32,
    dest: String,
}

impl Condition {
    fn passes_field_check(&self, value: u32) -> bool {
        // print!("{} {} = ", self.cmp_type, self.value);
        let res = match self.cmp_type.as_str() {
            ">" => value > self.value,
            "<" => value < self.value,
            _ => panic!(),
        };
        // println!("{res}");
        return res;
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
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

    fn sum(&self) -> u32 {
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
        let result = part1(input);
        dbg!(result);
        assert_eq!(19114, result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        dbg!(result);
        assert_eq!(395382, result);
    }
}