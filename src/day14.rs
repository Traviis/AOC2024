use std::collections::HashMap;

type InputType = HashMap<(i64, i64), Rock>;
type OutputType = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rock {
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[aoc_generator(day14)]
fn day14_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let r = match c {
                    'O' => Some(Rock::Round),
                    '#' => Some(Rock::Square),
                    _ => None,
                };

                if r.is_none() {
                    return None;
                }
                Some(((x as i64, y as i64), r.unwrap()))
            })
        })
        .collect()
}

fn slide_direction(max_x: i64, max_y: i64, map: &InputType, direction: Direction) -> InputType {
    //Just slide by 1, if you want to check if all of the rocks have moved, you can check if the
    //previous and this output are the same
    // Just implement North for now

    let mut new_map = map.clone();
    match direction {
        Direction::North => {
            //Slide the circle rocks up if they can be slid (there is no square rock, or circle rock above them
            for y in 1..=max_y {
                // Move down
                for x in 0..=max_x {
                    let current = map.get(&(x, y));
                    let above = map.get(&(x, y - 1));
                    if current == Some(&Rock::Round) && above == None {
                        new_map.insert((x, y - 1), Rock::Round);
                        new_map.remove(&(x, y));
                    }
                }
            }
            new_map
        }
        _ => unreachable!(),
    }
}

fn determine_weight(map: &InputType, direction: Direction) -> i64 {
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    map.iter()
        .filter(|(_, r)| **r == Rock::Round)
        .map(|((x, y), _)| {
            match direction {
                Direction::North => max_y + 1 - y,
                //We only ever check the weight of the north support beams
                _ => unreachable!(),
            }
        })
        .sum()
}

fn dump_map(map: &InputType) {
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = match map.get(&(x, y)) {
                Some(Rock::Round) => 'O',
                Some(Rock::Square) => '#',
                None => '.',
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn slide_until_no_movement(max_x: i64, max_y: i64, input: &InputType, dir: Direction) -> InputType {
    let mut current_map = input.clone();
    let mut next = slide_direction(max_x, max_y, &current_map, dir);
    while next != current_map {
        current_map = next;
        next = slide_direction(max_x, max_y, &current_map, dir);
    }
    current_map
}

fn slide_until_no_movement_elided(
    max_x: i64,
    max_y: i64,
    input: &InputType,
    dir: Direction,
) -> InputType {
    //Never did end up having to slide by 1, so just doing all the calculations at once would be
    //more effecient here

    //Find the end of the direction (determined by a square rock or a wall)
    //Then update all positions in between based on how many circlular rocks there are in that row
    //or column
}

#[aoc(day14, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();
    let slid_map = slide_until_no_movement(max_x, max_y, input, Direction::North);
    #[cfg(test)]
    dump_map(&slid_map);
    determine_weight(&slid_map, Direction::North) as u64
}

#[aoc(day14, part2)]
pub fn part2(input: &InputType) -> OutputType {
    part2_by_cycle(input, 1000000000)
}
pub fn part2_by_cycle(input: &InputType, cycles: usize) -> OutputType {
    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();
    //let CYCLES = 1000000000;
    //Cycle is north, west, south ,east
    let mut last_map = input.clone();
    for cycle in 0..cycles {
        let slid_north = slide_until_no_movement_elided(max_x, max_y, &last_map, Direction::North);
        // #[cfg(test)]
        // dump_map(&slid_north);
        let slid_west = slide_until_no_movement_elided(max_x, max_y, &slid_north, Direction::West);
        // #[cfg(test)]
        // dump_map(&slid_west);
        let slid_south = slide_until_no_movement_elided(max_x, max_y, &slid_west, Direction::South);
        // #[cfg(test)]
        // dump_map(&slid_south);
        last_map = slide_until_no_movement_elided(max_x, max_y, &slid_south, Direction::East);
        #[cfg(test)]
        dump_map(&last_map);
    }

    determine_weight(&last_map, Direction::North) as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    }

    #[test]
    fn day14_part1() {
        assert_eq!(part1(&day14_parse(get_test_input())), 136);
    }

    #[test]
    fn day14_part2() {
        assert_eq!(part2(&day14_parse(get_test_input())), 64);
    }

    #[test]
    fn day14_part2_simple() {
        part2_by_cycle(&day14_parse(get_test_input()), 3);
    }
}
