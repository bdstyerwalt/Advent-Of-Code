use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let p1 = part1(input);
    let p2 = part2(input);
    println!("Part 1: {} | Part 2: {}", p1, p2);
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mod_map = input.lines().map(|line| {
        let mut vals = line.split(" -> ");
        let loc = vals.next().unwrap();
        let dst = vals.next().unwrap();
        let dest_vec = dst.split(", ").map(|s| s.to_string()).collect();

        let module: Module;
        if loc.contains("broadcaster") {
            module = Module::Broadcast(Broadcaster { dest_vec });
        } else if loc.contains("%") {
            module = Module::FF(FlipFlop::new(dest_vec))
        } else if loc.contains("&") {
            module = Module::CJ(Conjunction::new(input_vec, dest_vec))
        }
        return module;
    }).collect::<HashMap<String, Module>>();

    dbg!(mod_map);
    return mod_map;
}

fn part1(input: &str) -> u32 {   
    let starting_pulse = Pulse::Low;
    let mut module_map = parse(input);
    let mut propogation_queue: VecDeque<Propogation> = VecDeque::new();
    propogation_queue.push_front(Propogation{ dest: "Brodcaster".to_string(), pulse: starting_pulse });

    let mut high_cnt = 0;
    let mut low_cnt = 0;
    
    while let Some(prop) = propogation_queue.pop_front() {
        let (tgt, pls) = (prop.dest, prop.pulse);
        // TODO: Figure out how this works with enums
        let mut module = module_map.remove(&tgt).unwrap();
        
        let (next_pulse, dest_vec) = Module::process(&mut module, &tgt, pls);
        
        for d in dest_vec {
            let new_prop = Propogation{ dest: d, pulse: next_pulse.clone() };
            propogation_queue.push_back(new_prop);    
        }
        module_map.insert(tgt, module);

        match next_pulse {
            Pulse::High => high_cnt += 1,
            Pulse::Low => low_cnt += 1,
            Pulse::Zero => (),
        }
    }
    return high_cnt * low_cnt;
}

fn part2(input: &str) -> u32 {   
    return 0;
}

#[derive(Debug)]
struct FlipFlop {
    is_on: bool,
    dest_vec: Vec<String>,
}

impl FlipFlop {
    fn new(dest_vec: Vec<String>) -> Self {
        Self {
            is_on: false,
            dest_vec: dest_vec,
        }
    }
    
    // If a flip-flop module receives a high pulse, it is ignored and nothing happens. 
    // However, if a flip-flop module receives a low pulse, it flips between on and off. 
    // If it was off, it turns on and sends a high pulse. 
    // If it was on, it turns off and sends a low pulse.
    fn process(&mut self, pulse: Pulse) -> Pulse {
        if pulse == Pulse::High {
            return Pulse::Zero;
        }
        
        match self.is_on {
            true => {
                self.is_on = false;
                return Pulse::Low;
            }
            false => {
                self.is_on = true;
                return Pulse::High;
            }
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
    dest_vec: Vec<String>,
}

// Conjunction modules remember the type of the most recent pulse
// received from each of their connected input modules
impl Conjunction {
    // initially default to remembering a low pulse for each input
    fn new(input_vec: Vec<String>, dest_vec: Vec<String>) -> Self {
        let mem = input_vec.iter().map(|dest| {
            (dest.clone(), Pulse::Low)
        }).collect::<HashMap<String, Pulse>>();
        
        Self {
            memory: mem,
            dest_vec: dest_vec,
        }
    }
    
    // When a pulse is received, the conjunction module first updates its memory for that input. 
    // Then, if it remembers high pulses for all inputs, it sends a low pulse; 
    // otherwise, it sends a high pulse.
    fn process(&mut self, input: &String, pulse: Pulse) -> Pulse {
        self.memory.insert(input.clone(), pulse);
        
        if self.memory.values().all(|inp| inp == &pulse) {
            return Pulse::Low
        }
        return Pulse::High
    }
}

#[derive(Debug)]
struct Broadcaster {
   dest_vec: Vec<String>, 
}

#[derive(Debug)]
enum Module {
    FF(FlipFlop),
    CJ(Conjunction),
    Broadcast(Broadcaster),
}

impl Module {
    fn process(module: &mut Module, input: &String, pulse: Pulse) -> (Pulse, Vec<String>) {
        let new_pulse: Pulse;
        let dest_vec: Vec<String>;
        match module {
            Module::FF(mut flip) => { 
                new_pulse = flip.process(pulse);
                dest_vec = flip.dest_vec;
            },
            Module::CJ(mut conj) => {
                new_pulse = conj.process(input, pulse);
                dest_vec = conj.dest_vec;
            },
            Module::Broadcast(bdcst) => {
                new_pulse = pulse;
                dest_vec = bdcst.dest_vec.clone();
            },
        }
        return (new_pulse, dest_vec);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
    Zero,
}

#[derive(Debug)]
struct Propogation {
    dest: String, 
    pulse: Pulse,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample1() {
        let input = include_str!("sample.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(32000000, p1);
    }

    #[test]
    fn test_sample2() {
        let input = include_str!("sample2.txt");
        let p1 = part1(input);
        dbg!(p1);
        assert_eq!(11687500, p1);
    }
}