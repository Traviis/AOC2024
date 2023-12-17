use std::collections::BTreeMap;
use std::hash::Hash;

type RockMap = BTreeMap<(i64, i64), Rock>;
type InputType = RockMap;
type OutputType = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    map.iter()
        .filter(|(_, r)| **r == Rock::Round)
        .map(|((_, y), _)| {
            match direction {
                Direction::North => max_y + 1 - y,
                //We only ever check the weight of the north support beams
                _ => unreachable!(),
            }
        })
        .sum()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn slide_until_no_movement(max_x: i64, max_y: i64, input: &InputType, dir: Direction) -> InputType {
    let mut current_map = input.clone();
    let mut next = slide_direction(max_x, max_y, &current_map, dir);
    while next != current_map {
        current_map = next;
        next = slide_direction(max_x, max_y, &current_map, dir);
    }
    current_map
}

fn slide_until_no_movement_elided(max_x: i64, max_y: i64, new_map: &mut InputType, dir: Direction) {
    //Never did end up having to slide by 1, so just doing all the calculations at once would be
    //more effecient here

    //Find the end of the direction (determined by a square rock or a wall)
    //Then update all positions in between based on how many circlular rocks there are in that row
    //or column

    match dir {
        Direction::North => {
            for y in 1..=max_y {
                // move up (y is inverted) until we hit a square rock or a wall
                for x in 0..=max_x {
                    let current = new_map.get(&(x, y));
                    // keep moving up until we hit a wall or square rock
                    if current == Some(&Rock::Round) {
                        //find end
                        let mut end = y;
                        // Find the end, this is where the rock will go
                        //First select the rock we are on, then move up until we hit one above.
                        for y2 in (0..=y).rev() {
                            let above = new_map.get(&(x, y2 - 1));
                            if above == Some(&Rock::Square) || above == Some(&Rock::Round) {
                                //If we hit another rock, then we have to stop
                                end = y2;
                                break;
                            }
                            if y2 == 0 {
                                //If we got all the way to the top, then we can move it to end
                                end = 0;
                                break;
                            }
                        }

                        //Move the rock up, if you're not in the same spot
                        if end != y {
                            new_map.insert((x, end), Rock::Round);
                            new_map.remove(&(x, y));
                        }
                    }
                }
            }
        } //North
        Direction::East => {
            for x in (0..=max_x).rev() {
                // move right until we hit a square rock or a wall
                for y in 0..=max_y {
                    let current = new_map.get(&(x, y));
                    // keep moving right until we hit a wall or square rock
                    if current == Some(&Rock::Round) {
                        //find end
                        let mut end = x;
                        // Find the end, this is where the rock will go
                        //First select the rock we are on, then move right until we hit one above.
                        for x2 in x..=max_x {
                            let right = new_map.get(&(x2 + 1, y));
                            if right == Some(&Rock::Square) || right == Some(&Rock::Round) {
                                //If we hit another rock, then we have to stop
                                end = x2;
                                break;
                            }
                            if x2 == max_x {
                                //If we got all the way to the top, then we can move it to end
                                end = max_x;
                                break;
                            }
                        }

                        //Move the rock up, if you're not in the same spot
                        if end != x {
                            new_map.insert((end, y), Rock::Round);
                            new_map.remove(&(x, y));
                        }
                    }
                }
            }
        } // end east
        Direction::West => {
            for x in 0..=max_x {
                // move left until we hit a square rock or a wall
                for y in 0..=max_y {
                    let current = new_map.get(&(x, y));
                    // keep moving left until we hit a wall or square rock
                    if current == Some(&Rock::Round) {
                        //find end
                        let mut end = x;
                        // Find the end, this is where the rock will go
                        //First select the rock we are on, then move left until we hit one above.
                        for x2 in (0..=x).rev() {
                            let left = new_map.get(&(x2 - 1, y));
                            if left == Some(&Rock::Square) || left == Some(&Rock::Round) {
                                //If we hit another rock, then we have to stop
                                end = x2;
                                break;
                            }
                            if x2 == 0 {
                                //If we got all the way to the top, then we can move it to end
                                end = 0;
                                break;
                            }
                        }

                        //Move the rock up, if you're not in the same spot
                        if end != x {
                            new_map.insert((end, y), Rock::Round);
                            new_map.remove(&(x, y));
                        }
                    }
                }
            }
        } //end west
        Direction::South => {
            for y in (0..=max_y).rev() {
                // move down until we hit a square rock or a wall
                for x in 0..=max_x {
                    let current = new_map.get(&(x, y));
                    // keep moving down until we hit a wall or square rock
                    if current == Some(&Rock::Round) {
                        //find end
                        let mut end = y;
                        // Find the end, this is where the rock will go
                        //First select the rock we are on, then move down until we hit one above.
                        for y2 in y..=max_y {
                            let below = new_map.get(&(x, y2 + 1));
                            if below == Some(&Rock::Square) || below == Some(&Rock::Round) {
                                //If we hit another rock, then we have to stop
                                end = y2;
                                break;
                            }
                            if y2 == max_y {
                                //If we got all the way to the top, then we can move it to end
                                end = max_y;
                                break;
                            }
                        }

                        //Move the rock up, if you're not in the same spot
                        if end != y {
                            new_map.insert((x, end), Rock::Round);
                            new_map.remove(&(x, y));
                        }
                    }
                }
            }
        } //end south
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();

    let mut slid_map = input.clone();
    //let slid_map = slide_until_no_movement(max_x, max_y, input, Direction::North);
    slide_until_no_movement_elided(max_x, max_y, &mut slid_map, Direction::North);
    #[cfg(test)]
    dump_map(&slid_map);
    determine_weight(&slid_map, Direction::North) as u64
}

#[aoc(day14, part2)]
pub fn part2(input: &InputType) -> OutputType {
    part2_by_cycle(input, 1000000000)
}

// lazy_static! {
//     static ref CYCLE_MEMO: BTreeMap<RockMap, RockMap> = BTreeMap::new();
// }

fn run_cycle(max_x: i64, max_y: i64, input: &mut InputType) {
    slide_until_no_movement_elided(max_x, max_y, input, Direction::North);
    slide_until_no_movement_elided(max_x, max_y, input, Direction::West);
    slide_until_no_movement_elided(max_x, max_y, input, Direction::South);
    slide_until_no_movement_elided(max_x, max_y, input, Direction::East);

    #[cfg(test)]
    dump_map(&input);
}

pub fn part2_by_cycle(input: &InputType, cycles: usize) -> OutputType {
    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();
    //let CYCLES = 1000000000;
    //Cycle is north, west, south ,east

    let mut memo: BTreeMap<RockMap, RockMap> = BTreeMap::new();

    #[allow(unused_variables)]
    let mut cache_hit = 0;

    //Definitly a cycle, to determine cycle length, let's keep a vec of transforms, if we ever find one in the map, then we know we've found a cycle, and can just forward to the end

    let mut last_cache_size = 1;

    //let mut loop_breaker_count = None;

    let mut loop_start = None;

    let mut last_map = input.clone();
    for cycle in 0..cycles {
        if let Some((cycle_num, ref cycle_map)) = loop_start {
            if *cycle_map == last_map {
                // println!("Cycle detected, cycle length {}", cycle - cycle_num);
                let cycle_length = cycle - cycle_num;
                //At this point, we know the cycle length, and we know that we are just at the
                //start of the cycle, that means we can just find where the cycle will be at when we hit the final bit
                let cycles_left = cycles - cycle_num;
                let cycles_to_skip = cycles_left % cycle_length;
                // println!("Cycles left {}, cycles to skip {}", cycles_left, cycles_to_skip);
                for _ in 0..cycles_to_skip {
                    run_cycle(max_x, max_y, &mut last_map);
                }
                break;
            }
        }

        if cycle % 1000 == 0 {
            #[cfg(test)]
            {
                println!("Cycle {}", cycle);
                println!("Cache hit {} cache_size {}", cache_hit, memo.len());
            }
            if memo.len() == last_cache_size {
                #[cfg(test)]
                println!("Cache size hasn't changed, cycle detected");
                //There is a cycle in here, we don't know how big it is thoug, let's loop until we see the same map twice
                loop_start = Some((cycle, last_map.clone()));
            }
            last_cache_size = memo.len();
        }
        if let Some(cycle_map) = memo.get(&last_map) {
            last_map = cycle_map.clone();
            cache_hit += 1;
            continue;
        }
        let before_cycle = last_map.clone();
        run_cycle(max_x, max_y, &mut last_map);
        memo.insert(before_cycle, last_map.clone());
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
    fn elided_is_non() {
        let mut input = day14_parse(get_test_input());
        let hard_method = slide_until_no_movement(9, 9, &input, Direction::North);
        slide_until_no_movement_elided(9, 9, &mut input, Direction::North);
        println!("Hard");
        dump_map(&hard_method);
        println!();
        println!("Elided");
        dump_map(&input);
        assert_eq!(hard_method, input);
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
        let should_be = day14_parse(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        );
        let mut cycler = day14_parse(get_test_input());
        run_cycle(9, 9, &mut cycler);
        println!("After 1 cycle");
        dump_map(&cycler);

        run_cycle(9, 9, &mut cycler);
        println!("After 2 cycles");
        dump_map(&cycler);

        run_cycle(9, 9, &mut cycler);
        println!("After 3 cycles");
        dump_map(&cycler);

        assert_eq!(cycler, should_be);
    }
}
