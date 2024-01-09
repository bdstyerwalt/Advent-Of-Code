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
        let (t, mut loc) = loc.split_at(1);

        let dst = vals.next().unwrap();
        let dest_vec = dst.split(", ").map(|s| s.to_string()).collect();

        let module: Module;
        match t {
            "b" => {
                loc = "broadcaster";
                module = Module::Broadcast(Broadcaster { dest_vec });
            },
            "%" => module = Module::FF(FlipFlop::new(dest_vec)),
            "&" => module = Module::CJ(Conjunction::new(dest_vec)),    
            _ => panic!("Should be one of the previous arms")        
        }
        return (loc.to_string(), module);
    }).collect::<HashMap<String, Module>>();

    let mut final_map = mod_map.clone();
    for (i, (loc, module)) in mod_map.iter().enumerate() {
        let mut new_conj: Conjunction;
        match module {
            Module::CJ(conj) => {
                new_conj = conj.clone();
                for (j, (l2, m2)) in mod_map.iter().enumerate() {
                    if i == j { continue; }
        
                    let dest_vec: Vec<String>;
                    match m2 {
                        Module::FF(inner) => dest_vec = inner.dest_vec.clone(),
                        Module::CJ(inner) => dest_vec = inner.dest_vec.clone(),
                        Module::Broadcast(inner) => dest_vec = inner.dest_vec.clone(),
                    }
        
                    for tgt in &dest_vec {
                        if loc == tgt {
                            new_conj.memory.insert(l2.to_string(), Pulse::Low);
                        }
                    }
                }
                final_map.insert(loc.to_string(), Module::CJ(new_conj));
            },
            _ => continue,
        }
        
    }

    return final_map;
}

fn part1(input: &str) -> u32 {   
    let mut module_map = parse(input);
    let (mut high, mut low) = (0, 0);
    for _ in 0..1000 {
        let res = process(&mut module_map, Pulse::Low);
        high += res.0;
        low += res.1;
    }
    return high * low;
}

fn process(module_map: &mut HashMap<String, Module>, starting_pulse: Pulse) -> (u32, u32) {
    let mut high_cnt = 0;
    let mut low_cnt = 0;
    match starting_pulse {
        Pulse::High => high_cnt += 1,
        Pulse::Low => low_cnt += 1,
        Pulse::Zero => (),
    }

    let mut propogation_queue: VecDeque<Propogation> = VecDeque::new();
    propogation_queue.push_front(Propogation{ dest: "broadcaster".to_string(), pulse: starting_pulse });

    while let Some(prop) = propogation_queue.pop_front() {
        // dbg!(&prop);
        let (tgt, pls) = (prop.dest, prop.pulse);

        let module = module_map.get(&tgt).unwrap();
        let dest_vec = module.get_dests();
        let mut d_mod: Module;
        for d in dest_vec {
            let mut next_pulse = Pulse::Zero;
            if module_map.contains_key(&d) {
                d_mod = module_map.remove(&d).unwrap();
                (d_mod, next_pulse) = Module::process(d_mod, tgt.clone(), pls);
                module_map.insert(d.clone(), d_mod);
            }
            
            println!("{tgt} -{pls}-> {d}");
            match pls {
                Pulse::High => high_cnt += 1,
                Pulse::Low => low_cnt += 1,
                Pulse::Zero => continue,
            }
            
            if next_pulse == Pulse::Zero { continue; }
            let new_prop = Propogation{ dest: d, pulse: next_pulse.clone() };
            propogation_queue.push_back(new_prop);  
        }
        // dbg!(&propogation_queue);
        // println!("------\n");
    }
    dbg!(&high_cnt, &low_cnt);
    return (high_cnt, low_cnt);
}

fn part2(input: &str) -> u32 {   
    return 0;
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
    dest_vec: Vec<String>,
}

// Conjunction modules remember the type of the most recent pulse
// received from each of their connected input modules
impl Conjunction {
    // initially default to remembering a low pulse for each input
    fn new(dest_vec: Vec<String>) -> Self {
        // let mem = input_vec.iter().map(|dest| {
        //     (dest.clone(), Pulse::Low)
        // }).collect::<HashMap<String, Pulse>>();
        
        Self {
            memory: HashMap::new(),
            dest_vec: dest_vec,
        }
    }
    
    // When a pulse is received, the conjunction module first updates its memory for that input. 
    // Then, if it remembers high pulses for all inputs, it sends a low pulse; 
    // otherwise, it sends a high pulse.
    fn process(&mut self, input: String, pulse: Pulse) -> Pulse {
        // dbg!(&self.memory);
        self.memory.insert(input, pulse);
        // dbg!(&self.memory);
        
        if self.memory.values().all(|inp| inp == &Pulse::High) {
            return Pulse::Low
        }
        return Pulse::High
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
   dest_vec: Vec<String>, 
}

#[derive(Debug, Clone)]
enum Module {
    FF(FlipFlop),
    CJ(Conjunction),
    Broadcast(Broadcaster),
}

impl Module {
    fn process(module: Module, input: String, pulse: Pulse) -> (Module, Pulse) {
        let new_pulse: Pulse;
        match module {
            Module::FF(mut flip) => { 
                new_pulse = flip.process(pulse);
                return (Module::FF(flip), new_pulse);
            },
            Module::CJ(mut conj) => {
                new_pulse = conj.process(input, pulse);
                return (Module::CJ(conj), new_pulse);
            },
            Module::Broadcast(bdcst) => {
                return (Module::Broadcast(bdcst), pulse);
            },
        }
    }

    fn get_dests(&self) -> Vec<String> {
        match self {
            Module::FF(flip) => return flip.dest_vec.clone(),
            Module::CJ(conj) => return conj.dest_vec.clone(),
            Module::Broadcast(bdcst) => return bdcst.dest_vec.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
    Zero,
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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