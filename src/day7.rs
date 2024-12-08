type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day7)]
fn day7_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day7, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day7, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        todo!();
    }

    #[test]
    fn day7_part1() {
        assert_eq!(part1(&day7_parse(get_test_input())), 0);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(&day7_parse(get_test_input())), 0);
    }
}
