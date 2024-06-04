use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input, 1000000000);
    dbg!(output);
}

pub fn process(input: &str, cycle_cnt: usize) -> usize {
    let mut rocks = parse(input);
    rocks.cycle_rocks(cycle_cnt);
    let result = rocks.calculate_load(Direction::North);
    // println!("RESULT {result}");
    return result;
}

fn parse(input: &str) -> RockPlatform {
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
    let rocks = RockPlatform::new(rows, cols);
    return rocks;
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

    fn map_row_to_cols(&mut self) {
        let mut cols: Vec<Vec<char>> = vec![];
        let mut new_puzzle: bool = true;
        for row in &self.rows {
            if new_puzzle {
                new_puzzle = false;
                cols = vec![vec![]; row.len()];
            }

            for (i, c) in row.chars().enumerate() {
                cols[i].push(c);
            }
        }
        self.cols = RockPlatform::char_vecs_to_string_vecs(cols);
    }

    fn map_col_to_rows(&mut self) {
        let mut rows: Vec<Vec<char>> = vec![];
        let mut new_puzzle: bool = true;
        for col in &self.cols {
            if new_puzzle {
                new_puzzle = false;
                rows = vec![vec![]; col.len()];
            }

            for (i, c) in col.chars().enumerate() {
                rows[i].push(c);
            }
        }
        self.rows = RockPlatform::char_vecs_to_string_vecs(rows);
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

        let mut load = 0;
        for sect in &eval_map {
            for (i, c) in sect.chars().enumerate() {
                if c == 'O' {
                    load += eval_map.len() - i;
                }
            }
        }
        return load;
    }

    fn cycle_rocks(&mut self, cycle_cnt: usize) {
        // Each cycle tilts the platform four times so that the rounded rocks roll
        // north, then west, then south, then east
        let mut history: HashMap<u32, usize> = HashMap::new();
        let mut found_cycle = false;
        let mut i = 0;
        while i < cycle_cnt {
            self.tilt_rocks(Direction::North);
            
            self.tilt_rocks(Direction::West);

            self.rows.reverse();
            self.map_row_to_cols();
            self.tilt_rocks(Direction::South);
            self.rows.reverse();
            self.map_row_to_cols();

            self.cols.reverse();
            self.map_col_to_rows();
            self.tilt_rocks(Direction::East);
            self.cols.reverse();
            self.map_col_to_rows();

            if !found_cycle {
                if let Some(cycle_start) = history.get(&grid_hash(&self.rows)) {
                    let cycle_len = i - cycle_start;
                    let repeats = (cycle_cnt - i) / cycle_len;
                    i += repeats * cycle_len;
                    found_cycle = true;
                }
                history.insert(grid_hash(&self.rows), i);
            }
            i +=1;
        }
    }

    fn tilt_rocks(&mut self, dir: Direction) {
        let eval_map: &mut Vec<String>;
        match dir {
            Direction::North => eval_map = &mut self.cols,
            Direction::South => eval_map = &mut self.cols,
            Direction::East => eval_map = &mut self.rows,
            Direction::West => eval_map = &mut self.rows,
        }
        
        for i in 0..eval_map.len() {
            let sect = &eval_map[i];
            let mut new_str: Vec<char> = vec!['.'; sect.len()];
            let mut min_spot = 0;
            for (j, c) in sect.chars().enumerate() {
                if c == '#' {
                    new_str.remove(j);
                    new_str.insert(j, '#');
                    min_spot = j+1;
                } else if c == 'O' {
                    new_str.remove(min_spot);
                    new_str.insert(min_spot, 'O');
                    min_spot += 1;
                }
            }
            let new_str = String::from_iter(new_str.to_vec());
            eval_map[i] = new_str;
        }
        
        match dir {
            Direction::North => self.map_col_to_rows(),
            Direction::South => self.map_col_to_rows(),
            Direction::East => self.map_row_to_cols(),
            Direction::West => self.map_row_to_cols(),
        }
    }

}

fn grid_hash(grid: &Vec<String>) -> u32 {
    let mut hash = 1u32;
    for line in grid.iter() {
        for c in line.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(c as u32);
        }
    }
    return hash;
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
        assert_eq!(64, process(input, 1000000000));
    }

    #[test]
    fn test_part2_3cycle() {
        let input = include_str!("part2_3cycle.txt");
        assert_eq!(64, process(input, 3));
    }
}
