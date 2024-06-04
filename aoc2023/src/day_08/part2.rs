use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> i64 {
    let mut lines = input.lines();
    let instructions: Vec<String> = lines.next().unwrap().chars().map(|c| c.to_string()).collect();
    
    lines.next(); // skip empty line

    let mut starting_locs: Vec<String> = vec![];
    let mut mappings: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let vals: Vec<&str> = line.split(" = ").collect();
        let key: &str = vals[0];
        if key.ends_with("A") {
            // println!("New Starting Location: {}", key);
            starting_locs.push(key.to_string())
        }

        let left_right: Vec<&str> = vals[1].split(", ").collect();
        let left = left_right[0].replace("(", "").clone();
        let right = left_right[1].replace(")", "").clone();
        
        mappings.insert(key.to_string(), (left, right));
    }
    //println!("{:?}", mappings);

    let mut curr_step: Vec<String> = starting_locs;
    let mut solve_values: Vec<i64> = vec![];
    for i in 0..curr_step.len() {
        let loop_inst: Vec<String> = instructions.clone();
        let mut loop_inst = loop_inst.iter().cycle();
        let mut step_count: i64 = 0;
        //print!("Starting at: {:?}", curr_step[i]);
        while !curr_step[i].ends_with("Z") {
            let step = loop_inst.next().unwrap();
            let (_k, v) = mappings.get_key_value(&curr_step[i]).unwrap();
            if step == "R" {
                curr_step[i] = v.1.to_string();
            } else /* L */ {
                curr_step[i] = v.0.to_string();
            }
            step_count += 1;
            //print!(" -> ({}|{}) {:?}", step_count, step, curr_step[i]);
            if curr_step[i].ends_with("Z") { break }
        }
        solve_values.push(step_count);
        //println!("\n");
    }
    // println!("{:?}", solve_values);
    return lcm(&solve_values);
}

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let input = include_str!("p2_sample.txt");
        assert_eq!(6, process(&input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(12357789728873, process(&input));
    }

    #[test]
    fn test_sample_lcm() {
        let input = include_str!("p2_test_case.txt");
        let output = process(&input);
        dbg!(output);
        assert_eq!(30, output);
    }
    
    #[test]
    fn it_works() {
        assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2, 4, 6, 8, 10]), 120);
        assert_eq!(lcm(&[3, 6, 9, 12, 15]), 180);
        assert_eq!(lcm(&[10]), 10);
        assert_eq!(lcm(&[21, 110]), 2310);
    }
}