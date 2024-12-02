type InputType = Vec<Vec<i64>>;
type OutputType = u64;

#[aoc_generator(day2)]
fn day1_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_safe(row: &[i64]) -> bool {
    // levels are all increasing or all descreasing
    // any two adjacent levels differ by at least one, at most 3
    let mut increasing = None;
    let mut last_num = row[0];

    for num in row.iter().skip(1) {
        // Must be increasing or decreasing, set the initial direction
        if increasing.is_none() && last_num < *num {
            increasing = Some(true);
        } else if increasing.is_none() && last_num > *num {
            increasing = Some(false);
        }

        if (increasing.is_some() && increasing.unwrap() && last_num > *num)
            || increasing.is_some() && !increasing.unwrap() && last_num < *num
        {
            return false;
        }

        let diff = (last_num - num).abs();

        if !(1..=3).contains(&diff) {
            return false;
        }

        last_num = *num;
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.iter().filter(|row| is_safe(row)).count() as u64
}

// This isn't a very satisfying solution, it would be more interesting to have the is_safe function
// be able to determine if a row is safe with a single pass, however, that becomes really difficult
// particularly when the thing you removed is the second or first item, which would make my
// detection logic break. In this case, the input lists are all small, so it's not a big deal to
// just blast your way thourgh by removing the value each time and checking if it's safe.
#[aoc(day2, part2)]
pub fn part2(input: &InputType) -> OutputType {
    //Input list is small, let's brute force it
    input
        .iter()
        .filter(|report| {
            let mut report = (**report).clone();
            is_safe(&report)
                || (0..report.len()).any(|i| {
                    //If it's not safe, then go through each one and remove one, then check if it's safe
                    let removed_val = report[i];
                    report.remove(i);
                    if is_safe(&report) {
                        true
                    } else {
                        report.insert(i, removed_val);
                        false
                    }
                    //I was doing a clone here on the report for every iteration of the insertion
                    //check, however, benchmarking shows that a single clone at the start is ~17%
                    //faster, even with insertions and removals. The code is harder to read, but might
                    //as well play a little golf.
                    // The original (in the any function):
                    // let mut report_copy = (*report).clone();
                    // report_copy.remove(i);
                    // is_safe(&report_copy)
                })
        })
        .count() as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    }

    #[test]
    fn day2_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 2);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(part2(&day1_parse(get_test_input())), 4);
    }
}
