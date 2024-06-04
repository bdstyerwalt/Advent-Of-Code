fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> usize {
    let mut rows: Vec<String> = vec![];
    let mut cols: Vec<Vec<char>> = vec![];
    let mut new_puzzle: bool = true;
    for line in input.lines() {
        if new_puzzle {
            new_puzzle = false;
            cols = vec![vec![]; line.len()];
        }

        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
        rows.push(line.to_string());
    }
    let rocks = RockPlatform::new(rows, cols.clone());

    let result = rocks.calculate_load(Direction::North);
    // println!("RESULT {result}");
    return result;
}

struct RockPlatform {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl RockPlatform {
    fn new(rows: Vec<String>, cols: Vec<Vec<char>>) -> Self {
        Self {
            rows: rows,
            cols: RockPlatform::char_vecs_to_string_vecs(cols),
        }
    }

    fn char_vecs_to_string_vecs(cols: Vec<Vec<char>>) -> Vec<String> {
        let mut new_cols: Vec<String> = vec![];
        for col_vec in cols {
            new_cols.push(String::from_iter(col_vec));
        }
        return new_cols;
    }

    fn calculate_load(&self, dir: Direction) -> usize {
        let mut eval_map: Vec<String>;
        match dir {
            Direction::North => {
                eval_map = self.cols.clone();
            },
            Direction::South => {
                eval_map = self.cols.clone();
                eval_map.reverse();
            },
            Direction::East => {
                eval_map = self.rows.clone();
            },
            Direction::West => {
                eval_map = self.rows.clone();
                eval_map.reverse();
            },
        }
        // dbg!(&eval_map);

        let mut load = 0;
        for sect in &eval_map {
            // dbg!(&sect);
            let mut max_load = eval_map.len();
            let spaces = sect.chars();
            let rock_count = spaces.filter(|c| c == &'O').count();
            if !sect.contains("#") {
                // print!("No Square Rocks: {load} -> ");
                for i in 0..rock_count {  
                    load += max_load - i;
                }
                // println!("{load}");
                continue;
            }
            
            for (i, c) in sect.chars().enumerate() {
                if c == '#' {
                    // print!("FOUND #: {max_load} -> ");
                    max_load = eval_map.len() - i - 1;
                    // println!("{max_load}");
                } else if c == 'O' {
                    load += max_load;
                    // println!("FOUND O: Adding {max_load} -> {load}");
                    max_load -= 1;
                }

            }
        }
        return load;
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(136, process(input));
    }
}
