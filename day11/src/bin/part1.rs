use std::{str::Lines, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let universe = expand_universe(input);
    let galaxy_map = find_galaxies(universe);
    return calc_shortest_path(galaxy_map);
}

fn expand_universe(universe: &str) -> Vec<String> {
    let mut expanded_rows: Vec<String> = vec![];
    let mut expanded_column_check = vec![];
    for (row, line) in universe.lines().enumerate() {
        if row == 0 {
            expanded_column_check = vec![true; line.len()+1];
        }
        if line.chars().all(|c| c=='.') {
            expanded_rows.push(line.to_string());
            expanded_rows.push(line.to_string());
            continue;
        }
        
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                expanded_column_check[col] = false;
            }
        }
        expanded_rows.push(line.to_string());
    }

    let mut expanded_universe: Vec<String> = vec![];
    for line in expanded_rows.iter() {
        let mut new_line: String = "".to_string();
        for (col, c) in line.chars().enumerate() {
            // println!("Line: {} | Col: {col} | Exp: {}", line.len(), expanded_column_check.len());
            if expanded_column_check[col] { 
                new_line.push_str(".");
            }
            new_line.push_str(c.to_string().as_str());
        }
        // println!("{:?}", new_line);
        expanded_universe.push(new_line);
    }
    return expanded_universe;
}

fn find_galaxies(expanded_universe: Vec<String>) -> HashMap<i32, (i32, i32)> {
    let mut galaxy_map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut galaxy_count = 1;
    for (row, line) in expanded_universe.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_map.insert(galaxy_count, (col as i32, row as i32));
                galaxy_count += 1;
            }
        }
    }
    return galaxy_map;
}

fn calc_shortest_path(galaxy_map: HashMap<i32, (i32, i32)>) -> i32 {
    let mut shortest_paths: Vec<i32> = vec![];
    for (g1, l1) in &galaxy_map {
        for (g2, l2) in &galaxy_map {
            if g2 <= g1 {
                continue;
            }
            let dist = i32::abs(l1.0 - l2.0) + i32::abs(l1.1 - l2.1);
            shortest_paths.push(dist)
        }
    }
    return shortest_paths.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(374, process(input));
    }

    #[test]
    fn test_part1_soln() {
        let input = include_str!("input.txt");
        assert_eq!(9609130, process(input));
    }
}