use std::collections::HashMap;

use regex::Regex;

type InputType = (HashMap<String, Workflow>, Vec<Object>);
type OutputType = u64;

#[derive(Debug)]
pub struct Object {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attr {
    X,
    M,
    A,
    S,
    None, // Default
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Lt,
    Gt,
    None, //Default
}

#[derive(Debug)]
pub struct Rule {
    attr: Attr,
    op: Op,
    val: i64,
    dest: String,
}

impl Rule {
    fn get_lt_gt_ranges(self: &Self, range: &ViableRange) -> (ViableRange, ViableRange) {
        let mut lt_range = range.clone();
        let mut gt_range = range.clone();

        match self.attr {
            Attr::X => {
                match self.op {
                    Op::Lt => {
                        lt_range.max_x = self.val - 1;
                        gt_range.min_x = self.val; //Greater than or equal to
                    }
                    Op::Gt => {
                        gt_range.min_x = self.val + 1;
                        lt_range.max_x = self.val; //Less than or equal to
                    }
                    _ => unreachable!(),
                }
            }
            Attr::M => {
                match self.op {
                    Op::Lt => {
                        lt_range.max_m = self.val - 1;
                        gt_range.min_m = self.val; //Greater than or equal to
                    }
                    Op::Gt => {
                        gt_range.min_m = self.val + 1;
                        lt_range.max_m = self.val; //Less than or equal to
                    }
                    _ => unreachable!(),
                }
            }
            Attr::A => {
                match self.op {
                    Op::Lt => {
                        lt_range.max_a = self.val - 1;
                        gt_range.min_a = self.val; //Greater than or equal to
                    }
                    Op::Gt => {
                        gt_range.min_a = self.val + 1;
                        lt_range.max_a = self.val; //Less than or equal to
                    }
                    _ => unreachable!(),
                }
            }
            Attr::S => {
                match self.op {
                    Op::Lt => {
                        lt_range.max_s = self.val - 1;
                        gt_range.min_s = self.val; //Greater than or equal to
                    }
                    Op::Gt => {
                        gt_range.min_s = self.val + 1;
                        lt_range.max_s = self.val; //Less than or equal to
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }

        (lt_range, gt_range)
    }
}

type Workflow = Vec<Rule>;

#[aoc_generator(day19)]
fn day19_parse(input: &str) -> InputType {
    let mut lines = input.lines();
    let mut workflows = HashMap::<String, Workflow>::new();
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
                workflow.push(Rule {
                    attr: Attr::None,
                    op: Op::None,
                    val: 0,
                    dest: dest.to_string(),
                });
            } else if part_count == 2 {
                let cond = parts.next().unwrap();
                let dest = parts.next().unwrap();
                let caps = rule_regex.captures(cond).unwrap();
                let attr = match caps.get(1).unwrap().as_str() {
                    "x" => Attr::X,
                    "m" => Attr::M,
                    "a" => Attr::A,
                    "s" => Attr::S,
                    _ => panic!(),
                };
                let op = match caps.get(2).unwrap().as_str() {
                    "<" => Op::Lt,
                    ">" => Op::Gt,
                    _ => panic!(),
                };
                let val = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                // a<2006:qkq
                workflow.push(Rule {
                    attr,
                    op,
                    val,
                    dest: dest.to_string(),
                });
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
        objects.push(Object { x, m, a, s });
    }

    (workflows, objects)
}

#[aoc(day19, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let (workflows, objects) = input;

    objects
        .iter()
        .filter(|obj| {
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
                        _ => unreachable!(),
                    };
                    if (rule.op == Op::Lt && val < rule.val)
                        || (rule.op == Op::Gt && val > rule.val)
                    {
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

#[derive(Clone, Debug, Copy,PartialEq,Eq)]
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

    let mut viable_range = ViableRange {
        min_x: 1,
        max_x: 4000,
        min_m: 1,
        max_m: 4000,
        min_a: 1,
        max_a: 4000,
        min_s: 1,
        max_s: 4000,
    };

    //We don't care about the objects, we need to determine how many possible combinations will be
    //accepted. I think this can be done by taking the range of values of each and multiplying them
    //together.

    //If we always start at a single rule, then we can search all paths and see if they reach the
    //end in an acceptance path or not. We have to track the possible range for each of the xmas
    //values while we do it.

    //This is a giant tree, so we can keep track of the viable range for that combination of rules as we traverse it.

    let mut stack = Vec::<(String, usize, ViableRange)>::new();
    let mut good_ranges = Vec::<ViableRange>::new();
    stack.push(("in".to_string(), 0, viable_range));

    while let Some(item) = stack.pop() {
        //Iterate through each step in the workflow, if it ends in A then add to the viable ranges.
        #[cfg(test)]
        println!("{:?}", item);
        let (workflow_name, idx, mut viable_range) = item;
        let workflow = workflows.get(&workflow_name).unwrap();

        //First check the two possibilities of the rule
        let rule = &workflow[idx];
        let mut range = viable_range.clone();

        if rule.attr == Attr::None {
            if rule.dest == "R" {
                //Rejected, throw out this entire branch
                continue;
            }
            if rule.dest == "A" {
                //Accepted range, this is a good branch
                good_ranges.push(range);
                #[cfg(test)]
                println!("Found a good range (first inspection): {:?}", range);
                continue;
            }

            // Default, we need to go to the next workflow
            #[cfg(test)]
            println!(
                "Pushing {:?} to stack (default)",
                (rule.dest.clone(), 0, range)
            );
            stack.push((rule.dest.clone(), 0, range));
        } else {
            //Get the two possible ranges and their appropriate ranges
            //Find all the values that are less than the rule,and send them to the next workflow
            //Find the other part that is more than the rule and send them to the next index
            let (lt_range, gt_range) = rule.get_lt_gt_ranges(&range);

            //These passed the test, so we can proceed to their destination
            let next_rule_range = match rule.op {
                Op::Lt => &lt_range,
                Op::Gt => &gt_range,
                _ => unreachable!(),
            }; 

            //These failed the test, so can move to the next index
            let next_index_range = match rule.op {
                Op::Lt => &gt_range,
                Op::Gt => &lt_range,
                _ => unreachable!(),
            };


            //For the accepted ranges, if their destination is A, go ahead and just accept it
            // If it's not R, then we push it onto the stack to be tested
            // If it's R, we just stop processing on this branch
            if rule.dest == "A" {
                good_ranges.push(*next_rule_range);
                #[cfg(test)]
                println!("Found a good range: {:?}", next_rule_range);
            } else if rule.dest != "R" {
                #[cfg(test)]
                println!(
                    "Pushing {:?} to stack",
                    (rule.dest.clone(), 0, *next_rule_range)
                );
                stack.push((rule.dest.clone(), 0, *next_rule_range));
            }

            //Regardless of the accepted one, we need to push the "failure" branch onto the stack
            //as well to see where it heads off to; we don't have to check for A or R here, becuase
            //we do it at the start of the loop

            #[cfg(test)]
            println!(
                "Pushing next_idx {:?} to stack",
                (workflow_name.clone(), idx + 1, *next_index_range)
            );
            //Always push the next workflow to the stack with the next index
            stack.push((workflow_name.clone(), idx + 1, *next_index_range));
        }
    }

    // Now we have a list of all the possible ranges, we need to find the number of combinations
    // that are possible. We can do this by multiplying the ranges together.
    let mut total = 0;
    for range in good_ranges {
        let x_range = range.max_x - range.min_x + 1;
        let m_range = range.max_m - range.min_m + 1;
        let a_range = range.max_a - range.min_a + 1;
        let s_range = range.max_s - range.min_s + 1;
        total += x_range * m_range * a_range * s_range;
    }
    total as u64
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

    #[test]
    fn day19_test_bisect() {
        let orig_range = ViableRange {
            min_x: 1,
            max_x: 4000,
            min_m: 1,
            max_m: 4000,
            min_a: 1,
            max_a: 4000,
            min_s: 1,
            max_s: 4000,
        };
        let test_rule = Rule {
            attr: Attr::X,
            op: Op::Lt,
            val: 2000,
            dest: "A".to_string(),
        };

        let (lt_range, gt_range) = test_rule.get_lt_gt_ranges(&orig_range);
        assert_eq!(
            lt_range,
            ViableRange {
                min_x: 1,
                max_x: 1999,
                min_m: 1,
                max_m: 4000,
                min_a: 1,
                max_a: 4000,
                min_s: 1,
                max_s: 4000,
            }
        );
        assert_eq!(
            gt_range,
            ViableRange {
                min_x: 2000,
                max_x: 4000,
                min_m: 1,
                max_m: 4000,
                min_a: 1,
                max_a: 4000,
                min_s: 1,
                max_s: 4000,
            }
        );
    }
}
