use std::time::Instant;
use std::collections::HashMap;
use std::ops::Range;
use std::fs;

pub fn run() {
    let input_file = fs::read_to_string("src\\day_24\\input.txt").expect("File not found!");

    println!("\n--Day 24------");
    let now = Instant::now();
	let p1 = part1(&input_file, 200000000000000f64, 400000000000000f64);
	println!("Part 1: {p1} in [{}ms]", now.elapsed().as_millis());

    let now = Instant::now();
	let p2 = part2(&input_file);
	println!("Part 2: {p2} in [{}s]", now.elapsed().as_secs());
    println!("--------------");
}

fn parse(input: &str) -> Puzzle {
    let hailstone_map = input.lines().enumerate().map(|(id, line)| {
        let mut info = line.split("@");
        let pos = info.next().unwrap();
        let vel = info.next().unwrap();

        let pos = Pos::new(pos.split(", ").map(|loc| loc.trim().parse().unwrap()).collect());
        let vel = Vel::new(vel.split(", ").map(|loc| loc.trim().parse().unwrap()).collect());
        let stone = Hailstone::new(pos, vel);
        (id, stone)
    }).collect::<HashMap<usize, Hailstone>>();
    
    return Puzzle::new(hailstone_map);
}

fn part1(input: &str, left: f64, right: f64) -> usize {
    let puzzle = parse(input);
    return evalue_test_area(puzzle, left..right);
}

fn part2(input: &str) -> f64 {
    let puzzle = parse(input);

    // Get Rock X, Y, Vx, Vy
    let mut a_matrix: Vec<Vec<f64>> = vec![vec![0f64; 4]; 4];
    let mut b_matrix = [0f64; 4];
    for i in 0..4 {
        let hs1 = puzzle.hailstone_map.get(&i).unwrap();
        let hs2 = puzzle.hailstone_map.get(&(i+1)).unwrap();
                
        a_matrix[i][0] = hs2.vel.y - hs1.vel.y;
        a_matrix[i][1] = hs1.vel.x - hs2.vel.x;
        a_matrix[i][2] = hs1.pos.y - hs2.pos.y;
        a_matrix[i][3] = hs2.pos.x - hs1.pos.x;
        
        b_matrix[i] = (-hs1.pos.x * hs1.vel.y) + 
                      (hs1.pos.y * hs1.vel.x) + 
                      (hs2.pos.x * hs2.vel.y) - 
                      (hs2.pos.y * hs2.vel.x);
    }
    gaussian_elim(&mut a_matrix, &mut b_matrix);
    // dbg!(&b_matrix);

    let x = b_matrix[0].round();
    let y = b_matrix[1].round();
    let vx = b_matrix[2].round();
    // let vy = b_matrix[3].round();

    // Get Z, VZ
    let mut a_matrix: Vec<Vec<f64>> = vec![vec![0f64; 2]; 2];
    let mut b_matrix = [0f64; 2];
    for i in 0..2 {
        let hs1 = puzzle.hailstone_map.get(&i).unwrap();
        let hs2 = puzzle.hailstone_map.get(&(i+1)).unwrap();
                
        a_matrix[i][0] = hs1.vel.x - hs2.vel.x;
        a_matrix[i][1] = hs2.pos.x - hs1.pos.x;
        
        b_matrix[i] = (-hs1.pos.x * hs1.vel.z) + 
                      (hs1.pos.z * hs1.vel.x) + 
                      (hs2.pos.x * hs2.vel.z) - 
                      (hs2.pos.z * hs2.vel.x) - 
                      ((hs2.vel.z - hs1.vel.z) * x) - 
                      ((hs1.pos.z - hs2.pos.z) * vx);
    }
    // dbg!(&a_matrix, &b_matrix);
    gaussian_elim(&mut a_matrix, &mut b_matrix);
    // dbg!(&b_matrix);

    let z = b_matrix[0].round();
    // let vz = b_matrix[1].round();

    // println!("Found Rock at ({x}, {y}, {z}) with vel ({vx}, {vy}, {vz})");

    return x+y+z;
}

fn gaussian_elim(a: &mut Vec<Vec<f64>>, b: &mut [f64]) {
    let n = a.len();
    for i in 0..n {
        let pivot = a[i][i];
        for j in 0..n {
            a[i][j] = a[i][j] / pivot;
        }
        b[i] = b[i] / pivot;

        for k in 0..n {
            if k != i {
                let factor = a[k][i];
                for m in 0..n {
                    a[k][m] = a[k][m] - (factor * a[i][m])
                }
                b[k] = b[k] - (factor * b[i]);
            }
        }
    }
}

fn evalue_test_area(puzzle: Puzzle, test_range: Range<f64>) -> usize {
    let mut passing_stones = 0;
    for i in 0..puzzle.stone_count {
        let hs1 = puzzle.hailstone_map.get(&i).unwrap();
        for j in i+1..puzzle.stone_count {
            let hs2 = puzzle.hailstone_map.get(&j).unwrap();
           
            if hs1.a*hs2.b == hs1.b*hs2.a {
                // println!("HS {i} & HS {j} are Parallel");
                continue;
            }

            let x = ((hs1.c*hs2.b) - (hs2.c*hs1.b)) / ((hs1.a*hs2.b) - (hs2.a*hs1.b));
            let y = ((hs2.c*hs1.a) - (hs1.c*hs2.a)) / ((hs1.a*hs2.b) - (hs2.a*hs1.b));
            
            let is_within_range = test_range.contains(&x) && test_range.contains(&y);
            
            let is_hs1_future_point = ((x - hs1.pos.x)*hs1.vel.x) > 0f64 && ((y - hs1.pos.y)*hs1.vel.y) > 0f64;
            let is_hs2_future_point = ((x - hs2.pos.x)*hs2.vel.x) > 0f64 && ((y - hs2.pos.y)*hs2.vel.y) > 0f64;
            
            if is_within_range && is_hs1_future_point && is_hs2_future_point {
                // println!("HS {i} & HS {j} intersect INSIDE the test range at ({:.3}, {:.3})", x, y);
                passing_stones += 1;
                continue;
            }

            // if !is_within_range {
            //     println!("HS {i} & HS {j} intersect OUTSIDE the test range at ({:.3}, {:.3})", x, y);
            // }

            // if !is_hs1_future_point {
            //     println!("HS {i} & HS {j} intersect in the past for HS {i} at ({:.3}, {:.3})", x, y);
            // }

            // if !is_hs2_future_point {
            //     println!("HS {i} & HS {j} intersect in the past for HS {j} at ({:.3}, {:.3})", x, y);
            // }
        }
    }
    // 19548 is too high
    return passing_stones;
}

struct Puzzle {
    hailstone_map: HashMap<usize, Hailstone>,
    stone_count: usize,
}

impl Puzzle {
    fn new(hailstone_map: HashMap<usize, Hailstone>) -> Self {
        Self {
            stone_count: hailstone_map.len(),
            hailstone_map,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Pos,
    vel: Vel,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(pos: Pos, vel: Vel) -> Self {
        Self {
            pos,
            vel,
            a: vel.y,
            b: -vel.x,
            c: (vel.y*pos.x) - (vel.x*pos.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

impl Pos {
    fn new(pos_vec: Vec<f64>) -> Self {
        Self {
            x: pos_vec[0],
            y: pos_vec[1],
            z: pos_vec[2],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vel {
    x: f64,
    y: f64,
    z: f64,
}

impl Vel {
    fn new(vel_vec: Vec<f64>) -> Self {
        Self {
            x: vel_vec[0],
            y: vel_vec[1],
            z: vel_vec[2],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(2, part1(input, 7f64, 27f64));
        assert_eq!(47f64, part2(input));
    }

	#[test]
	fn test_input() {
        let input = include_str!("input.txt");
        assert_eq!(17906, part1(input, 200000000000000f64, 400000000000000f64)); // 1.908s
        // assert_eq!(6378, part2(input)); // 8hr 45m 41s
    }
}