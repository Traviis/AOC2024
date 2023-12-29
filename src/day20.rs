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

impl Module {
    fn dests(self: &Self) -> &Vec<String> {
        match self {
            Module::Broadcaster(dests) => dests,
            Module::FlipFlop(_, dests) => dests,
            Module::Conjunction(_, dests) => dests,
        }
    }
}
//process in the order they are sent

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            });
        }
    });

    modules
}

#[aoc(day20, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut modules = input.clone(); //State is stored in the map

    let mut queue = VecDeque::new();
    //Queue is: destination_name, pulse_type, and source_name
    let presses = 1000;

    let mut low_pulses = presses; //From the button press
    let mut high_pulses = 0;

    let broadcast_node = modules.get("broadcaster").unwrap().clone();

    let mut seen = HashMap::new();
    let mut cycle_length = HashMap::new();

    for press in 0..presses {
        propogate(
            press as u64,
            &broadcast_node,
            &mut modules,
            &mut queue,
            &mut high_pulses,
            &mut low_pulses,
            &mut cycle_length,
            &mut seen,
            "",
        );
    }

    (low_pulses * high_pulses) as u64
}

fn propogate(
    cycles: u64,
    broadcast_node: &Module,
    modules: &mut InputType,
    queue: &mut VecDeque<(String, Pulse, String)>,
    high_pulses: &mut i64,
    low_pulses: &mut i64,
    cycle_length: &mut HashMap<String, u64>,
    seen: &mut HashMap<String, u64>,
    conjunction_name: &str,
) {
    let dests = match broadcast_node {
        Module::Broadcaster(ref dests) => dests,
        _ => unreachable!(),
    };
    for dest in dests.iter() {
        *low_pulses += 1;
        queue.push_back((dest.clone(), Pulse::Low, "broadcaster".to_string())); //Send low pulse to all
    }

    while let Some((name, pulse, source)) = queue.pop_front() {
        let modu = modules.get_mut(&name);
        if modu.is_none() {
            continue;
        }

        //Are we a conjunction_feeder?
        if pulse == Pulse::High && conjunction_name == name {
            //we only care about high pulses
            if let Some(existing_cycle) = cycle_length.get(&source) {
                //We are a conjunction feeder, so we need to check if we've seen this pulse before
                if let Some(last_seen_cycle) = seen.get(&source) {
                    let cycle_diff = cycles - last_seen_cycle;
                    if cycle_diff != *existing_cycle {
                        cycle_length.insert(source.clone(), cycle_diff);
                    }
                    //Should we check how many times we have cycled? Or just that we have? Could there be an unstable cycle?
                }
                seen.insert(source.clone(), cycles);
            }
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
                            Pulse::High => *high_pulses += 1,
                            Pulse::Low => *low_pulses += 1,
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
                        Pulse::High => *high_pulses += 1,
                        Pulse::Low => *low_pulses += 1,
                    };
                    queue.push_back((dest.clone(), pulse_to_send, name.clone()));
                }
            }
        }
    }
}

#[aoc(day20, part2)]
pub fn part2(input: &InputType) -> OutputType {
    //We have to assume that these cycle, find the conjunction that attachs to the rx node.
    // It will be low when all of it's inputs are high, thus, we can find the cycle length for each of the second to last nodes inputs and then find the least common multiple of them.
    //Queue is: destination_name, pulse_type, and source_name

    let mut modules = input.clone();

    //Find the destination that has rx
    let feed_rx = modules
        .iter()
        .find(|(_, modu)| {
            //this should be a single node, and in this case, it's one that is also a Conjunction
            modu.dests().contains(&"rx".to_string())
        })
        .unwrap();

    let feed_rx_name = feed_rx.0.clone();

    let mut cycle_length = input
        .iter()
        .filter(|(_, modu)| modu.dests().contains(&feed_rx.0))
        .map(|(name, _)| (name.clone(), 0))
        .collect::<HashMap<String, u64>>();

    //Now that we have all the conjunction feeders, let's detect cycles.
    // Go through, and whenever we see one of the conjunction feeders get a high signal, mark that as the length
    // If you see it again n times (via the seen queue) you know the cycle length is the one we saw

    //I had to make a LOT of assumptions here, namely that there would be a cycle, and it would be
    //a stable cycle. I only checked if it cycled a single time, which is pretty ballsy, if that
    //didn't work, I would have had to have tried a few times to make sure that the cycle was
    //stable, but the first cycle that appeared just happened to be correct, so I went with it.

    let mut seen: HashMap<String, u64> = HashMap::new();

    let mut queue = VecDeque::new();
    //Queue is: destination_name, pulse_type, and source_name

    let mut low_pulses = 0; //From the button press
    let mut high_pulses = 0;

    let broadcast_node = modules.get("broadcaster").unwrap().clone();

    let mut cycles: u64 = 0;

    loop {
        propogate(
            cycles,
            &broadcast_node,
            &mut modules,
            &mut queue,
            &mut high_pulses,
            &mut low_pulses,
            &mut cycle_length,
            &mut seen,
            &feed_rx_name,
        );
        cycles += 1;
        //Check if all the feeders have a non zero cycle length
        if cycle_length.values().all(|v| *v != 0) {
            break;
        }
    }

    let lcm = cycle_length.iter().map(|(_, v)| *v).fold(1, |acc, x| {
        let gcd = gcd(acc, x);
        acc * x / gcd
    });

    lcm
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
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
