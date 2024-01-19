use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
	let input = include_str!("input.txt");

	let start = Instant::now();
	let p1 = part1(input);
	println!("Part 1 {p1} [{}]", start.elapsed().as_millis());

	let start = Instant::now();
	let p2 = part2(input);
	println!("Part 2 {p2} [{}]", start.elapsed().as_millis());
}

fn parse(input: &str) -> Puzzle {
	let mut node_map: HashMap<String, HashSet<String>> = HashMap::new();	
	for line in input.lines() {
		let mut line = line.split(":");
		let left = line.next().unwrap().trim().to_string();
		let nodes: Vec<String> = line.next().unwrap().split_whitespace().map(|n| n.trim().to_string()).collect();

		for node in nodes {
			let mut left_set = HashSet::new();
			left_set.insert(left.clone());
			let mut node_set = HashSet::new();
			node_set.insert(node.clone());

			node_map.entry(left.clone()).and_modify(|links| { links.insert(left.clone()); }).or_insert(left_set); // forward connection
			node_map.entry(node.clone()).and_modify(|links| { links.insert(left.clone()); }).or_insert(node_set); // reverse connection
		}
	}
	
	for (id, nodes) in &node_map {
		println!("{id} has {} links -> {nodes:?}", nodes.len())
	}

	return Puzzle::new(node_map);
}

fn part1(input: &str) -> usize {
    let puzzle = parse(input);
	return 0;
}

fn part2(input: &str) -> usize {
	return 0;
}

struct Puzzle {
	node_map: HashMap<String, HashSet<String>>,
}

impl Puzzle {
	fn new(node_map: HashMap<String, HashSet<String>>) -> Self {
		Self {
			node_map,
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(54, part1(input));
        // assert_eq!(47f64, part2(input));
    }

	#[test]
	fn test_input() {
        let input = include_str!("input.txt");
        // assert_eq!(17906, part1(input, 200000000000000f64, 400000000000000f64)); // 1.908s
        // assert_eq!(6378, part2(input)); // 8hr 45m 41s
    }
}