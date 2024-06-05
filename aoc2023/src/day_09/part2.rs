pub fn process(input: &str) -> i32 {
    let mut result: i32 = 0;

    for line in input.lines() {
        let values: Vec<i32> = line.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let mut history: Vec<Vec<i32>> = calculate_history(values);
        history = extrapolate_backwards(history);
        result += history.last().unwrap().first().unwrap();
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

fn extrapolate_backwards(mut history: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    history.reverse();
    history[0].insert(0, 0);
    for i in 1..history.len() {
        let val: i32 = history[i].first().unwrap() - history[i-1].first().unwrap();
        history[i].insert(0, val);
    }
    return history;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(2, process(input));
    }
}