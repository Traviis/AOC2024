use memoize::memoize;
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

type InputType = Vec<Stone>;
type OutputType = u64;

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Copy, Debug)]
pub struct Stone(i64);

impl Deref for Stone {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[aoc_generator(day11)]
fn day11_parse(input: &str) -> InputType {
    input
        .split(" ")
        .map(|x| Stone(x.parse::<i64>().unwrap()))
        .collect()
}

//Turns out that memoizing this isn't strictly needed, but it shaves off 6ms from the part 2 run
//time so why not
#[memoize]
fn apply_rules(stone: Stone) -> (Option<Stone>, Option<Stone>) {
    let val = stone.0;
    // If 0, replace with 1
    if val == 0 {
        return (Some(Stone(1)), None);
    }
    //If even number of digits, split in half (stringwise), and return the two halves (dropping leading 0s)
    if val.to_string().len() % 2 == 0 {
        let s = val.to_string();
        let half = s.len() / 2;
        let (left, right) = s.split_at(half);
        return (
            Some(Stone(left.parse().unwrap())),
            Some(Stone(right.parse().unwrap())),
        );
    }

    // Otherwise, multiple stone by 2024
    (Some(Stone(val * 2024)), None)
}

fn solve(input: &InputType, iters: u64) -> u64 {
    let mut stones = input
        .clone()
        .iter()
        .map(|stone| (*stone, 1))
        .collect::<BTreeMap<Stone, u64>>();

    for _ in 0..iters {
        let mut new_stones = BTreeMap::new();
        for (stone, count) in stones.iter() {
            let (s1, s2) = apply_rules(*stone);
            if let Some(s1) = s1 {
                *new_stones.entry(s1).or_insert(0) += *count;
            }
            if let Some(s2) = s2 {
                *new_stones.entry(s2).or_insert(0) += *count;
            }
        }
        //println!("New stones: {:?}", new_stones);
        stones = new_stones;
    }

    stones.values().sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &InputType) -> OutputType {
    // All the rules are applied at THE SAME TIME, so copy the current vec, apply all the rules.
    solve(input, 25)

    //Naive
    //for _ in 0..25 {
    //    let mut new_stones = Vec::new();
    //    for stone in stones.iter() {
    //        new_stones.extend(apply_rules(stone.clone()));
    //    }
    //    stones = new_stones;
    //}

    //stones.len() as u64
}

#[aoc(day11, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solve(input, 75)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "125 17"
    }

    #[test]
    fn day11_part1() {
        assert_eq!(part1(&day11_parse(get_test_input())), 55312);
    }
}
