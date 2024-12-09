type InputType = (usize, usize, BTreeMap<Point, char>);
type OutputType = u64;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn new_u(x: u64, y: u64) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }

    fn dist(&self, other: &Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }

    fn in_bounds(&self, max_x: usize, max_y: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x <= max_x as i64 && self.y <= max_y as i64
    }

    fn add(&self, other: &Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[aoc_generator(day8)]
fn day8_parse(input: &str) -> InputType {
    input.lines().enumerate().fold(
        (0, 0, BTreeMap::new()),
        |(max_x, _, mut map), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    map.insert(Point::new_u(x as u64, y as u64), c);
                }
            });

            (max_x.max(line.len() - 1), y, map)
        },
    )
}

#[allow(dead_code)]
fn dump_map(
    max_x: usize,
    max_y: usize,
    map: &BTreeMap<Point, char>,
    highlights: &BTreeSet<Point>,
    anti_nodes: &BTreeSet<Point>,
) {
    let max_x = max_x as u64;
    let max_y = max_y as u64;

    use ansi_term::Color::Green;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if anti_nodes.get(&Point::new_u(x, y)).is_some() {
                print!("#");
            } else if let Some(c) = map.get(&Point::new_u(x, y)) {
                if highlights.contains(&Point::new_u(x, y)) {
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
    let mut anti_nodes: BTreeSet<Point> = BTreeSet::new();

    let max_x = input.0;
    let max_y = input.1;
    let map = &input.2;

    for (p, c) in map.iter() {
        //Let's find all the other frequencies that match, they need not be on a perfect 1:1 line, you need to care about the rise over run

        //Find other towers
        for (p2, c2) in map.iter() {
            if *c2 != *c {
                continue;
            }

            if p.x == p2.x && p.y == p2.y {
                //Skip yourself
                continue;
            }

            if part2 {
                anti_nodes.insert(*p); //Since we skipped ourselves, add in the resonant node
                                       //for... ourselves.
                let dist = p.dist(p2);
                let mut next = p.add(&dist);
                while next.in_bounds(max_x, max_y) {
                    anti_nodes.insert(next);
                    next = next.add(&dist);
                }
            } else {
                let new_antinode = p.add(&p.dist(p2));

                if new_antinode.in_bounds(max_x, max_y) {
                    anti_nodes.insert(new_antinode);
                }
            }
        }

        #[cfg(test)]
        {
            println!("original tower: ({},{}): {}", p.x, p.y, c);
            println!("Antinodes:");
            println!("{:?}", anti_nodes);
            dump_map(max_x, max_y, map, &BTreeSet::new(), &anti_nodes);
        }
    }

    // It doesn't care about how many unique antinodes, just that they exist
    anti_nodes.len() as u64
}

#[aoc(day8, part1)]
pub fn part1(input: &InputType) -> OutputType {
    println!("{:?}", input);
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
