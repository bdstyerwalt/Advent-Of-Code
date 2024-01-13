use std::collections::{BinaryHeap, HashSet, VecDeque, HashMap};
use std::cmp::Ordering;
use std::hash::Hash;

fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input);
    let p2 = part2(input);
    println!("Part 1: {} | Part 2: {}", p1, p2);
}

fn parse(input: &str) -> Puzzle {
    let slabs = input.lines().enumerate().map(|(_row, line)| {
        let mut line = line.split("~");
        let beg: Vec<usize> = line.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let end: Vec<usize> = line.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let slab = Slab::new(Coord::new(beg), Coord::new(end));
        // println!("Slab {row}: {:?}", slab);
        return slab
    }).collect::<BinaryHeap<Slab>>();
    return Puzzle::new(slabs);
}

fn part1(input: &str) -> usize {
    let mut puzzle = parse(input);
    puzzle.apply_slab_gravity();
    puzzle.check_supporting_slabs();

    let mut disentegrate_count = 0;
    for (_id, slabs) in &puzzle.supports {
        let mut can_disentegrate = true;
        for id2 in slabs {
            let sby = puzzle.supported_by.get(id2).unwrap();
            // println!("{id2} - {:?}", sby);
            if sby.len() < 2 {
                can_disentegrate = false;
                break;
            }
        }
        if can_disentegrate { 
            disentegrate_count += 1;
            // println!("{id} can be disentegrated");
        }
    }

    // 574 is too high
    return disentegrate_count;
}

fn part2(input: &str) -> usize {
    let mut puzzle = parse(input);
    puzzle.apply_slab_gravity();
    puzzle.check_supporting_slabs();

    let mut fall_count = 0;
    for (_id, slabs) in &puzzle.supports {
        let mut eval: VecDeque<String> = VecDeque::new();
        let mut falling_slabs: HashSet<String> = HashSet::new();
        for id2 in slabs {
            let sby = puzzle.supported_by.get(id2).unwrap();
            if sby.len() < 2 {
                // if slab would fall add it to eval
                eval.push_back(id2.clone());
                falling_slabs.insert(id2.clone());
            }
        }       

        while let Some(eval_id) = eval.pop_front() {
            let slabs2 = puzzle.supports.get(&eval_id).unwrap();
            for slab in slabs2 {
                // if slab would fall add cascading slabs to eval
                let sby = puzzle.supported_by.get(slab).unwrap();
                if sby.iter().all(|s| falling_slabs.contains(s)) {
                    // println!("{slab} would also fall because of {eval_id}");
                    // if slab would fall add it to eval
                    eval.push_back(slab.clone());
                    falling_slabs.insert(slab.clone());
                }
            }
        }
        // if !falling_slabs.is_empty() {
        //     println!("{id} would cause {falling_slabs:?} to fall: ");
        // }
        fall_count += falling_slabs.len();
    }

    // 77900 is too high
    return fall_count;
}

struct Puzzle {
    // change to binary heap with custom ordering
    slabs: BinaryHeap<Slab>,
    slab_ids: Vec<String>,
    supports: HashMap<String, Vec<String>>,
    supported_by: HashMap<String, Vec<String>>,
    supported_points: HashSet<Coord>,
}

impl Puzzle {
    fn new(slabs: BinaryHeap<Slab>) -> Self {
        Self {
            slabs,
            slab_ids: vec![],
            supports: HashMap::new(),
            supported_by: HashMap::new(),
            supported_points: HashSet::new(),
        }
    }

    fn apply_slab_gravity(&mut self) {
        // println!("Applying Slab Gravity...");
        let mut supported_points: HashSet<Coord> = HashSet::new();
        let mut count = 0;
        let alphabet = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                        "U", "V", "W", "X", "Y", "Z"];
        let mut reduced_slabs: BinaryHeap<Slab> = BinaryHeap::new();
        while let Some(mut slab) = self.slabs.pop() {
            // println!("{count} -> mod {}, div, {}", count%26, count/26);
            let mut id = alphabet[count%26].to_string();
            let mut temp = count;
            while temp >= 26 {
                temp = temp/26;
                id.push_str(alphabet[temp%26]);
            }
            slab.id = id.clone();
            self.slab_ids.push(id);
            // println!("\n\n----- Slab {} -----", slab.id);
            count += 1;
    
            let mut decrement: isize = -1;
            let mut found_support = false;
            let mut test_points = slab.points.clone();
            let mut new_points: VecDeque<Coord> = VecDeque::new();
            while !found_support {
                decrement += 1;
                while let Some(mut p) = test_points.pop_front() {
                    p.z -= 1;
                    if supported_points.contains(&p) || p.z == 0 {
                        found_support = true;
                        // println!("Found support at {p:?}");
                    }
                    new_points.push_back(p);
                }
                std::mem::swap(&mut test_points, &mut new_points);
            }
    
            slab.points.clear();
            for mut p in test_points {
                p.z += 1; // raise one block above "support"
                // println!("Adding point {p:?}");
                supported_points.insert(p);
                slab.points.push_back(p)
            }
            slab.beg.z -= decrement as usize;
            slab.end.z -= decrement as usize;
    
            reduced_slabs.push(slab);
        }

        self.slabs = reduced_slabs;
        self.supported_points = supported_points;
    }

    fn check_supporting_slabs(&mut self) {
        // println!("Checking Slab Supports...");
        let mut slabs = self.slabs.clone();
        let mut supports: HashMap<String, Vec<String>> = HashMap::new();
        let mut supported_by: HashMap<String, Vec<String>> = HashMap::new();
        while let Some(slab) = slabs.pop() {
            let mut sup_vals: Vec<String> = vec![];
            let mut sby_vals: Vec<String> = vec![];
            for s2 in &self.slabs {
                if slab.id == s2.id { continue; }

                let test_points = slab.points.clone();
                for mut p in test_points {
                    p.z += 1;
                    if s2.points.contains(&p) {
                        // println!("{} supports {}", slab.id, s2.id);
                        sup_vals.push(s2.id.clone());
                        break;
                    }
                }

                let test_points = slab.points.clone();
                for mut p in test_points {
                    p.z -= 1;
                    if s2.points.contains(&p) {
                        // println!("{} supported by {}", slab.id, s2.id);
                        sby_vals.push(s2.id.clone());
                    }
                }
            }
            supports.insert(slab.id.clone(), sup_vals);

            for sby_id in sby_vals {
                let mut s_vec: Vec<String> = match supported_by.get(&slab.id) {
                    Some(s_vec) => s_vec.clone(),
                    None => vec![],
                };
                if s_vec.contains(&sby_id) {
                    continue;
                }
                s_vec.push(sby_id.clone());
                // println!("{} supported by {:?}", &slab.id, &s_vec);
                supported_by.insert(slab.id.clone(), s_vec);
            }
        }
        self.supports = supports;
        self.supported_by = supported_by;
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Slab {
    id: String,
    beg: Coord,
    end: Coord,
    points: VecDeque<Coord>,
    slabs_beneath: usize,
    can_disentegrate: bool,
}

impl Slab {
    fn new(beg: Coord, end: Coord) -> Self {
        Self {
            id: String::from(""),
            beg,
            end,
            points: Slab::get_points(&beg, &end),
            slabs_beneath: 0,
            can_disentegrate: false,
        }
    }
    
    fn get_points(beg: &Coord, end: &Coord) -> VecDeque<Coord> {
        // loop over range of x,y,z to find all 3D points
        let mut points: VecDeque<Coord> = VecDeque::new();
        if beg.x != end.x {
            for p in beg.x..=end.x {
                let mut pt = beg.clone();
                pt.x = p;
                points.push_back(pt);
            }

        } else if beg.y != end.y {
            for p in beg.y..=end.y {
                let mut pt = beg.clone();
                pt.y = p;
                points.push_back(pt);
            }
        } else if beg.z != end.z {
            for p in beg.z..=end.z {
                let mut pt = beg.clone();
                pt.z = p;
                points.push_back(pt);
            }
        } else {
            points.push_back(*beg);
        }
        return points;
    }
}

impl Ord for Slab {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.beg.z.cmp(&self.beg.z)
            .then_with(|| self.end.z.cmp(&other.end.z))
    }
}

impl PartialOrd for Slab {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn new(nums: Vec<usize>) -> Self {
        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(5, part1(input));
        assert_eq!(7, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(393, part1(input));
        assert_eq!(58440, part2(input));
    }

    #[test]
    fn test_get_points() {
        let beg = Coord::new(vec![4, 0, 21]);
        let end = Coord::new(vec![4, 0, 21]);
        let res = Slab::get_points(&beg, &end);
        assert_eq!(1, res.len())
    }
}