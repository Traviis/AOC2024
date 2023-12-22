type InputType = Vec<(Vec<Spring>, Vec<i64>)>;
type OutputType = u64;

pub enum Spring {
    Functional, // .
    Damaged,    // #
    Unknown,    // ?
}

#[aoc_generator(day12)]
fn day12_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let mut sp = line.split(" ");
            let springs = sp
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Functional,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                })
                .collect::<Vec<Spring>>();
            let nums = sp
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            (springs, nums)
        })
        .collect::<Vec<_>>()
}

fn recurse(springs: Vec<Spring>, nums: Vec<i64>, num_in_group: i64) -> u64 {
    if springs.is_empty() {
        return 0;
    }
    let mut solutions = 0;

    solutions
}

#[aoc(day12, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|(springs, nums)| recurse(*springs.clone(), nums.clone(), 0))
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(_input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }

    #[test]
    fn day12_part1() {
        assert_eq!(part1(&day12_parse(get_test_input())), 21);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(&day12_parse(get_test_input())), 0);
    }
}
