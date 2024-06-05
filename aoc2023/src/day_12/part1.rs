pub fn process(input: &str) -> u32 {
    let (springs, groups) = parse_spring_groups(&input);

    let result: u32 = springs.iter().enumerate().zip(groups).map(|((_i, sp), gr)| {
        // print!("{i} -> ");
        let res = evaluate_row(sp, gr);
        // println!("RESULT: {res}");
        return res;
    }).sum();
    
    return result;
}

fn evaluate_row(spring_combos: &SpringGrouping, numbers: Vec<u32>) -> u32 {    
    // println!("EVAL ROW");
    let mut result = 0;
    for spring_group in &spring_combos.values {
        let broken_spring_vec = get_broken_springs(spring_group);
        // println!("broken: {:?} {:?}, numbers: {:?}", spring_group, broken_spring_vec, numbers);
        if broken_spring_vec == numbers {
            // println!("MADE IT!");
            result += 1;
        }
    }    
    return result;
}

fn parse_spring_groups(input: &str) -> (Vec<SpringGrouping>, Vec<Vec<u32>>) {
    let mut springs: Vec<SpringGrouping> = vec![];
    let mut groups: Vec<Vec<u32>> = vec![];
    for (_i, line) in input.lines().enumerate() {
        // println!("{} out of 1000", i+1);
        let mut data = line.split_whitespace();
        springs.push(SpringGrouping::new(data.nth(0).unwrap()));
        groups.push(data.nth(0).unwrap().split(",").map(|x| x.trim().parse().unwrap()).collect());
    }
    return (springs, groups);
}

fn get_broken_springs(springs: &String) -> Vec<u32> {
    let mut broken_groups: Vec<u32> = vec![];
    let mut in_broken_spring: bool = false;
    let mut broken_size = 0;
 
    for (i, c) in springs.chars().enumerate() {
        if c == '#' {
            if !in_broken_spring {
                in_broken_spring = true;
            }
            broken_size += 1;
        }
        let end_of_row = i == springs.len()-1;
        if (c != '#' && in_broken_spring) || (c == '#' && end_of_row) {
            broken_groups.push(broken_size);
            in_broken_spring = false;
            broken_size = 0;
        }
    }
    return broken_groups;
}

#[derive(Debug)]
struct SpringGrouping {
    values: Vec<String>,
}

impl SpringGrouping {
    fn new(values: &str) -> Self {
        Self {
            values: SpringGrouping::generate_combos(values),
        }
    }

    fn generate_combos(values: &str) -> Vec<String> {
        // println!("GENERATING COMBOS");
        let question_count = values.chars().filter(|c| c == &'?').count();
        let mut result: Vec<String> = vec![(&values).to_string()];
        for _i in 0..question_count {
            for _j in 0..result.len() {
                let curr = result.remove(0);
                let val1 = curr.clone().replacen("?", "#", 1);
                let val2 = curr.clone().replacen("?", ".", 1);

                // println!("Dbg ({i}, {j}): {:?} | {:?}, {:?}", curr, val1, val2);

                result.push(val1);
                result.push(val2);

            }
            // println!();
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(21, process(input));
    }
}