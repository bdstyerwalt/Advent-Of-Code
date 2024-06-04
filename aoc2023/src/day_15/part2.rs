use std::collections::HashMap;


fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> u32 {
    let lenses = parse(input);
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    for lens in lenses {
        if lens.contains('-') {
            let mut info = lens.split('-');
            let label = info.next().expect("Should be a str");
            let hash = hash_step(label);

            if let Some(mut box_vec) = boxes.remove(&hash) {
                box_vec.retain(|&x| x.0 != label);
                boxes.insert(hash, box_vec);
            }
        } else if lens.contains('=') {
            let mut info = lens.split('=');
            let label = info.next().expect("Should be a str");
            let hash = hash_step(label);

            let focal: u32 = info.next().expect("Should have a value here")
                                 .parse().expect("Should be an integer");
            if let Some(mut box_vec) = boxes.remove(&hash) {
                
                if box_vec.iter().any(|x| x.0 == label) {
                    let mut ins_idx = 0;
                    for (i, (lbl, _f)) in box_vec.iter().enumerate() {
                        if lbl == &label {
                            ins_idx = i;
                            break;
                        }
                    }
                    box_vec.remove(ins_idx);
                    box_vec.insert(ins_idx, (label, focal));
                } else {   
                    box_vec.push((label, focal));
                }
                boxes.insert(hash, box_vec);
            } else {
                let box_vec = vec![(label, focal)];
                boxes.insert(hash, box_vec);
            }
        }
    }
    return boxes.iter().map(|(box_num, box_vec)| {
      box_vec.iter().enumerate().map(|(i, (lens, focal))| {
        let res = (box_num + 1) * (i as u32 + 1) * focal;
        // println!("(box {box_num} * {} (slot [{lens}]) * {focal} (focal length) = {res}", i+1);
        return res;
      }).sum::<u32>()
    }).sum();
}

fn parse(input: &str) -> Vec<&str> {
    let steps: Vec<&str> = input.split(",").collect();
    return steps;
}

fn hash_step(step: &str) -> u32 {
    let val = step.chars().fold(0, |acc, c| {
        ((acc + c as u32) * 17) % 256
    });
    return val;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(145, process(input));
    }
}
