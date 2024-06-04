use crate::day_18::integer_polygons::shoelace_formula;
use crate::day_18::integer_polygons::picks_therom;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(input: &str) -> Vec<Dig> { 
    let dig_plan = input.lines().map(|line| {
        let mut sect = line.split_whitespace();
        sect.next();
        sect.next();
        let clr = sect.next().unwrap().to_string();
        let mut num = clr.replace("(#", "").replace(")", "");
        let dir = num.split_off(5);

        let num = num.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            let c_val = match c {
                'f' => "15".to_string(),
                'e' => "14".to_string(),
                'd' => "13".to_string(),
                'c' => "12".to_string(),
                'b' => "11".to_string(),
                'a' => "10".to_string(),
                _   => c.to_string(),
            };
            let c_val: i64 = c_val.parse().unwrap();
            let p_val: i64 = i64::pow(16, i as u32);
            acc + (c_val * p_val)
        });

        let dir = match dir.as_str() {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("Should be a value of 0-3")
        };
        Dig::new(dir.to_string(), num)
    }).collect::<Vec<Dig>>();
    return dig_plan;
}

pub fn process(input: &str) -> i64 {
    let dig_plan = parse(input);
    let mut max_row = 0;
    let mut pos = (0, 0);
    let mut verticies: Vec<(i64, i64)> = vec![];

    let mut meters_from_perimeter = 0;
    for dig in dig_plan {

        for _ in 1..=dig.num {
            match dig.dir.as_str() {
                "U" => pos.0 -= 1,
                "D" => pos.0 += 1,
                "L" => pos.1 -= 1,
                "R" => pos.1 += 1,
                _ => panic!("should have a directon")
            }
            if pos.0 > max_row { max_row = pos.0 }
            verticies.push(pos);
        }
        meters_from_perimeter += dig.num;    
    }
    verticies = convert_rowcol_to_xy(max_row, verticies);
    find_starting_vertex(&mut verticies);
    let area = shoelace_formula(&verticies);

    // - 1 to remove the duplicate start point
    let interior_points = picks_therom((verticies.len()-1)  as i64, area); 

    println!("Perimeter length {}", meters_from_perimeter);
    println!("Interior area {}", area);
    println!("Interior points {}", interior_points);
    
    return meters_from_perimeter + interior_points;
}

fn convert_rowcol_to_xy(max_row: i64, verticies: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    return verticies.iter().map(|(row, col)| (*col, max_row - row)).collect::<Vec<(i64, i64)>>();
}

fn find_starting_vertex(verticies: &mut Vec<(i64, i64)>) {
    let mut start_vertex = (i64::MAX, i64::MIN);
    let mut start_index = 0;
    for v in verticies.iter() {
        if v.0 < start_vertex.0 {
            start_vertex.0 = v.0;
        }
    }

    for (i, v) in verticies.iter().enumerate() {
        if v.0 == start_vertex.0 && v.1 > start_vertex.1 {
            start_vertex.1 = v.1;
            start_index = i;
        }
    }

    if start_index > 0 {
        let mut new_start = verticies.split_off(start_index);
        new_start.append(verticies);
        *verticies = new_start;
    }

    if verticies[0].1 == verticies[1].1 {
        let mut new_start = verticies.split_off(1);
        new_start.append(verticies);
        new_start.reverse();
        *verticies = new_start;
    }

    verticies.push(verticies[0]);
}

struct Dig {
    dir: String,
    num: i64,
}

impl Dig {
    fn new(dir: String, num: i64) -> Self {
        Self {
            dir,
            num,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(952408144115, process(input));
    }
}