use std::fs;

pub fn run() {
    let input_file: String = fs::read_to_string("src\\day_01\\input.txt").expect("File not found!");

    println!("\n--Day 01------");
    println!("Part 1: {}", &part1(&input_file));
    println!("Part 2: {}", &part2(&input_file));
    println!("--------------");
}

fn part1(input_file: &str) -> i32 {
    let elves = parse(&input_file);
    let p1 = find_max(&elves);
    return p1;
}

fn part2(input_file: &str) -> i32 {    
    let elves = parse(&input_file);
    let p2 = find_top_three(elves);
    return p2;
}


fn find_max(cal_vec: &Vec<i32>) -> i32 {
    return *cal_vec.iter().max().unwrap_or(&0i32);
}

fn find_top_three(mut cal_vec: Vec<i32>) -> i32 {
    let one = find_max(&cal_vec);
    let mut index = cal_vec.iter().position(|&x| x == one).unwrap();
    cal_vec.remove(index);
    // println!("1: {one} ({index})");
    
    let two = find_max(&cal_vec);
    index = cal_vec.iter().position(|&x| x == two).unwrap();
    cal_vec.remove(index);
    // println!("2: {two} ({index})");
    
    let three = find_max(&cal_vec);
    index = cal_vec.iter().position(|&x| x == three).unwrap();
    cal_vec.remove(index);
    // println!("3: {three} ({index})");

    
    return one + two + three;
}

fn parse(input: &str) -> Vec<i32> {
    let mut calories_vec = vec![];
    let mut elf_cals: i32 = 0;
    
    for line in input.lines() {
        if line.is_empty() {
            // println!("{elf_cals}");
            calories_vec.push(elf_cals);
            elf_cals = 0;
            continue
        }
        
        elf_cals += line.parse::<i32>().unwrap();
    }
    
    if elf_cals > 0 {
        // println!("{elf_cals}");
        calories_vec.push(elf_cals);
    }
    
    return calories_vec;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_file(file: &str) -> String {
        return fs::read_to_string(format!("src\\day_01\\{file}.txt")).expect("File not found!");
    }
    

    #[test]
    fn sample_p1() {
        let elf_cals = parse(&get_file("sample"));
        let result = find_max(&elf_cals);
        assert_eq!(result, 24000i32)
    }
    
    #[test]
    fn sample_p2() {
        let elf_cals = parse(&get_file("sample"));
        let result = find_top_three(elf_cals);
        assert_eq!(result, 45000i32)
    }

    #[test]
    fn input_p1() {
        let elf_cals = parse(&get_file("input"));
        let result = find_max(&elf_cals);
        assert_eq!(result, 66306i32)
    }
    
    #[test]
    fn input_p2() {
        let elf_cals = parse(&get_file("input"));
        let result = find_top_three(elf_cals);
        assert_eq!(result, 195292i32)
    }
}