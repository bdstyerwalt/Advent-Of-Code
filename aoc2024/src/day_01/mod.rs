use std::collections::HashMap;

pub fn run() {
    let day_idx = file!().find("day_").expect("Couldn't find `day_` in file path") + 4;
    let day = file!().get(day_idx..day_idx+2).unwrap();
    let input_file = include_str!("input.txt");
    
    println!("\n--Day {day}------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn parse(input: &str) -> Puzzle {
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        let temp = line.split_whitespace().collect::<Vec<&str>>();
        list_a.push(temp[0].parse().unwrap());
        list_b.push(temp[1].parse().unwrap());
    }
    
    //println!("{:?}", list_a);
    //println!("{:?}", list_b);
    return Puzzle { list_a, list_b };
}

fn part1(input_file: &str) -> i32 {
    let puzzle = parse(&input_file);
    let mut a = puzzle.list_a;
    let mut b = puzzle.list_b;
    a.sort();
    b.sort();

    let combo = a.iter().zip(b.iter());
    let p1 = combo.fold(0, |acc, (x, y)| acc + (x - y).abs());
    // println!("{:?}", p1);

    return p1;
}

fn part2(input_file: &str) -> i32 {    
    let mut puzzle = parse(&input_file);
    let mut id_map: HashMap<i32, i32> = HashMap::new();

    let p2 = puzzle.list_a.iter().fold(0, |acc, a| {
        match id_map.get(&a) {
            Some(v) => acc + a*v,
            None => {
                let found: Vec<i32> = puzzle.list_b.extract_if(.., |b| b == a).collect();
                let cnt = found.len() as i32;
                id_map.insert(*a, cnt);
                return acc + a * cnt;
            }
        }
    });
        
    return p2;
}

struct Puzzle {
    list_a: Vec<i32>,
    list_b: Vec<i32>
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_p1() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(11, p1);
    }

    #[test]
    fn test_sample_p2() {
        let input = include_str!("sample.txt");
        let p2 = part2(input);
        dbg!(p2);
        assert_eq!(31, p2);
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let (p1, p2) = (part1(input), part2(input));
        dbg!(p1, p2);
        assert_eq!(1110981, p1);
        assert_eq!(24869388, p2);
    }
}