use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let (pipe_map, mut maze_pos) = parse_maze(input);
    print!("START: {:?} - ", maze_pos.start);
    check_borders_from_start(&mut maze_pos, &pipe_map);
    println!("{:?}", maze_pos.dir);
    loop {
        check_borders(&mut maze_pos, &pipe_map);
        println!("{:?}", maze_pos.curr);
        if maze_pos.curr == maze_pos.start {
            break
        }
    }
    return maze_pos.steps/2;
}

fn parse_maze(input: &str) -> (HashMap<(i32, i32), String>, Position) {
    let mut maze_pos: Position = Position::new();
    let mut pipe_map: HashMap<(i32, i32), String> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            pipe_map.insert((x, y), c.to_string());
            if c == 'S' {
                println!("FOUND START: ({}{})", x, y);
                maze_pos.set_start(x, y);
            }
        }
    }
    // println!("Map: {:?}", pipe_map);
    return (pipe_map, maze_pos);
}

fn check_borders(pos: &mut Position, pipes: &HashMap<(i32, i32), String>) {
    
    let next_pos = match pos.dir {
        Direction::North => (pos.curr.0, pos.curr.1 - 1),
        Direction::South => (pos.curr.0, pos.curr.1 + 1),
        Direction::East => (pos.curr.0 + 1, pos.curr.1),
        Direction::West => (pos.curr.0 - 1, pos.curr.1),
    };

    pos.set_curr(next_pos.0, next_pos.1);

    let next_pipe = pipes.get(&(next_pos.0, next_pos.1)).unwrap();
    println!("Next: pos({},{}), pipe {} going {:?}", next_pos.0, next_pos.1, next_pipe, pos.dir);
    
    if next_pipe.as_str() == "S" {
        return
    }

    pos.dir = match (next_pipe.as_str(), &pos.dir) {
        ("|", Direction::North) => Direction::North,  // is a vertical pipe connecting north and south.
        ("|", Direction::South) => Direction::South,  
        
        ("-", Direction::East) => Direction::East,  // is a horizontal pipe connecting east and west.
        ("-", Direction::West) => Direction::West,  

        ("L", Direction::South) => Direction::East,  // is a 90-degree bend connecting north and east.
        ("L", Direction::West) => Direction::North,

        ("J", Direction::South) => Direction::West,  // is a 90-degree bend connecting north and west.
        ("J", Direction::East) => Direction::North,  

        ("7", Direction::North) => Direction::West,  // is a 90-degree bend connecting south and west.
        ("7", Direction::East) => Direction::South,  

        ("F", Direction::North) => Direction::East,  // is a 90-degree bend connecting south and east.
        ("F", Direction::West) => Direction::South,  // is a 90-degree bend connecting south and east.

        _ => panic!("YOU WENT THE WRONG WAY!")
    }
}

fn check_borders_from_start(pos: &mut Position, pipes: &HashMap<(i32, i32), String>) {
    let (x, y) = pos.curr;
    let north = (x, y-1);
    let east = (x+1, y);
    let west = (x-1, y);
    let south = (x, y+1);

    if "|F7".contains(pipes.get(&north).unwrap()) {
        pos.dir = Direction::North;
    }  else if "|LJ".contains(pipes.get(&south).unwrap()) {
        pos.dir = Direction::South;
    } else if "-J7".contains(pipes.get(&east).unwrap()) {
        pos.dir = Direction::East;
    } else if "-LF".contains(pipes.get(&west).unwrap()) {
        pos.dir = Direction::West;
    } else {
        panic!("Couldn't find pipe")
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Position {
    start: (i32, i32),
    curr: (i32, i32),
    dir: Direction,
    path: Vec<(i32, i32)>,
    steps: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            start: (0, 0),
            curr: (0, 0),
            dir: Direction::North,
            path: vec![],
            steps: 0,
        }
    }

    fn set_start(&mut self, x: i32, y: i32) {
        self.start = (x, y);
        self.curr = (x, y);
    }

    fn set_curr(&mut self, x: i32, y: i32) {
        self.curr = (x, y);
        self.path.push(self.curr);
        self.steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = include_str!("simple.txt");
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_simple_noisy() {
        let input = include_str!("simple_noisy.txt");
        assert_eq!(4, process(input));
    }
    
    #[test]
    fn test_complex() {
        let input = include_str!("complex.txt");
        assert_eq!(8, process(input));
    }
    
    #[test]
    fn test_complex_noisy() {
        let input = include_str!("complex_noisy.txt");
        assert_eq!(8, process(input));
    }

    #[test]
    fn test_part1_soln() {
        let input = include_str!("input.txt");
        assert_eq!(6738, process(input));
    }
}