use std::{collections::{HashMap, HashSet, VecDeque}, time::Instant};
use Trail::*;

fn main() {
	let now = Instant::now();
	let input = include_str!("input.txt");
	let p1 = part1(input);
	println!("Part 1: {p1} in [{}ms]", now.elapsed().as_millis());
	let p2 = part2(input);
	println!("Part 2: {p2} in [{}s]", now.elapsed().as_secs());
}

fn parse(input: &str) -> Puzzle {
	let max_row: usize = input.lines().count();
	let mut max_col: usize = 0;
	let hiking_map = input.lines().enumerate().flat_map(|(row, line)| {
		if row == 0 {
			max_col = line.len();
		}
		line.chars().enumerate().map(move |(col, ch)| {
			let pos = Pos { row, col };
		    let trail = match ch {
				'.' => Path,
				'#' => Forest,
				'^' => Slope(Direction::North),
				'v' => Slope(Direction::South),
				'>' => Slope(Direction::East),
				'<' => Slope(Direction::West),
				_ => panic!("Unrecognized charachter {ch}"),
			};
		    return (pos, trail);
		})
	}).collect::<HashMap<Pos, Trail>>();
	return Puzzle::new(hiking_map, max_row, max_col);
}

fn part1(input: &str) -> usize {
    let mut puzzle = parse(input);
    a_star_p1(&mut puzzle);
	let (max_path, visited) = puzzle.paths.iter().max_by_key(|(&key, _val)| key).unwrap();
	puzzle.print_hiking_map(visited);
	return *max_path;
}

fn part2(input: &str) -> usize {
	let mut puzzle = parse(input);
	return a_star_p2(&mut puzzle);
}

fn a_star_p1(puzzle: &mut Puzzle) {
	let mut exploration_queue: VecDeque<(Pos, HashSet<Pos>)> = VecDeque::new();
	exploration_queue.push_back((puzzle.start_pos, HashSet::new()));

	while let Some((curr_pos, mut visited)) = exploration_queue.pop_front() {
		visited.insert(curr_pos);

		if curr_pos == puzzle.target_pos {
			// println!("Found a path! lenght={}", visited.len());
			puzzle.paths.insert(visited.len()-1, visited); // minus one to remove starting position
			continue;
		}

		let curr_trail = puzzle.hiking_map.get(&curr_pos).unwrap();
		for n in get_neighbors_part1(curr_pos, curr_trail, puzzle) {
			if !visited.contains(&n) {
				exploration_queue.push_back((n, visited.clone()))
			}
		}
	}
}

fn get_neighbors_part1(pos: Pos, trail: &Trail, puzzle: &Puzzle) -> Vec<Pos> {
	let mut neighbors: Vec<Pos> = vec![];
	for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
		let mut row = pos.row.clone() as isize;
		let mut col = pos.col.clone() as isize;
		row += dir.0;
		col += dir.1;

		match trail {
			Slope(trail_dir) => {
				match (dir, trail_dir) {
					((-1, 0), Direction::North) => (),
					((1, 0), Direction::South) => (),
					((0, 1), Direction::East) => (),
					((0, -1), Direction::West) => (),
					((_, _), _) => continue,
				}
			}
			_ => (),
		}

		if row >= 0 && row <= puzzle.max_row as isize && col >= 0 && col <= puzzle.max_col as isize {
			let row = row as usize;
			let col = col as usize;
			let new_pos = Pos {row, col};
			
			let n_trail = puzzle.hiking_map.get(&new_pos).unwrap();
			match n_trail {
				Forest => continue,
				_ => neighbors.push(new_pos),
			}			
		}
	}
	return neighbors;
}

fn a_star_p2(puzzle: &mut Puzzle) -> usize {
	let mut exploration_queue: VecDeque<(Pos, HashSet<Pos>)> = VecDeque::new();
	exploration_queue.push_back((puzzle.start_pos, HashSet::new()));
	let mut max_val = 0;

	while let Some((curr_pos, mut visited)) = exploration_queue.pop_front() {
		visited.insert(curr_pos);

		if curr_pos == puzzle.target_pos {
			let path_len = visited.len()-1;
			if path_len > max_val {
				println!("Found a path! lenght={path_len}, Q[{}]", exploration_queue.len());
				max_val = path_len;
			}
			continue;
		}

		for n in get_neighbors_part2(curr_pos, puzzle) {
			if !visited.contains(&n) {
				exploration_queue.push_front((n, visited.clone()))
			}
		}
	}

	return max_val;
}

fn get_neighbors_part2(pos: Pos, puzzle: &Puzzle) -> Vec<Pos> {
	let mut neighbors: Vec<Pos> = vec![];
	for dir in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
		let mut row = pos.row.clone() as isize;
		let mut col = pos.col.clone() as isize;
		row += dir.0;
		col += dir.1;

		if row >= 0 && row <= puzzle.max_row as isize && col >= 0 && col <= puzzle.max_col as isize {
			let row = row as usize;
			let col = col as usize;
			let new_pos = Pos {row, col};
			
			let n_trail = puzzle.hiking_map.get(&new_pos).unwrap();
			match n_trail {
				Forest => continue,
				_ => neighbors.push(new_pos),
			}			
		}
	}
	return neighbors;
}

struct Puzzle {
	hiking_map: HashMap<Pos, Trail>,
	start_pos: Pos,
	target_pos: Pos,
	max_row: usize,
	max_col: usize,
	paths: HashMap<usize, HashSet<Pos>>
}

impl Puzzle {
	fn new(hiking_map: HashMap<Pos, Trail>, max_row: usize, max_col: usize) -> Self {
		Self {
			start_pos: Puzzle::find_starting_pos(hiking_map.clone()),
			target_pos: Puzzle::find_target_pos(hiking_map.clone()),
			hiking_map,
			max_row,
			max_col,
			paths: HashMap::new(),
		}
	}
	
	fn find_starting_pos(map: HashMap<Pos, Trail>) -> Pos {
		let filt = map.iter().filter_map(|(&pos, &trail)| {
		    if trail == Trail::Path {
		        return Some((pos, trail));
		    } else {
		        return None;
		    }
	    }).collect::<HashMap<Pos, Trail>>();
		return filt.into_keys().min().expect("Should Exist");
	}

	fn find_target_pos(map: HashMap<Pos, Trail>) -> Pos {
		let filt = map.iter().filter_map(|(&pos, &trail)| {
		    if trail == Trail::Path {
		        return Some((pos, trail));
		    } else {
		        return None;
		    }
	    }).collect::<HashMap<Pos, Trail>>();
		return filt.into_keys().max().expect("Should Exist");
	}

	fn print_hiking_map(&self, visited: &HashSet<Pos>) {
		for row in 0..self.max_row {
			for col in 0..self.max_col {
				let pos = Pos { row, col };
				if visited.contains(&pos) {
					print!("O")
				} else {
					let trail = self.hiking_map.get(&pos).expect("Should exist.");
					match trail {
						Path => print!("."),
						Forest => print!("#"),
						Slope(dir) => {
							match dir {
								Direction::North => print!("^"),
								Direction::South => print!("v"),
								Direction::East => print!(">"),
								Direction::West => print!("<"),
							}
						},
					}
				}
			}
			println!();
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
	row: usize,
	col: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Trail {
	Path,
	Forest,
	Slope(Direction),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        assert_eq!(94, part1(input));
        assert_eq!(154, part2(input));
    }

	#[test]
	fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(2166, part1(input)); // 249ms
        assert_eq!(6378, part2(input)); // 8hr 45m 41s
    }
}