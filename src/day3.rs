use std::collections::{HashMap, HashSet};

pub enum Symbol {
    Number(u8),
    Plus,      // +
    Star,      // *
    Dollar,    // $
    Hash,      // #
    Equal,     // =
    Slash,     // /
    Ampersand, // &
    Minus,     // -
    Percent,   // %
    At,        // @
}

type InputType = HashMap<(i32, i32), Symbol>;
type OutputType = u64;

#[aoc_generator(day3)]
fn day3_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let symbol = match c {
                    //Not sure if we will need all of these, but I have a feeling part 2 will do special logic with different symbols
                    '*' => Some(Symbol::Star),
                    '#' => Some(Symbol::Hash),
                    '$' => Some(Symbol::Dollar),
                    '+' => Some(Symbol::Plus),
                    '=' => Some(Symbol::Equal),
                    '/' => Some(Symbol::Slash),
                    '&' => Some(Symbol::Ampersand),
                    '-' => Some(Symbol::Minus),
                    '%' => Some(Symbol::Percent),
                    '@' => Some(Symbol::At),
                    '0'..='9' => Some(Symbol::Number(c.to_digit(10).unwrap() as u8)),
                    '.' => None,
                    _ => panic!("Invalid symbol {}", c),
                };
                ((x as i32, y as i32), symbol)
            })
        })
        .filter(|(_, symbol)| symbol.is_some())
        .map(|((x, y), symbol)| ((x, y), symbol.unwrap()))
        .collect()
}

fn find_number_start_index(input: &InputType, x: i32, y: i32) -> (i32, i32) {
    let mut current_x = x;

    while let Some(Symbol::Number(_)) = input.get(&(current_x, y)) {
        current_x -= 1;
    }
    (current_x + 1, y)
}

fn find_number_from_start(input: &InputType, x: i32, y: i32) -> u64 {
    let mut current_x = x;
    let mut number_str = String::new();

    while let Some(Symbol::Number(n)) = input.get(&(current_x, y)) {
        number_str.push_str(&n.to_string());
        current_x += 1;
    }
    number_str.parse::<u64>().unwrap()
}

#[aoc(day3, part1)]
pub fn part1(input: &InputType) -> OutputType {
    // Go through the x and y, if you find a number, determine if it's adjacent to a Symbol, if so, mark it as valid (and find its sibling values)
    let max_x = input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = input.keys().map(|(_, y)| y).max().unwrap();

    let mut valid_number_starts: HashSet<(i32, i32)> = HashSet::new();

    for x in 0..=*max_x {
        for y in 0..=*max_y {
            if let Some(Symbol::Number(_)) = input.get(&(x, y)) {
                // Let's be lazy and just check ALL numbers, if this is slow, we should be able to
                // determine if the number is already in the valid_number_starts list BEFORE we
                // iterate, but eh.

                // Check if it's adjacent to a symbol
                for x_offset in -1..=1 {
                    for y_offset in -1..=1 {
                        if x_offset == 0 && y_offset == 0 {
                            continue; //skip yourself
                        }
                        let adjacent_symbol = input.get(&(x + x_offset, y + y_offset));
                        if adjacent_symbol.is_some() {
                            if let Some(Symbol::Number(_)) = adjacent_symbol {
                                continue; //skip numbers
                            }
                            //We found an adjacent symbol, fiund the start of the number
                            valid_number_starts.insert(find_number_start_index(input, x, y));
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    println!("Valid number starts: {:?}", valid_number_starts);

    //Iterate through the valid_number_starts, and find the end of the number
    let valid_numbers = valid_number_starts
        .iter()
        .map(|(x, y)| find_number_from_start(input, *x, *y))
        .collect::<Vec<_>>();

    #[cfg(test)]
    println!("Valid numbers: {:?}", valid_numbers);

    valid_numbers.iter().sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &InputType) -> OutputType {
    // In this case, it's easier to identify all of the "gears" (*) and then check if it's adjacent to two number groups

    let gears = input
        .iter()
        .filter(|(_, symbol)| {
            if let Symbol::Star = symbol {
                true
            } else {
                false
            }
        })
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();

    #[cfg(test)]
    println!("Gears: {:?}", gears);

    let mut gear_mults = Vec::new();

    //Now that we have all the gears, check if they are adjacent to EXACTLY two number groups
    //Find All adjacent groups, then ensure that there are only 2,
    for (gear_x, gear_y) in gears {
        let mut adjacent_group_starts = HashSet::new();
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue; //skip yourself
                }
                let adjacent_symbol = input.get(&(gear_x + x_offset, gear_y + y_offset));
                if let Some(Symbol::Number(_)) = adjacent_symbol {
                    adjacent_group_starts.insert(find_number_start_index(
                        input,
                        gear_x + x_offset,
                        gear_y + y_offset,
                    ));
                }
            }
        }

        if adjacent_group_starts.len() != 2 {
            continue;
        }

        gear_mults.push(
            adjacent_group_starts
                .iter()
                .map(|(x, y)| find_number_from_start(input, *x, *y))
                .product::<u64>(),
        );
    }

    gear_mults.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    }

    #[test]
    fn day3_part1() {
        assert_eq!(part1(&day3_parse(get_test_input())), 4361);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(&day3_parse(get_test_input())), 467835);
    }
}
