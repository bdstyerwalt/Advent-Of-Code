pub fn process(input: &str) -> u32 {
    let mut puzzles: Vec<Puzzle> = vec![];
    let mut rows: Vec<String> = vec![];
    let mut cols: Vec<Vec<char>> = vec![];
    let mut new_puzzle: bool = true;
    for line in input.lines() {
        if !new_puzzle && line.is_empty() {
            new_puzzle = true;
            puzzles.push(Puzzle::new(rows.clone(), cols.clone()));
            rows.clear();
            cols.clear();
            continue;
        }

        if new_puzzle {
            new_puzzle = false;
            cols = vec![vec![]; line.len()];
        }

        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
        rows.push(line.to_string());
    }
    puzzles.push(Puzzle::new(rows.clone(), cols.clone()));
    
    let mut result: u32 = 0;
    for (_i, puzzle) in puzzles.iter().enumerate() {
        // print!("Solving puzzle {}! -> ", i+1);
        let mut intermediate = 0;
        intermediate += check_horizontal(&puzzle);
        intermediate += check_vertical(&puzzle);
        // println!("{intermediate}");
        result += intermediate;
        // println!("Result: {:?}\n", result);
    }
    return result;
}

fn check_vertical(puzzle: &Puzzle) -> u32 {
    let mut result: u32 = 0;
    let n = puzzle.n_cols;
    let cols = &puzzle.columns;
    for i in 2..n+1 {
        if i % 2 == 0 {
            result += check_from_middle(&cols[0..i].to_vec(), i);
        } 
        if result > 0 {
            // dbg!(&cols[0..i].to_vec());
            return result;
        }
    }

    if result == 0 {
        let mut temp_cols = cols.clone();
        let mut temp_res = 0;
        temp_cols.reverse();
        for i in 2..n+1 {
            if i % 2 == 0 {
                temp_res = check_from_middle(&temp_cols[0..i].to_vec(), i);
            } 
            if temp_res > 0 {
                result += n as u32 - temp_res;
                // dbg!(&temp_cols[0..i].to_vec());
                return result;
            }
        }
    }
    return result;
}

fn check_horizontal(puzzle: &Puzzle) -> u32 {
    let mut result: u32 = 0;
    let n = puzzle.n_rows;
    let rows = &puzzle.rows;

    for i in 2..n+1 {
        if i % 2 == 0 {
            result += 100*check_from_middle(&rows[0..i].to_vec(), i);
        } 
        if result > 0 {
            // dbg!(&rows[0..i].to_vec());
            return result;
        }
    }

    let mut temp_rows = rows.clone();
    let mut temp_res = 0;
    temp_rows.reverse();
    for i in 2..n+1 {
        if i % 2 == 0 {
            temp_res = check_from_middle(&temp_rows[0..i].to_vec(), i);
        } 
        if temp_res > 0 {
            result += 100*(n as u32 - temp_res);
            // dbg!(&temp_rows[0..i].to_vec());
            return result;
        }
    }
    return result;
}

fn check_from_middle(rows: &Vec<String>, stop: usize) -> u32 {
    let mid = stop / 2;
    let mut top = mid-1;
    let mut bot = mid;

    let mut is_mirror = true;
    let mut diffs = 0;
    for _i in 0..mid {
        let left = &rows[top];
        let right = &rows[bot];
        if left != right {
            for (c1, c2) in left.chars().zip(right.chars()) {
                if c1 != c2 {
                    diffs += 1;
                }
            }
            if diffs > 1 {
                is_mirror = false;
                break;
            }
        }

        if top == 0 {
            break;
        }
        top -= 1;
        bot += 1
    }
    
    let mut result: u32 = 0;
    if is_mirror && diffs > 0 {
        // dbg!(rows);
        // print!("diffs {diffs} -> ");
        result = mid as u32;
    }
    return result;
}

struct Puzzle {
    rows: Vec<String>,
    n_rows: usize,
    columns: Vec<String>,
    n_cols: usize,
}

impl Puzzle {
    fn new(rows: Vec<String>, columns: Vec<Vec<char>>) -> Self {
        Self {
            n_rows: rows.len(),
            rows: rows,
            n_cols: columns.len(),
            columns: Self::char_vecs_to_string_vecs(columns),
        }
    }

    fn char_vecs_to_string_vecs(cols: Vec<Vec<char>>) -> Vec<String> {
        let mut new_cols: Vec<String> = vec![];
        for col_vec in cols {
            new_cols.push(String::from_iter(col_vec));
        }
        return new_cols;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_sample() {
        let input = fs::read_to_string("src\\day_13\\sample.txt").expect("Couldn't read file");
        assert_eq!(400, process(&input));
    }

    #[test]
    fn test_puzzle1() {
        let input = fs::read_to_string("src\\day_13\\puzzle1.txt").expect("Couldn't read file");
        assert_eq!(5, process(&input));
    }

    #[test]
    fn test_puzzle2() {
        let input = fs::read_to_string("src\\day_13\\puzzle2.txt").expect("Couldn't read file");
        assert_eq!(1, process(&input));
    }

    #[test]
    fn test_puzzle3() {
        let input = fs::read_to_string("src\\day_13\\puzzle3.txt").expect("Couldn't read file");
        assert_eq!(1100, process(&input));
    }
}
