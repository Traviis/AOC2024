type InputType = Vec<(Vec<Spring>, Vec<i64>)>;
type OutputType = u64;

enum Spring {
    Functional, // .
    Damaged,    // #
    Unknown,    // ?
}

#[aoc_generator(day12)]
fn day12_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let sp = line.split(" ");
            let springs = line
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
                .map(|s| s.parse::<i64>().unwrap());

            (spring, nums)
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //I think roughly, you can assume that you take the largest one first, and see where it's
    //possible to fit, but leaving any groups to the left by number of spacess they can fit in....
    // This seems like there is an algorithm here that I don't know.... Maybe some sort of dynamic programming?
    todo!();
}

#[aoc(day12, part2)]
pub fn part2(input: &InputType) -> OutputType {
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
