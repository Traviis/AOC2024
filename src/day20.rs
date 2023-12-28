
type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(day20)]
fn day20_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(day20, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day20, part2)]
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
    fn day20_part1() {
        assert_eq!(part1(&day20_parse(get_test_input())), 11687500);
    }

    #[test]
    fn day20_part2() {
        assert_eq!(part2(&day20_parse(get_test_input())), 0);
    }
}
