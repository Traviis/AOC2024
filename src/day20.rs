use regex::Regex;
use std::collections::{HashMap, VecDeque};

type InputType = HashMap<String, Module>;
type OutputType = u64;

#[derive(Debug, Clone)]
pub enum Module {
    Broadcaster(Vec<String>),
    FlipFlop(Pulse, Vec<String>),
    Conjunction(Vec<(String, Pulse)>, Vec<String>), //default low, remember last input FROM EACH INPUT
}
//process in the order they are sent

#[derive(Debug, Clone, Copy,PartialEq,Eq)]
pub enum Pulse {
    Low,
    High,
}

#[aoc_generator(day20)]
fn day20_parse(input: &str) -> InputType {
    //First get all of the modules, then make a second pass for the conjunction modules and find
    //their number of inputs; Order them by name, since while order matters for propogation, it
    //doesn't matter for determining state

    let reg = Regex::new(r"([%&]*)([a-z]+) -> ([a-z, ]+)").unwrap();

    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|l| {
            let caps = reg.captures(l).unwrap();

            let mod_type = caps.get(1).unwrap().as_str();
            let name = caps.get(2).unwrap().as_str().to_string();
            let dests = caps
                .get(3)
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            (
                name,
                match mod_type {
                    "%" => Module::FlipFlop(Pulse::Low, dests),
                    "&" => Module::Conjunction(vec![], dests),
                    _ => Module::Broadcaster(dests),
                },
            )
        })
        .collect();

    let cmods = modules.clone(); //Don't even with me

    //Now go through and find the conjunctions
    modules.iter_mut().for_each(|(name, modu)| {
        if let Module::Conjunction(ref mut inputs, _) = modu {
            //Once you find a conjunction, find all places that input to it
            cmods.iter().for_each(|(name2, modu2)| match modu2 {
                Module::Broadcaster(dests) => {
                    if dests.contains(name) {
                        inputs.push((name2.clone(), Pulse::Low));
                    }
                }
                Module::FlipFlop(_, dests) => {
                    if dests.contains(name) {
                        inputs.push((name2.clone(), Pulse::Low));
                    }
                }
                Module::Conjunction(_, dests) => {
                    if dests.contains(name) {
                        inputs.push((name2.clone(), Pulse::Low));
                    }
                }
                _ => (),
            });
        }
    });

    modules
}

fn count_pulses(input: &InputType, presses: i64) -> OutputType {
    let mut modules = input.clone(); //State is stored in the map

    let mut queue = VecDeque::new();
    //Queue is: destination_name, pulse_type, and source_name

    let mut low_pulses = presses; //From the button press
    let mut high_pulses = 0;

    let broadcast_node = modules.get("broadcaster").unwrap().clone();

    for pressed in 0..presses {
        let dests = match broadcast_node {
            Module::Broadcaster(ref dests) => dests,
            _ => unreachable!(),
        };
        for dest in dests.iter() {
            low_pulses += 1;
            queue.push_back((dest.clone(), Pulse::Low, "broadcaster".to_string())); //Send low pulse to all
        }

        while let Some((name, pulse, source)) = queue.pop_front() {
            let modu = modules.get_mut(&name);
            if modu.is_none() {
                continue;
            }

            match modu.unwrap() {
                Module::Broadcaster(_) => unreachable!(),
                Module::FlipFlop(state, dests) => {
                    if pulse == Pulse::Low {
                        //Low pulse
                        *state = match *state {
                            Pulse::Low => Pulse::High,
                            Pulse::High => Pulse::Low,
                        };
                        for dest in dests.iter() {
                            match *state {
                                Pulse::High => high_pulses += 1,
                                Pulse::Low => low_pulses += 1,
                            };
                            queue.push_back((dest.clone(), *state, name.clone()));
                        }
                    } else {
                        //High pulse are ignored
                    }
                }
                Module::Conjunction(sources, dests) => {
                    //Update the remembered values
                    sources.iter_mut().for_each(|(name, state)| {
                        if *name == source {
                            *state = pulse;
                        }
                    });
                    let all_high = sources.iter().all(|(_, state)| *state == Pulse::High);
                    
                    let pulse_to_send = if all_high { Pulse::Low } else { Pulse::High };

                    for dest in dests.iter() {
                        match pulse_to_send {
                            Pulse::High => high_pulses += 1,
                            Pulse::Low => low_pulses += 1,
                        };
                        queue.push_back((dest.clone(), pulse_to_send, name.clone()));
                    }
                }
            }
        }
    }

    (low_pulses * high_pulses) as u64
}

#[aoc(day20, part1)]
pub fn part1(input: &InputType) -> OutputType {
    count_pulses(input, 1000)
}

#[aoc(day20, part2)]
pub fn part2(input: &InputType) -> OutputType {
    //We have to assume that these cycle, find the conjunction that attachs to the rx node.
    // It will be low when all of it's inputs are high, thus, we can find the cycle length for each of the second to last nodes inputs and then find the least common multiple of them.
    let mut modules = input.clone(); //State is stored in the map

    let mut queue = VecDeque::new();
    //Queue is: destination_name, pulse_type, and source_name

    let broadcast_node = modules.get("broadcaster").unwrap().clone();

    let mut seen : HashMap<String, i64> = HashMap::new();
    let mut cycle_length : HashMap<String, i64>::new() = HashMap::new();

    loop {
        let dests = match broadcast_node {
            Module::Broadcaster(ref dests) => dests,
            _ => unreachable!(),
        };
        for dest in dests.iter() {
            queue.push_back((dest.clone(), Pulse::Low, "broadcaster".to_string())); //Send low pulse to all
        }

        while let Some((name, pulse, source)) = queue.pop_front() {
            let modu = modules.get_mut(&name);
            if modu.is_none() {
                continue;
            }

            match modu.unwrap() {
                Module::Broadcaster(_) => unreachable!(),
                Module::FlipFlop(state, dests) => {
                    if pulse == Pulse::Low {
                        //Low pulse
                        *state = match *state {
                            Pulse::Low => Pulse::High,
                            Pulse::High => Pulse::Low,
                        };
                        for dest in dests.iter() {
                            match *state {
                                Pulse::High => high_pulses += 1,
                                Pulse::Low => low_pulses += 1,
                            };
                            queue.push_back((dest.clone(), *state, name.clone()));
                        }
                    } else {
                        //High pulse are ignored
                    }
                }
                Module::Conjunction(sources, dests) => {
                    //Update the remembered values
                    sources.iter_mut().for_each(|(name, state)| {
                        if *name == source {
                            *state = pulse;
                        }
                    });
                    let all_high = sources.iter().all(|(_, state)| *state == Pulse::High);
                    
                    let pulse_to_send = if all_high { Pulse::Low } else { Pulse::High };

                    for dest in dests.iter() {
                        match pulse_to_send {
                            Pulse::High => high_pulses += 1,
                            Pulse::Low => low_pulses += 1,
                        };
                        queue.push_back((dest.clone(), pulse_to_send, name.clone()));
                    }
                }
            }
        }
    }



}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
    }

    #[test]
    fn day20_part1() {
        assert_eq!(part1(&day20_parse(get_test_input())), 32000000);
    }

    // #[test]
    // fn day20_part2() {
    //     assert_eq!(part2(&day20_parse(get_test_input())), 0);
    // }
}
