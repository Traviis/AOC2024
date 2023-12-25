use std::collections::HashMap;

use regex::Regex;

type InputType = (HashMap::<String,Workflow>, Vec<Object>);
type OutputType = u64;


#[derive(Debug)]
pub struct Object {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Attr {
    X,
    M,
    A,
    S,
    None // Default
}

#[derive(Debug,PartialEq,Eq)]
pub enum Op {
    Lt,
    Gt,
    None //Default
}

#[derive(Debug)]
pub struct Rule {
    attr: Attr,
    op: Op,
    val: i64,
    dest: String,
}

type Workflow = Vec<Rule>;

#[aoc_generator(day19)]
fn day19_parse(input: &str) -> InputType {
    let mut lines = input.lines();
    let mut workflows = HashMap::<String,Workflow>::new();
    let mut objects = Vec::<Object>::new();


    let flow_regex = Regex::new(r"([a-z]+)\{([A-Za-z0-9,<>:]+)\}").unwrap();
    let rule_regex = Regex::new(r"([a-z]+)([<>])([0-9]+)").unwrap();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut workflow = Vec::<Rule>::new();
        let caps = flow_regex.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();

        let rules = caps.get(2).unwrap().as_str().split(',');
        //a<2006:qkq,m>2090:A,rfg
        for rule in rules {
            //a<2006:qkq,
            //m>2090:A,
            //rfg
            let mut parts = rule.split(':');
            let part_count = parts.clone().count(); //lol
            if part_count == 1 {
                // Default
                let dest = parts.next().unwrap();
                workflow.push(Rule{attr: Attr::None, op: Op::None, val: 0, dest: dest.to_string()});
            } else if part_count == 2 {

                let cond = parts.next().unwrap();
                let dest = parts.next().unwrap();
                let caps = rule_regex.captures(cond).unwrap();
                let attr = match caps.get(1).unwrap().as_str() {
                    "x" => Attr::X,
                    "m" => Attr::M,
                    "a" => Attr::A,
                    "s" => Attr::S,
                    _ => panic!()
                };
                let op = match caps.get(2).unwrap().as_str() {
                    "<" => Op::Lt,
                    ">" => Op::Gt,
                    _ => panic!()
                };
                let val = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                // a<2006:qkq
                workflow.push(Rule{attr, op, val, dest: dest.to_string()});
            } else {
                panic!();
            }
        }

        workflows.insert(name.to_string(), workflow);
    }

    let obj_regex = Regex::new(r"x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)").unwrap();
    // {x=787,m=2655,a=1222,s=2876}

    while let Some(line) = lines.next() {
        let caps = obj_regex.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let m = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let a = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let s = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        objects.push(Object{x, m, a, s});
    }

    (workflows, objects)
}

#[aoc(day19, part1)]
pub fn part1(input: &InputType) -> OutputType {

    let (workflows, objects) = input;


    objects.iter().filter(|obj| {
        let mut current_workflow = workflows.get("in").unwrap();
        let mut current_idx = 0;

        loop {
            let rule = &current_workflow[current_idx];
            if rule.attr == Attr::None {
                if rule.dest == "R" {
                    return false;
                }
                if rule.dest == "A" {
                    return true;
                }
                current_workflow = workflows.get(&rule.dest).unwrap(); //Default
                current_idx = 0;
            } else {
                //Check the condition of the rules and see if we go to the next rule or next destination
                let val = match rule.attr {
                    Attr::X => obj.x,
                    Attr::M => obj.m,
                    Attr::A => obj.a,
                    Attr::S => obj.s,
                    _ => unreachable!()
                };
                 if (rule.op == Op::Lt && val < rule.val) || (rule.op == Op::Gt && val > rule.val) {
                     if rule.dest == "R" {
                        return false;
                     }
                     if rule.dest == "A" {
                        return true;
                     }
                    current_workflow = workflows.get(&rule.dest).unwrap(); //Default
                    current_idx = 0;
                } else {
                    current_idx += 1;
                };
            }
        }
    })
    .map(|item| item.x + item.m + item.a + item.s)
    .sum::<i64>() as u64

}

struct ViableRange {
    min_x: i64,
    max_x: i64,
    min_m: i64,
    max_m: i64,
    min_a: i64,
    max_a: i64,
    min_s: i64,
    max_s: i64,
}


#[aoc(day19, part2)]
pub fn part2(input: &InputType) -> OutputType {

    let (workflows, _) = input;

    let mut viable_range = ViableRange{ min_x : 0, max_x: 4000, min_m: 0, max_m: 4000, min_a: 0, max_a: 4000, min_s: 0, max_s: 4000 };

    //We don't care about the objects, we need to determine how many possible combinations will be
    //accepted. I think this can be done by taking the range of values of each and multiplying them
    //together.

    //If we always start at a single rule, then we can search all paths and see if they reach the
    //end in an acceptance path or not. We have to track the possible range for each of the xmas
    //values while we do it.

    //This is a giant tree, so we can keep track of the viable range for that combination of rules as we traverse it.
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
    }

    #[test]
    fn day19_part1() {
        assert_eq!(part1(&day19_parse(get_test_input())), 19114);
    }

    #[test]
    fn day19_part2() {
        assert_eq!(part2(&day19_parse(get_test_input())), 167409079868000);
    }
}
