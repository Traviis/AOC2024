type InputType = Vec<(Vec<Spring>, Vec<i64>)>;
type OutputType = u64;

use cached::proc_macro::cached;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
            let mut springs = sp
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Functional,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("Invalid spring"),
                })
                .collect::<Vec<Spring>>();
            //Cheat a little bit and stick a functional spring at the end so we can just know when we hit EOL
            //springs.push(Spring::Functional);
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

#[allow(dead_code)]
fn dump_spring_map(springs: &Vec<Spring>, nums: &Vec<i64>) {
    for s in springs.iter() {
        match s {
            Spring::Functional => print!("."),
            Spring::Damaged => print!("#"),
            Spring::Unknown => print!("?"),
        }
    }
    print!(" ");
    for n in nums.iter() {
        print!("{},", n);
    }
    println!();
}

#[cached]
fn recurse(springs: Vec<Spring>, nums: Vec<i64>, num_in_group: i64) -> u64 {
    if springs.is_empty() {
        if nums.len() == 0 && num_in_group == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    let mut solutions = 0;
    let possible = match springs[0] {
        Spring::Functional => vec![Spring::Functional],
        Spring::Damaged => vec![Spring::Damaged],
        Spring::Unknown => vec![Spring::Functional, Spring::Damaged],
    };
    for c in possible.iter() {
        if c == &Spring::Damaged {
            solutions += recurse(springs[1..].to_vec(), nums.clone(), num_in_group + 1);
        } else {
            if num_in_group > 0 {
                if nums.len() > 0 && nums[0] == num_in_group {
                    solutions += recurse(springs[1..].to_vec(), nums[1..].to_vec(), 0);
                }
            } else {
                solutions += recurse(springs[1..].to_vec(), nums.clone(), 0);
            }
        }
    }

    solutions
}

#[aoc(day12, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|(springs, nums)| {
            let mut mod_springs = springs.clone();
            mod_springs.push(Spring::Functional); //Cheat for memoization
            recurse(mod_springs, nums.clone(), 0)
        })
        .sum()
}

fn replace_spring_map(springs: &Vec<Spring>, nums: &Vec<i64>) -> (Vec<Spring>, Vec<i64>) {
    //Replace each set of springs with 5 copies of itself, followed by a ?
    let mut new_springs = vec![];
    for x in 0..5 {
        new_springs.extend(springs.clone());
        if x < 4 {
            new_springs.push(Spring::Unknown);
        }
    }

    //Replace each set of nums with 5 copies of itself
    let mut new_nums = vec![];
    for _ in 0..5 {
        new_nums.extend(nums.clone());
    }

    (new_springs, new_nums)
}

#[aoc(day12, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let modified_map = input
        .iter()
        .map(|(springs, nums)| replace_spring_map(&springs, &nums))
        .collect();

    part1(&modified_map)
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
    fn day12_test_spring_map() {
        let input = day12_parse(get_test_input());
        let (springs, nums) = input[0].clone();
        let (new_springs, new_nums) = replace_spring_map(&springs, &nums);

        let expected_raw = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        let expected = day12_parse(expected_raw);
        dump_spring_map(&new_springs, &new_nums);
        dump_spring_map(expected[0].0.as_ref(), expected[0].1.as_ref());
        assert_eq!(expected[0], (new_springs, new_nums));
    }

    #[test]
    fn day12_simple_test_spring_map() {
        let simple_input = ".# 1";
        let input = day12_parse(simple_input);
        let (springs, nums) = input[0].clone();
        let (new_springs, new_nums) = replace_spring_map(&springs, &nums);

        let expected_raw = ".#?.#?.#?.#?.# 1,1,1,1,1";
        let expected = day12_parse(expected_raw);

        println!("Raw");
        dump_spring_map(springs.as_ref(), nums.as_ref());
        println!("New");
        dump_spring_map(&new_springs, &new_nums);
        println!("Expected");
        dump_spring_map(expected[0].0.as_ref(), expected[0].1.as_ref());
        assert_eq!(expected[0], (new_springs, new_nums));
    }

    #[test]
    fn day12_part1() {
        assert_eq!(part1(&day12_parse(get_test_input())), 21);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(&day12_parse(get_test_input())), 525152);
    }
}
