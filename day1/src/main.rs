/*  The newly-improved calibration document consists of lines of text; 
    Each line originally contained a specific calibration value that 
    the Elves now need to recover. On each line, the calibration value
    can be found by combining the first digit and the last digit (in that order)
    to form a single two-digit number.
*/

use std::{fs, error::Error};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = fs::read_to_string("input.txt")?;
    
    let re = Regex::new(r"(\D*)(?<first>\d)(\w*\d*)(?<last>\d)(\w*)").unwrap();
    let single_num = Regex::new(r"(\D*)(?<first>\d)").unwrap();

    let mut acc: i32 = 0;
    for line in input_file.lines() {
        let mut first: i32;
        let mut last: i32;
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            first = caps["first"].parse().unwrap();
            last = caps["last"].parse().unwrap();
        } else if single_num.is_match(line) {
            let caps = single_num.captures(line).unwrap();
            first = caps["first"].parse().unwrap();
            last = first;
        } else {
            todo!()
        }
        acc += 10*first + last;
        print!("{:?}", first);
        print!("{:?} ", last);
        println!("{}\n---", acc);
    }
    
    println!("\n\n------\n{}\n-----", acc);
    return Ok(())
}
