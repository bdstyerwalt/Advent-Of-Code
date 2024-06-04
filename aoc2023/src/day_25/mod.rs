use std::collections::VecDeque;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

use std::fs;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_25\\input.txt").expect("File not found!");

    println!("\n--Day 25------");
    let start = Instant::now();
	let p1 = part1(&input_file);
	println!("Part 1 {p1} [{:.3?}]", start.elapsed());
    println!("--------------");
}

fn parse<'a>(input: &'a str) -> Graph<'a> {
	let mut graph = Graph::new();
	for line in input.lines() {
		let mut line = line.split(":");
		let left: &'a str = line.next().unwrap().trim();
		let nodes: Vec<&'a str> = line.next().unwrap().split_whitespace().map(|n| n.trim()).collect();

		// println!("Node {left} with {nodes:?}");
		for node in nodes {
			graph.add_edge(left, node, 1);
		}
	}

	return graph;
}

fn part1(input: &str) -> usize {
    let graph = parse(input);
	let (cut, partition) = stoer_wagner(&graph);
	// dbg!(cut);
	return partition.len() * (graph.verticies.len() - partition.len());
}

fn stoer_wagner<'a>(graph: &'a Graph) -> (usize, Vec<&'a str>) {
	let mut contracted_graph = graph.clone();
	
	let mut contractions = Vec::new();
	let mut best_phase = 0;
	let mut best_cut_weight = usize::MAX;

	// while there is more than one vertex	
	for phase in 0..graph.verticies.len()-1 {
		// println!("Phase {} -> best{}[{}]", phase+1, best_phase, best_cut_weight);
		let (s, t, cut_weight) = minimum_cut_phase(&contracted_graph);

		if cut_weight < best_cut_weight {
			best_phase = phase;
			best_cut_weight = cut_weight;
		}
		contractions.push((s, t));

		// update edges previously connected to t
		for (node, weight) in contracted_graph.get_edges(t) {
			contracted_graph.update_edge(s, node, weight)
		}
		contracted_graph.remove_node(t);
	}

	let mut partition_adj = HashMap::new();
	// take the contractions through the best phase
	for (s, t) in contractions.iter().take(best_phase) {
		partition_adj.entry(*s).or_insert_with(Vec::new).push(*t);
		partition_adj.entry(*t).or_insert_with(Vec::new).push(*s);
	}

	let mut visited = HashSet::new();
	let mut frontier = VecDeque::new();
	frontier.push_front(contractions[best_phase].1);
	while let Some(node) = frontier.pop_front() {
		if visited.contains(node) {
			continue;
		}
		visited.insert(node);

		if let Some(edges) = partition_adj.get(node) {
			edges.iter().for_each(|edge| {
				frontier.push_back(edge);
			})
		}
	}

	let partition = visited.into_iter().collect();
	// dbg!(&partition);
	return (best_cut_weight, partition);
}

fn minimum_cut_phase<'a>(graph: &Graph<'a>) -> (&'a str, &'a str, usize) {
	// setup priority queue with all edges
	let mut queue = BinaryHeap::new();
	graph.verticies.iter().for_each(|v| {
		queue.push(State {v, w: 0})
	});

	let mut cut_weight = 0;
	let mut s = "";
	let mut t = "";

	while let Some(state) = queue.pop() {
		let (node, weight) = (state.v, state.w);
		s = t;
		t = node;
		cut_weight = weight;

		for (edge, weight) in graph.get_edges(node) {
			let mut new_queue = BinaryHeap::new();
			while let Some(mut state) = queue.pop() {
				if state.v == edge {
					state.w += weight;
				}
				new_queue.push(state);
			}
			std::mem::swap(&mut queue, &mut new_queue);
		}
	}
	return (s, t, cut_weight)
}

#[derive(Debug, PartialEq, Eq)]
struct State<'a> {
	v: &'a str,
	w: usize,
}

impl<'a> Ord for State<'a> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.w.cmp(&other.w)
	}
}

impl<'a> PartialOrd for State<'a> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, Clone)]
struct Graph<'a> {
	verticies: HashSet<&'a str>,
	edges: HashMap<&'a str, HashMap<&'a str, usize>>,
}

impl<'a> Graph<'a> {
	fn new() -> Self {
		Self {
			edges: HashMap::new(),
			verticies: HashSet::new(),
		}
	}

	fn get_edges(&self, node: &'a str) -> Vec<(&'a str, usize)>{
		return self.edges.get(node).unwrap().iter().map(|(node, weight)| (*node, *weight)).collect();
	}

	fn add_edge(&mut self, n1: &'a str, n2: &'a str, w: usize) {
		// add both edges to vertex list
		self.verticies.insert(n1);
		self.verticies.insert(n2);

		// add both directions of edge
		self.edges.entry(n1).or_default().insert(n2, w);
		self.edges.entry(n2).or_default().insert(n1, w);
	}

	fn update_edge(&mut self, n1: &'a str, n2: &'a str, w: usize) {
		*self.edges.entry(n1).or_default().entry(n2).or_insert(0) += w;
		*self.edges.entry(n2).or_default().entry(n1).or_insert(0) += w;
	}

	fn remove_node(&mut self, node: &'a str) {
		// remove node as key from vertex and edge map
		self.verticies.remove(node);
		self.edges.remove(node);

		// Remove every refrence to node as a value from edge map
		self.edges.iter_mut().for_each(|(_, edges)| {
			edges.remove(node);
		});
	}
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_stoer_wagner() {
		let mut graph = Graph::new();
		
		let one = "1";
		let two = "2";
		let three = "3";
		let four = "4";
		let five = "5";
		let six = "6";
		let seven = "7";
		let eight = "8";
		
		graph.add_edge(one, two, 2);
		graph.add_edge(one, five, 3);
		graph.add_edge(two, five, 2);
		graph.add_edge(two, six, 2);
		graph.add_edge(two, three, 3);
		graph.add_edge(three, seven, 2);
		graph.add_edge(three, four, 4);
		graph.add_edge(four, seven, 2);
		graph.add_edge(four, eight, 2);
		graph.add_edge(five, six, 3);
		graph.add_edge(six, seven, 1);
		graph.add_edge(seven, eight, 3);

		let (_cut_size, partition) = stoer_wagner(&graph);
		let res = partition.len() * (graph.verticies.len() - partition.len());
		assert_eq!(16, res);
    }

	#[test]
    fn test_stoer_wagner_weight1() {
        let input = include_str!("stoer_wagner.txt");
		let res = part1(input);
		assert_eq!(16, res);
    }

    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(54, part1(input));
    }

	#[test]
	fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(619225, part1(input));
    }
}