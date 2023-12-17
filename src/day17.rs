
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day17)]
fn day17_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day17, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day17, part2)]
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
    fn day17_part1() {
        assert_eq!(part1(&day17_parse(get_test_input())), 0);
    }

    #[test]
    fn day17_part2() {
        assert_eq!(part2(&day17_parse(get_test_input())), 0);
    }
}
