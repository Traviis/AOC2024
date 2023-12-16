#[allow(unused_imports)]
use colored::Colorize as _;

use std::collections::HashMap;

type InputType = Vec<HashMap<(i64, i64), Land>>;
type OutputType = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Land {
    Ash,  // .
    Rock, // #
}

#[aoc_generator(day13)]
fn day13_parse(input: &str) -> InputType {
    let mut line_iter = input.split("\n");

    let mut output = Vec::new();

    let mut current_map = HashMap::new();

    let mut current_y = 0;

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            output.push(current_map);
            current_map = HashMap::new();
            current_y = 0;
        } else {
            line.trim().chars().enumerate().for_each(|(x, c)| {
                //Could do sparse maps, but let's make it easy
                let thing = match c {
                    '.' => Land::Ash,
                    '#' => Land::Rock,
                    _ => panic!("Unknown character ({})", c),
                };
                current_map.insert((x as i64, current_y), thing);
            });
            current_y += 1;
        }
    }
    output.push(current_map);
    output
}

fn is_reflection(
    max_x: i64,
    max_y: i64,
    map: &HashMap<(i64, i64), Land>,
    row: bool,
    p_1: i64,
    p_2: i64,
) -> i64 {
    let mut possible_mismatches = 0;
    if row {
        for (left, right) in (0..=p_1).rev().zip(p_2..=max_x) {
            for y in 0..=max_y {
                if map[&(left, y)] != map[&(right, y)] {
                    possible_mismatches += 1;
                }
            }
        }
    } else {
        for (top, bottom) in (0..=p_1).rev().zip(p_2..=max_y) {
            for x in 0..=max_x {
                if map[&(x, top)] != map[&(x, bottom)] {
                    possible_mismatches += 1;
                }
            }
        }
    }

    possible_mismatches
}

fn dump_map(map: &HashMap<(i64, i64), Land>, highlight: &Vec<(i64, i64)>) {
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = match map.get(&(x, y)) {
                Some(Land::Ash) => '.',
                Some(Land::Rock) => '#',
                None => ' ',
            };
            let v = if highlight.contains(&(x, y)) {
                c.to_string().red()
            } else {
                c.to_string().normal()
            };
            print!("{}", v);
        }
        println!();
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &InputType) -> OutputType {
    solver(input, 0)
}
pub fn solver(input: &InputType, expected_diff: i64) -> OutputType {
    input
        .iter()
        .enumerate()
        .map(|(_map_idx, map)| {
            let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
            let max_y = *map.keys().map(|(_, y)| y).max().unwrap();

            let mut running_total = 0;

            //Check rows first
            for x in 0..max_x {
                if is_reflection(max_x, max_y, map, true, x, x + 1) == expected_diff {
                    #[cfg(test)]
                    {
                        println!(
                            "map_idx: {} Found reflection at x = {}",
                            _map_idx,
                            (x + x + 1) as f64 / 2.0
                        );
                    }
                    running_total += x + 1;
                    break;
                }
            }

            //Check columns
            for y in 0..max_y {
                if is_reflection(max_x, max_y, map, false, y, y + 1) == expected_diff {
                    #[cfg(test)]
                    {
                        println!(
                            "map_idx: {} Found reflection at y = {}",
                            _map_idx,
                            (y + y + 1) as f64 / 2.0
                        );
                    }

                    running_total += (y + 1) * 100;
                    break;
                }
            }

            running_total
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solver(input, 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "#.##..##.
                ..#.##.#.
##......#
##......#
                ..#.##.#.
                ..##..##.
#.#.##.#.

#...##..#
#....#..#
                ..##..###
#####.##.
#####.##.
                ..##..###
#....#..#"
    }

    #[test]
    fn day13_part1() {
        assert_eq!(part1(&day13_parse(get_test_input())), 405);
    }

    #[test]
    fn day13_part2() {
        assert_eq!(part2(&day13_parse(get_test_input())), 400);
    }
}
