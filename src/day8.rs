type InputType = (usize, usize, BTreeMap<(u64, u64), char>);
type OutputType = u64;

use std::collections::{BTreeMap, BTreeSet};

#[aoc_generator(day8)]
fn day8_parse(input: &str) -> InputType {
    input.lines().enumerate().fold(
        (0, 0, BTreeMap::new()),
        |(max_x, _max_y, mut map), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    map.insert((x as u64, y as u64), c);
                }
            });
            (max_x.max(line.len()), y, map)
        },
    )
}

#[allow(dead_code)]
fn dump_map(
    max_x: usize,
    max_y: usize,
    map: &BTreeMap<(u64, u64), char>,
    highlights: &BTreeSet<(u64, u64)>,
    anti_nodes: BTreeMap<(u64, u64), char>,
) {
    let max_x = max_x as u64;
    let max_y = max_y as u64;

    use ansi_term::Color::{Green, Red};

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(a) = anti_nodes.get(&(x, y)) {
                print!("{}", Red.paint(a.to_string()));
            } else if let Some(c) = map.get(&(x, y)) {
                if highlights.contains(&(x, y)) {
                    print!("{}", Green.paint(c.to_string()));
                } else {
                    print!("{}", c);
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(input: &InputType, part2: bool) -> OutputType {
    //Antinodes can occur on the same spot
    let mut anti_nodes: BTreeMap<(u64, u64), BTreeSet<char>> = BTreeMap::new();

    let max_x = input.0;
    let max_y = input.1;
    let map = &input.2;

    for ((x, y), c) in map.iter() {
        //Let's find all the other frequencies that match, they need not be on a perfect 1:1 line, you need to care about the rise over run

        let x = *x;
        let y = *y;
        let c = *c;

        let mut other_towers: BTreeSet<(u64, u64)> = BTreeSet::new();

        for ((x2, y2), c2) in map.iter() {
            if *c2 != c {
                continue;
            }

            if x == *x2 && y == *y2 {
                //Skip yourself
                continue;
            }
            other_towers.insert((*x2, *y2));
        }

        #[cfg(test)]
        {
            println!("original tower + other towers: ({},{}): {}", x, y, c);
            dump_map(max_x, max_y, map, &other_towers, BTreeMap::new());
        }

        for (x2, y2) in other_towers.iter() {
            let rise = (y as i64 - *y2 as i64).abs();
            let run = (x as i64 - *x2 as i64).abs();
            // Inject an antinode on the opposite of the rise and run for each tower
            // For the first tower

            //Having a hard time visualizing this, I think I need to be concious of the direction
            let going_up = *y2 > y;
            let going_right = *x2 > x;

            let rise = if going_up { rise } else { -rise };
            let run = if going_right { run } else { -run };

            let mut dx = vec![x as i64 - run, x as i64 + run];
            let mut dy = vec![y as i64 - rise, y as i64 + rise];

            if part2 {
                // These continue on by the run and rise until they're off the edge
                // Add dx and dy for each element in here until they're off the edge, add or subtract by run and rise as appropriate
                //Kind of a lame way to do this, but whatever.
                let mut mult = 2;
                loop {
                    let next_x = dx[0] - (mult * run);
                    let next_y = dy[0] - (mult * rise);
                    if next_x >= 0
                        && next_y >= 0
                        && next_x <= max_x as i64
                        && next_y <= max_y as i64
                    {
                        dx.push(next_x);
                        dy.push(next_y);
                    } else {
                        break;
                    }
                    mult += 1;
                }
                let mut mult = 2;
                loop {
                    let next_x = dx[1] - (mult * run);
                    let next_y = dy[1] - (mult * rise);
                    if next_x >= 0
                        && next_y >= 0
                        && next_x <= max_x as i64
                        && next_y <= max_y as i64
                    {
                        dx.push(next_x);
                        dy.push(next_y);
                    } else {
                        break;
                    }
                    mult += 1;
                }
            }

            for (t_x, t_y) in dx.iter().zip(dy.iter()) {
                let t_x = *t_x;
                let t_y = *t_y;

                if t_x >= 0 && t_y >= 0 && t_x <= max_x as i64 && t_y <= max_y as i64 {
                    //Antinodes can not occur outside the map NOR can they occur on the same spot of a tower that is the same frequency
                    let mut insert = false;
                    if let Some(c2) = map.get(&(t_x as u64, t_y as u64)) {
                        //In part2, we don't care if there is an antenna in the same place
                        if c != *c2 || part2 {
                            //There is something here, but it's not the same frequency
                            insert = true;
                        }
                    } else {
                        insert = true;
                    }

                    if insert {
                        anti_nodes
                            .entry((t_x as u64, t_y as u64))
                            .or_default()
                            .insert(c);
                    }
                }
            }
        }
        #[cfg(test)]
        {
            println!("original tower: ({},{}): {}", x, y, c);
            println!("Antinodes:");
            println!("{:?}", anti_nodes);
            dump_map(
                max_x,
                max_y,
                map,
                &other_towers,
                anti_nodes
                    .iter()
                    .map(|(k, v)| (*k, *v.iter().next().unwrap()))
                    .collect(),
            );
        }
    }

    // It doesn't care about how many unique antinodes, just that they exist
    anti_nodes.len() as u64
}

#[aoc(day8, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //let's find antinodes, the rules for that are:
    // any point in line with two of the same frequencies (characters)
    // Only when one antenna is twice as far away from the other
    solve(input, false)
}

#[aoc(day8, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solve(input, true)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    fn simple_test_input() -> &'static str {
        "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
    }

    fn simpleish_test_input() -> &'static str {
        "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
.........."
    }

    #[test]
    fn day8_part1_simple() {
        assert_eq!(part1(&day8_parse(simple_test_input())), 2);
    }
    #[test]
    fn day8_part1_simple2() {
        // 3,1
        // 0,2
        // 2,6
        // 6,7
        assert_eq!(part1(&day8_parse(simpleish_test_input())), 4);
    }

    #[test]
    fn day8_part1() {
        assert_eq!(part1(&day8_parse(get_test_input())), 14);
    }

    fn day8_part2_simple_input() -> &'static str {
        "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
    }

    #[test]
    fn day8_part2_simple() {
        assert_eq!(part2(&day8_parse(day8_part2_simple_input())), 9);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(part2(&day8_parse(get_test_input())), 34);
    }
}
