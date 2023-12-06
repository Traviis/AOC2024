
type InputType = Vec<Race>;
type OutputType = u64;

pub struct Race {
    time: u64,
    distance: u64,
}
#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {
    let mut lines = input.lines();

    //Time:      7  15   30
    //Distance:  9  40  200"
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    #[cfg(test)]
    {
        println!("times: {:?}", times);
        println!("distances: {:?}", distances);
    }


}

#[aoc(day6, part1)]
pub fn part1(input: &InputType) -> OutputType {
    todo!();
}

#[aoc(day6, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Time:      7  15   30
            Distance:  9  40  200"
    }

    #[test]
    fn day6_part1() {
        assert_eq!(part1(&day6_parse(get_test_input())), 288);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(&day6_parse(get_test_input())), 0);
    }
}
