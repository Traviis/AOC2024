use std::collections::BTreeMap;

type InputType = (Vec<i32>, Vec<i32>);
type OutputType = i32;

#[aoc_generator(day1)]
fn day1_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

#[aoc(day1, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut left = input.0.clone();
    let mut right = input.1.clone();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>()
}

#[aoc(day1, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let left = &input.0;
    //Convert to a count of the number of times each element appears
    let right = &input.1;

    let dup_list = right.iter().fold(BTreeMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    left.iter()
        .map(|l| dup_list.get(l).unwrap_or(&0) * l)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "3   4
4   3
2   5
1   3
3   9
3   3"
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 11);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(&day1_parse(get_test_input())), 31);
    }
}
