use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input, 1000000);
    dbg!(output);
}

pub fn process(universe: &str, scale: i64) -> i64 {
    let expanse = expand_universe(universe);
    let galaxy_map = find_galaxies(universe);
    return calc_shortest_path(galaxy_map, expanse, scale);
}

fn expand_universe(universe: &str) -> (Vec<i64>, Vec<i64>) {
    // println!("Expanding Universe Map...");
    // Check rows and columns for expansion.
    let mut expanded_col_check = vec![];
    let mut expanded_rows: Vec<i64> = vec![];
    for (row, line) in universe.lines().enumerate() {
        if row == 0 {
            expanded_col_check = vec![true; line.len()]
        }
        if line.chars().all(|c| c=='.') {
            expanded_rows.push(row as i64);
        }       
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                expanded_col_check[col] = false;
            }
        }
    }

    let mut expanded_cols: Vec<i64> = vec![];
    for i in 0..expanded_col_check.len() {
        if expanded_col_check[i] {
            expanded_cols.push(i as i64);
        }
    }

    // // expand columns
    // let mut expanded_cols: Vec<String> = vec![];
    // for (row, line) in universe.lines().enumerate() {
    //     let mut new_line: String = "".to_string();
    //     for (col, c) in line.chars().enumerate() {
    //         if expanded_column_check[col] { 
    //             print!("expanding cols... ");
    //             for _i in 0..scale-1 {
    //                 new_line.push_str(".");
    //             }
    //             println!("Done!");
    //         }
    //         new_line.push_str(c.to_string().as_str());
    //     }
    //     expanded_cols.push(new_line);
    // }


    // // expand rows to complete universe expansion
    // let mut expanded_universe: Vec<String> = vec![];
    // for (row, line) in expanded_cols.iter().enumerate() {
    //     if expanded_row_check[row] {
    //         print!("expanding rows... ");
    //         for _i in 0..scale-1 {
    //             expanded_universe.push(line.to_string());
    //         }
    //         println!("Done!");
    //     }
    //     expanded_universe.push(line.to_string());
    // }

    return (expanded_rows, expanded_cols);
}

fn find_galaxies(universe: &str) -> HashMap<i64, (i64, i64)> {
    // println!("Finding galaxy coordinates...");
    let mut galaxy_map: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut galaxy_count = 1;
    for (row, line) in universe.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                // println!("Found Galaxy {galaxy_count} at ({},{})", col, row);
                galaxy_map.insert(galaxy_count, (col as i64, row as i64));
                galaxy_count += 1;
            }
        }
    }
    return galaxy_map;
}

fn calc_shortest_path(galaxy_map: HashMap<i64, (i64, i64)>, expanse: (Vec<i64>, Vec<i64>), scale: i64) -> i64 {
    // print!("Calculation the shortest paths...");
    let exp_rows = expanse.0;
    let exp_cols = expanse.1;
    let scale = scale - 1;

    // print!("{:?}", exp_rows);
    // print!("{:?}", exp_cols);

    let mut shortest_paths: Vec<i64> = vec![];
    for (g1, loc1) in &galaxy_map {
        // print!("{g1}/{}", galaxy_map.len()+1);
        for (g2, loc2) in &galaxy_map {
            if g2 <= g1 {
                continue;
            }
            let mut row_delta = 0;
            for r in exp_rows.iter() {
                let r = *r;
                if (loc1.1 < r && loc2.1 > r) || (loc1.1 > r && loc2.1 < r) {
                    // println!("Adding row {r} delta {scale} between {g1}({}) and {g2}({})", loc1.0, loc2.0);
                    row_delta += scale;
                }
            }
            let mut col_delta = 0;
            for c in exp_cols.iter() {
                let c = *c;
                if (loc1.0 < c && loc2.0 > c) || (loc1.0 > c && loc2.0 < c) {
                    // println!("Adding col {c} delta {scale} between {g1}({}) and {g2}({})", loc1.1, loc2.1);
                    col_delta += scale;
                }
            }

            let dist = i64::abs(loc1.0 - loc2.0) + row_delta + i64::abs(loc1.1 - loc2.1) + col_delta;
            // println!("Dist {dist} between {g1}({},{}) and {g2}({},{})", loc1.1, loc1.0, loc2.1, loc2.0);
            shortest_paths.push(dist)
        }
    }
    // print!("count of pairs {}", shortest_paths.len());
    return shortest_paths.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_2x() {
        let input = include_str!("sample.txt");
        assert_eq!(374, process(input, 2));
    }

    #[test]
    fn test_sample_10x() {
        let input = include_str!("sample.txt");
        assert_eq!(1030, process(input, 10));
    }

    #[test]
    fn test_sample_100x() {
        let input = include_str!("sample.txt");
        assert_eq!(8410, process(input, 100));
    }

    // #[test]
    // fn test_part2_soln() {
    //     let input = include_str!("input.txt");
    //     assert_eq!(9609130, process(input));
    // }
}