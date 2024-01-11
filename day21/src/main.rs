use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input, 64);
    let p2 = part2(input);
    println!("Part 1: {} | Part 2: {}", p1, p2);
}

fn parse(input: &str) -> Puzzle { 
    let mut starting_pos = Pos { row: 0, col: 0 };
    let mut max_col = 0;
    let max_row = input.lines().count();
    let garden_map = input.lines().enumerate().flat_map(|(row, line)| {
        let chs = line.clone().chars();
        if row == 0 { max_col = chs.count() }
        match line.chars().into_iter().position(|c| c == 'S') {
            Some(col) => starting_pos = Pos { row, col },
            None => (),
        }

        line.chars().enumerate().map(move |(col, c)| {
            let pos = Pos { row, col };
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
    for i in 0..step_total {
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

fn part2(input: &str) -> u32 {
    return 0;
}


struct Puzzle {
    starting_pos: Pos,
    garden_map: HashMap<Pos, bool>,
    max_row: usize,
    max_col: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos {
    row: usize,
    col: usize,
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
}