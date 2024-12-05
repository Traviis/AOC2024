type InputType = Vec<&str>;
type OutputType = u64;

#[aoc_generator(dayX)]
fn dayX_parse(input: &str) -> InputType {
    todo!();
}

#[aoc(dayX, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(dayX, part2)]
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
    fn dayX_part1() {
        assert_eq!(part1(&dayX_parse(get_test_input())), 0);
    }

    #[test]
    fn dayX_part2() {
        assert_eq!(part2(&dayX_parse(get_test_input())), 0);
    }
}
