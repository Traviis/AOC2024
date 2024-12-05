type InputType = String;
type OutputType = u64;

use regex::Regex;

#[aoc_generator(day3)]
fn day3_parse(input: &str) -> InputType {
    input.to_string()
}

#[aoc(day3, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<u64>().unwrap();
            let b = cap[2].parse::<u64>().unwrap();
            a * b
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let re = Regex::new(r"(do\(\)|(don't\(\))|mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();

    let mut enabled = true;
    let mut running_total = 0;

    for cap in re.captures_iter(input) {
        let first_cap = cap.get(0).unwrap().as_str();
        if first_cap == "do()" {
            enabled = true;
        } else if first_cap == "don't()" {
            enabled = false;
        } else if enabled {
            let a = cap[3].parse::<u64>().unwrap();
            let b = cap[4].parse::<u64>().unwrap();
            running_total += a * b;
        }
    }

    running_total
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }
    fn get_test_input_part2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    #[test]
    fn day3_part1() {
        assert_eq!(part1(&day3_parse(get_test_input())), 161);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(&day3_parse(get_test_input_part2())), 48);
    }
}
