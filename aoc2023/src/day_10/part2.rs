use std::collections::HashMap;

pub fn process(input: &str) -> i32 {
    let (pipe_map, mut maze_pos) = parse_maze(input);
    // println!("START: {:?} - ", maze_pos.start);
    check_borders_from_start(&mut maze_pos, &pipe_map);
    //println!("{:?}", maze_pos.dir);
    loop {
        check_borders(&mut maze_pos, &pipe_map);
        //println!("{:?}", maze_pos.curr);
        if maze_pos.curr == maze_pos.start {
            break
        }
    }
    
    // for p in &maze_pos.path {
    //     print!("->({},{})", p.0, p.1);
    // }

    // println!("\n{:?}", maze_pos.verticies);
    find_starting_vertex(&mut maze_pos.verticies);
    // println!("\n{:?}", maze_pos.verticies);

    let area = shoelace_formula(&maze_pos.verticies);
    let interior = picks_therom(&maze_pos, area);
    // println!("Area = {}, Interior Points = {}", area, interior);
    return interior;
}

fn parse_maze(input: &str) -> (HashMap<(i32, i32), String>, Position) {
    let mut maze_pos: Position = Position::new();
    let mut pipe_map: HashMap<(i32, i32), String> = HashMap::new();

    let lines = input.lines().rev();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            pipe_map.insert((x, y), c.to_string());
            if c == 'S' {
                //println!("FOUND START: ({}{})", x, y);
                maze_pos.set_start(x, y);
            }
        }
    }
    // println!("Map: {:?}", pipe_map);
    return (pipe_map, maze_pos);
}

fn check_borders(pos: &mut Position, pipes: &HashMap<(i32, i32), String>) {
    
    let next_pos = match pos.dir {
        Direction::North => (pos.curr.0, pos.curr.1 + 1),
        Direction::South => (pos.curr.0, pos.curr.1 - 1),
        Direction::East => (pos.curr.0 + 1, pos.curr.1),
        Direction::West => (pos.curr.0 - 1, pos.curr.1),
    };

    pos.set_curr(next_pos.0, next_pos.1);

    let next_pipe = pipes.get(&(next_pos.0, next_pos.1)).unwrap();
    //println!("Next: pos({},{}), pipe {} going {:?}", next_pos.0, next_pos.1, next_pipe, pos.dir);
    
    if next_pipe.as_str() == "S" {
        return
    }

    pos.dir = match (next_pipe.as_str(), &pos.dir) {
        ("|", Direction::North) => Direction::North,  // is a vertical pipe connecting north and south.
        ("|", Direction::South) => Direction::South,  
        
        ("-", Direction::East) => Direction::East,  // is a horizontal pipe connecting east and west.
        ("-", Direction::West) => Direction::West,  

        // is a 90-degree bend connecting north and east.
        ("L", Direction::South) => {
            pos.verticies.push(next_pos);
            Direction::East
        },
        ("L", Direction::West) => {
            pos.verticies.push(next_pos);
            Direction::North
        },

        // is a 90-degree bend connecting north and west.
        ("J", Direction::South) => {
            pos.verticies.push(next_pos);
            Direction::West
        },  
        ("J", Direction::East) => {
            pos.verticies.push(next_pos);
            Direction::North
        },

        // is a 90-degree bend connecting south and west.
        ("7", Direction::North) => {
            pos.verticies.push(next_pos);
            Direction::West
        },  
        ("7", Direction::East) => {
            pos.verticies.push(next_pos);
            Direction::South
        },

        // is a 90-degree bend connecting south and east.
        ("F", Direction::North) => {
            pos.verticies.push(next_pos);
             Direction::East
        },
        ("F", Direction::West) => {
            pos.verticies.push(next_pos);
            Direction::South
        },

        _ => panic!("YOU WENT THE WRONG WAY!")
    }
}

fn check_borders_from_start(pos: &mut Position, pipes: &HashMap<(i32, i32), String>) {
    let (x, y) = pos.curr;
    let north = (x, y+1);
    let east = (x+1, y);
    let west = (x-1, y);
    let south = (x, y-1);

    if "|LJ".contains(pipes.get(&south).unwrap()) {
        pos.dir = Direction::South;
    } else if "-J7".contains(pipes.get(&east).unwrap()) {
        pos.dir = Direction::East;
    } else if "|F7".contains(pipes.get(&north).unwrap()) {
        pos.dir = Direction::North;
    }  else if "-LF".contains(pipes.get(&west).unwrap()) {
        pos.dir = Direction::West;
    } else {
        panic!("Couldn't find pipe")
    }
}

fn find_starting_vertex(verticies: &mut Vec<(i32, i32)>) {
    let mut start_vertex = (i32::MAX, i32::MIN);
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

fn shoelace_formula(permiter: &Vec<(i32, i32)>) -> i32 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut det: i32 = 0;
    for i in 0..&permiter.len()-1 {
        let (x1, y1) = permiter[i];
        let (x2, y2) = permiter[i+1];

        det += (x1*y2)-(x2*y1);
    }
    return det/2;
}

fn picks_therom(maze_pos: &Position, area: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1
    // Where:
    // A is area of the polygon
    // i is the interior number of points
    // b is the number of intger points on the polygon boundary (all points not just corners)

    // find i: i = A - b/2 + 1
    let b = (maze_pos.path.len() + 1) as i32;
    // println!("number of points on boundary: {b}");
    let interior_points = area - (b/2) + 1;

    return interior_points;
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
    verticies: Vec<(i32, i32)>,
    steps: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            start: (0, 0),
            curr: (0, 0),
            dir: Direction::North,
            path: vec![],
            verticies: vec![],
            steps: 0,
        }
    }

    fn set_start(&mut self, x: i32, y: i32) {
        self.start = (x, y);
        self.curr = (x, y);
        self.verticies.push(self.start)
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
        let input = include_str!("simple_enclose.txt");
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_simple2() {
        let input = include_str!("simple_enclose2.txt");
        assert_eq!(4, process(input));
    }

    #[test]
    fn test_large() {
        let input = include_str!("large_enclose.txt");
        assert_eq!(8, process(input));
    }
    
    #[test]
    fn test_junk() {
        let input = include_str!("junk_enclose.txt");
        assert_eq!(10, process(input));
    }

    #[test]
    fn test_part2_soln() {
        let input = include_str!("input.txt");
        assert_eq!(579, process(input));
    }
}