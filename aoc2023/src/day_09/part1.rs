fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> i32 {
    let mut result: i32 = 0;

    for line in input.lines() {
        let values: Vec<i32> = line.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let mut history: Vec<Vec<i32>> = calculate_history(values);
        history = extrapolate(history);
        result += history.last().unwrap().last().unwrap();
    }
    return result;
}

fn calculate_history(values: Vec<i32>) -> Vec<Vec<i32>> {
    let mut still_calc = true;
    let mut total_history:Vec<Vec<i32>> = vec![values];
    let mut curr_idx = 0;
    while still_calc {
        let curr_row: &Vec<i32> = &total_history[curr_idx];
        let mut new_row: Vec<i32> = vec![];
        for i in 0..curr_row.len()-1 {
            new_row.push(curr_row[i+1] - curr_row[i]);
        }
        if new_row.iter().all(|x| x == &0) { still_calc = false;}
        total_history.push(new_row);
        curr_idx += 1;
    }
    return total_history;
}

fn extrapolate(mut history: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    history.reverse();
    history[0].push(0);
    for i in 1..history.len() {
        let val: i32 = history[i].last().unwrap() + history[i-1].last().unwrap();
        history[i].push(val);
    }
    return history;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(114, process(input));
    }
}