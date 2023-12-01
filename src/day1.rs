type InputType = Vec<String>;
type OutputType = u64;

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref DIGIT_MAP : HashMap<&'static str, u64> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m.insert("ten", 10); //needed?
        m
    };
}

#[aoc_generator(day1)]
fn day1_parse(input: &str) -> InputType {
    input.lines().map(|line| line.to_string()).collect()
}

fn filter_digits(input: &str) -> String {
    input.chars().filter(|c| c.is_digit(10)).collect::<String>()
}

//Assumes that the input is a string of digits only
fn get_first_and_last_digit(input: &str) -> (char, char) {
    let mut num_iter = input.chars();

    //We can use 0 here, since we are in the end only caring about the sums, and if no number
    //exists, just use 0 as an identity for that
    let first_digit = num_iter.next().unwrap_or('0');
    let last_digit = num_iter.last().unwrap_or(first_digit); //if only a single digit just use the first, which waqs the first and last
    (first_digit, last_digit)
}

#[aoc(day1, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|line| {
            let numbers = filter_digits(line);
            let (first_digit, last_digit) = get_first_and_last_digit(&numbers);

            let val = (first_digit.to_string() + &last_digit.to_string())
                .parse::<u64>()
                .unwrap();

            #[cfg(test)]
            println!("{} {} {}", first_digit, last_digit, val);

            val
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &InputType) -> OutputType {
    //Just preprocess this and replace the numbers...
    // Letters can overlap, but we can "cheat" and just replace the first and the last one we find.
    // Iterate from the front, and find the first word, and also iterate from the back, and find the first word.
    part1(
        &input
            .iter()
            .map(|line| {
                let mut line = line.clone();

                let mut current_chunk = String::new();

                let line_clone = line.clone();
                let mut chars = line_clone.chars();

                while let Some(c) = chars.next() {
                    current_chunk.push(c);
                    for (word, val) in DIGIT_MAP.iter() {
                        if let Some(_) = current_chunk.find(word) {
                            line = line.replace(word, &val.to_string());
                        }
                    }
                    if c.is_digit(10) {
                        //If we encounter a digit before a number, we can proceed
                        break;
                    }
                }

                current_chunk.clear();

                //Found the first, now let's go from the reverse and find the last "number", be it
                //digit or string
                let line_clone = line.clone();
                let mut chars = line_clone.chars().rev();

                while let Some(c) = chars.next() {
                    current_chunk.push(c);
                    let reversed_chunk = current_chunk.chars().rev().collect::<String>();
                    let mut found = false;
                    for (word, val) in DIGIT_MAP.iter() {
                        if let Some(_) = reversed_chunk.find(word) {
                            let orig_index = line.len() - word.len(); //Find the index of the word in the original string
                                                                      // Doing this replace range makes sure that you get the correct word
                                                                      // you're suppose to be replacing, as oneone would replace the first
                                                                      // if we did it with .replace()
                            line.replace_range(
                                orig_index..orig_index + word.len(),
                                &val.to_string(),
                            );
                            found = true;
                            break;
                        }
                    }
                    if c.is_digit(10) || found {
                        //If we encounter a digit before a number or we found something to replace, bail out early
                        break;
                    }
                }

                #[cfg(test)]
                {
                    let digits = filter_digits(&line);
                    let (first, last) = get_first_and_last_digit(&digits);
                    println!("{} -> {} -> {}{}", line, digits, first, last);
                }

                line
            })
            .collect::<Vec<String>>(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    }

    fn get_test2_input() -> &'static str {
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 142);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(&day1_parse(get_test2_input())), 281);
    }
}
