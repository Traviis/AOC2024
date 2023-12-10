use regex::Regex;
use std::collections::HashMap;

type InputType = Map;
type OutputType = u64;

pub enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

pub struct Map {
    instructions: Vec<Instruction>,
    node_map: HashMap<String, Node>,
}

#[aoc_generator(day8)]
fn day8_parse(input: &str) -> InputType {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<Instruction>>();

    lines.next(); //skip blank line

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let node_map = lines
        .map(|l| {
            //AAA = (BBB, CCC)

            let caps = re.captures(l).unwrap();
            let name = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();

            (name, Node { left, right })
        })
        .collect::<HashMap<String, Node>>();

    Map {
        instructions,
        node_map,
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut instructions = input.instructions.iter().cycle();

    let mut current_node = input.node_map.get("AAA").unwrap();
    #[allow(unused_assignments)]
    let mut current_node_name = "AAA";
    let mut steps = 0;

    loop {
        match instructions.next().unwrap() {
            Instruction::Left => current_node_name = &current_node.left,
            Instruction::Right => current_node_name = &current_node.right,
        }

        current_node = input.node_map.get(current_node_name).unwrap();
        steps += 1;
        if current_node_name == "ZZZ" {
            break;
        }
    }

    steps
}

#[aoc(day8, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut instructions = input.instructions.iter().cycle();

    let mut current_node_names = input
        .node_map
        .iter()
        .filter_map(|(name, _)| {
            if name.ends_with("A") {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let orig_node_names = current_node_names.clone();
    let mut orig_node_cycle_length = current_node_names
        .iter()
        .map(|n| (n.clone(), 0))
        .collect::<HashMap<String, u64>>();

    //These cycle, thus, we can find the initial cycle length (of each A node to its Z node) and
    //then find the least common multiple of each

    let mut steps = 0;

    #[cfg(test)]
    {
        println!("{}: {:?}", steps, current_node_names);
    }

    loop {
        let instruction = instructions.next().unwrap();
        for (idx, current_node_name) in current_node_names.clone().iter().enumerate() {
            let current_node = input.node_map.get(current_node_name).unwrap();
            let new_node_name = match instruction {
                Instruction::Left => &current_node.left,
                Instruction::Right => &current_node.right,
            };
            #[cfg(test)]
            {
                println!("{}: {} -> {}", steps, current_node_name, new_node_name);
                println!("{:?}", current_node);
            }
            current_node_names[idx] = new_node_name.clone();
            if new_node_name.ends_with(&"Z") {
                let orig_name = orig_node_names[idx].clone();
                if orig_node_cycle_length.get(&orig_name).unwrap() == &0 {
                    orig_node_cycle_length.insert(orig_name, steps + 1);
                }
            }
        }

        #[cfg(test)]
        {
            println!("{}: {:?}", steps, current_node_names);
        }

        steps += 1;

        if orig_node_cycle_length.iter().all(|(_, v)| *v != 0) {
            break;
        }
    }
    #[cfg(test)]
    {
        println!("{:?}", orig_node_cycle_length);
    }

    //Find the least common multiple of each cycle length
    // lcm(a,b,c) = lcm(a,lcm(b,c))
    let lens = orig_node_cycle_length
        .iter()
        .map(|(_, v)| *v)
        .collect::<Vec<u64>>();
    let lcm = lens.iter().fold(lens[0], |acc, &x| {
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
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    }

    fn get_part2_test_input() -> &'static str {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    }

    #[test]
    fn day8_part1() {
        assert_eq!(part1(&day8_parse(get_test_input())), 6);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(&day8_parse(get_part2_test_input())), 6);
    }
}
