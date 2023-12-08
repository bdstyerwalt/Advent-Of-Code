use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let mut lines = input.lines();
    let instructions: Vec<String> = lines.next().unwrap().chars().map(|c| c.to_string()).collect();
    
    lines.next(); // skip empty line

    let mut starting_locs: Vec<String> = vec![];
    let mut mappings: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let vals: Vec<&str> = line.split(" = ").collect();
        let key: &str = vals[0];
        if key.ends_with("A") {
            println!("New Starting Location: {}", key);
            starting_locs.push(key.to_string())
        }

        let left_right: Vec<&str> = vals[1].split(", ").collect();
        let left = left_right[0].replace("(", "").clone();
        let right = left_right[1].replace(")", "").clone();
        
        mappings.insert(key.to_string(), (left, right));
    }
    //println!("{:?}", mappings);

    let mut step_count: i32 = 0;
    let mut curr_step: Vec<String> = starting_locs;

    let mut instructions = instructions.iter().cycle();

    //print!("Starting at: ");
    while curr_step.iter().any(|a| !a.ends_with("Z")) {
        let step = instructions.next().unwrap();
        //print!(" -> {:?}", curr_step);
        for i in 0..curr_step.len() {
            let (_k, v) = mappings.get_key_value(&curr_step[i]).unwrap();
            if step == "R" {
                curr_step[i] = v.1.to_string();
            } else /* L */{
                curr_step[i] = v.0.to_string();
            }
        }
        step_count += 1;
    }
    //println!("\n");
    return step_count;
}


#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_sample1() {
        let input = include_str!("p2_sample.txt");
        assert_eq!(6, process(&input));
    }
}