use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input, 64);
    let p2 = part2(input, 26501365);
    println!("Part 1: {} | Part 2: {}", p1, p2);
}

fn parse(input: &str) -> Puzzle { 
    let mut starting_pos = Pos::new(0, 0);
    let mut max_col = 0;
    let max_row = input.lines().count() - 1;
    let garden_map = input.lines().enumerate().flat_map(|(row, line)| {
        let chs = line.clone().chars();
        if row == 0 { max_col = chs.count() - 1 }
        match line.chars().into_iter().position(|c| c == 'S') {
            Some(col) => starting_pos = Pos::new(row, col),
            None => (),
        }

        line.chars().enumerate().map(move |(col, c)| {
            let pos = Pos::new(row, col);
            let is_valid = if c != '#' { true } else { false };
            return (pos, is_valid)
        })
    }).collect::<HashMap<Pos, bool>>();
    return Puzzle { starting_pos, garden_map, max_row, max_col};
}

fn part1(input: &str, step_total: usize) -> usize {
    let puzzle = parse(input);
    let mut curr_locations: Vec<Pos> = vec![puzzle.starting_pos];
    let mut next_locations: Vec<Pos> = vec![];
    for _i in 0..step_total {
        while let Some(pos) = curr_locations.pop() {
            // check each neighbor
            let mut north = pos.clone();
            if north.row != 0 {
                north.row -= 1;
                if *puzzle.garden_map.get(&north).unwrap() && !next_locations.contains(&north) {
                    next_locations.push(north)
                }
            }

            let mut south = pos.clone();
            if south.row != puzzle.max_row {
                south.row += 1;
                if *puzzle.garden_map.get(&south).unwrap() && !next_locations.contains(&south) {
                    next_locations.push(south)
                }
            }

            let mut east = pos.clone();
            if east.col != 0 {
                east.col -= 1;
                if *puzzle.garden_map.get(&east).unwrap() && !next_locations.contains(&east) {
                    next_locations.push(east)
                }
            }

            let mut west = pos.clone();
            if west.col != puzzle.max_col {
                west.col += 1;
                if *puzzle.garden_map.get(&west).unwrap() && !next_locations.contains(&west) {
                    next_locations.push(west)
                }
            }            
        }
        // println!("Step {i}: {:?}\n\n", &next_locations);
        curr_locations = next_locations.clone();
        next_locations.clear();
    }
    return curr_locations.len();
}

fn part2(input: &str, step_total: usize) -> i64 {
    let puzzle = parse(input);

    // TODO: Change to hashsets
    let mut curr_locations: HashSet<Loc> = HashSet::new();
    curr_locations.insert(Loc { pos: puzzle.starting_pos, midx: MapIndex {row: 0, col: 0} });
    let mut next_locations: HashSet<Loc> = HashSet::new();

    let mut values: Vec<i64> = vec![];
    for i in 1..=step_total {
        for loc in &curr_locations {
            let (pos, map_index) = (loc.pos, loc.midx);
            //          North,   South,  East,    West
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut new_map_idx = map_index.clone();
                let mut new_pos = pos.clone();
                let mut new_row = new_pos.row as i32 + dir.0;
                let mut new_col = new_pos.col as i32 + dir.1;

                if new_row < 0 {
                    new_row = puzzle.max_row as i32;
                    new_map_idx.row += dir.0 as isize;
                } else if new_row > puzzle.max_row as i32 {
                    new_row = 0;
                    new_map_idx.row += dir.0 as isize;
                }
                new_pos.row = new_row as usize;

                if new_col < 0 {
                    new_col = puzzle.max_col as i32;
                    new_map_idx.col += dir.1 as isize;
                } else if new_col > puzzle.max_col as i32 {
                    new_col = 0;
                    new_map_idx.col += dir.1 as isize;
                }
                new_pos.col = new_col as usize;

                let new_loc = Loc { pos: new_pos.clone(), midx: new_map_idx };
                if *puzzle.garden_map.get(&new_pos).unwrap() {
                    next_locations.insert(new_loc);
                }
            }
        }
        if i % 10 == 0 {
            // println!("Step {i}: {}", &next_locations.len());
        }
        std::mem::swap(&mut curr_locations, &mut next_locations);
        next_locations.clear();
        if i == 64 {
            println!("{}", curr_locations.len());
        }
        if [65, 196, 327].contains(&i) {
            values.push(curr_locations.len() as i64);
            println!("{values:?}");
        }
        if values.len() == 3 { break; }
    }
    let a = (values[2] - (2*values[1]) + values[0]) / 2;
    let b = values[1] - values[0] - a;
    let c = values[0];
    println!("{}", puzzle.max_col);
    let n = ((step_total - 65) / (puzzle.max_col + 1)) as i64;
    println!("{n}");
    return (a*n*n) + (b*n) + c;
    // 1952973769694283 "too high"
}



struct Puzzle {
    starting_pos: Pos,
    garden_map: HashMap<Pos, bool>,
    max_row: usize,
    max_col: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct MapIndex {
    row: isize,
    col: isize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    pos: Pos,
    midx: MapIndex,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(16, part1(input, 6));
        // assert_eq!(, part2(input));
    }

    #[test]
    fn test_p2_6steps() {
        let input = include_str!("sample.txt");
        assert_eq!(16, part2(input, 6))
    }

    #[test]
    fn test_p2_10steps() {
        let input = include_str!("sample.txt");
        assert_eq!(50, part2(input, 10))
    }

    #[test]
    fn test_p2_50steps() {
        let input = include_str!("sample.txt");
        assert_eq!(1594, part2(input, 50))
    }

    #[test]
    fn test_p2_100steps() {
        let input = include_str!("sample.txt");
        assert_eq!(6536, part2(input, 100))
    }

    #[test]
    fn test_p2_500steps() {
        let input = include_str!("sample.txt");
        assert_eq!(167004, part2(input, 500))
    }

    #[test]
    fn test_p2_1000steps() {
        let input = include_str!("sample.txt");
        assert_eq!(668697, part2(input, 1000))
    }

    #[test]
    fn test_p2_5000steps() {
        let input = include_str!("sample.txt");
        assert_eq!(16733044, part2(input, 5000))
    }
}