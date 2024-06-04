use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> i32 {
    let mut lines = input.lines();
    let instructions: Vec<String> = lines.next().unwrap().chars().map(|c| c.to_string()).collect();
    // println!("{:?}", instructions);
    
    lines.next(); // skip empty line
    let mut mappings: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        
        let vals: Vec<&str> = line.split(" = ").collect();
        let key: &str = vals[0];
        let left_right: Vec<&str> = vals[1].split(", ").collect();
        let left = left_right[0].replace("(", "").clone();
        let right = left_right[1].replace(")", "").clone();
        
        mappings.insert(key.to_string(), (left, right));
    }

    let mut step_count: i32 = 0;
    let mut curr_step: String = "AAA".to_string();
    let mut instructions = instructions.iter().cycle();
    // print!("Starting at: {curr_step}");
    while curr_step != "ZZZ" {
        let step = instructions.next().unwrap();
        let (_k, v) = mappings.get_key_value(&curr_step).unwrap();
        if step == "R" {
            curr_step = v.1.to_string();
        } else /* L */{
            curr_step = v.0.to_string();
        }
        // print!(" -> {curr_step}");
        step_count += 1;
    }
    // println!("\n");
    return step_count;
}


#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_sample1() {
        let input = include_str!("p1_sample1.txt");
        assert_eq!(2, process(&input));
    }

    #[test]
    fn test_sample2() {
        let input = include_str!("p1_sample2.txt");
        assert_eq!(6, process(&input));
    }
}