use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};
use std::{collections::BTreeMap, fmt};

pub struct Map(BTreeMap<Point, Height>);

type Point = (i64, i64);
type Height = i8;

type InputType = Map;
type OutputType = u64;

impl Deref for Map {
    type Target = BTreeMap<Point, Height>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    fn max_x(&self) -> i64 {
        self.iter().map(|((x, _), _)| *x).max().unwrap()
    }
    fn max_y(&self) -> i64 {
        self.iter().map(|((_, y), _)| *y).max().unwrap()
    }

    fn find_adjacent_with_plus_one(&self, p: &Point) -> Vec<Point> {
        let mut adjacent = Vec::new();
        let (x, y) = p;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_p = (x + dx, y + dy);
            if let Some(h) = self.get(&new_p) {
                if *h == self.get(p).unwrap() + 1 {
                    adjacent.push(new_p);
                }
            }
        }
        adjacent
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.max_y() {
            for x in 0..=self.max_x() {
                match self.get(&(x, y)) {
                    Some(h) => write!(f, "{}", h)?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[aoc_generator(day10)]
fn day10_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .fold(Map(BTreeMap::new()), |mut map, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '.' {
                    return; //tests
                }
                map.insert((x as i64, y as i64), c.to_digit(10).unwrap() as i8);
            });

            map
        })
}

impl Map {
    fn find_trailheads(&self) -> Vec<Point> {
        self.iter()
            .filter(|(_, v)| **v == 0)
            .map(|(p, _)| *p)
            .collect::<Vec<Point>>()
    }

    fn solve(&self, part2: bool) -> u64 {
        #[cfg(test)]
        print!("{}", self);
        //Find trailheads
        let trailheads = self.find_trailheads();

        #[cfg(test)]
        println!("{:?}", trailheads);

        // Just.... DFS?
        trailheads
            .iter()
            .map(|trailhead| {
                let mut completed_trails = BTreeSet::new();
                let mut completed_trails_count = 0;

                let mut stack = vec![(*trailhead, 0)];

                while let Some(considered_node) = stack.pop() {
                    let (node, height) = considered_node;
                    let adjacents = self.find_adjacent_with_plus_one(&node);

                    for adj in adjacents {
                        if *self.get(&adj).unwrap() == 9 {
                            completed_trails.insert(adj);
                            //For part2, we care about unique trails, in part 1, I was throwing this
                            //out, since I didn't care that multiple unique trails hit the same end
                            //node,
                            completed_trails_count += 1;
                        } else {
                            stack.push((adj, height + 1));
                        }
                    }
                }

                if part2 {
                    completed_trails_count
                } else {
                    completed_trails.len()
                }
            })
            .sum::<usize>() as u64
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.solve(false)
}

#[aoc(day10, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input.solve(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    fn get_simple_test_input() -> &'static str {
        "0123
1234
8765
9876"
    }

    fn get_simple_test_input2() -> &'static str {
        "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"
    }

    fn get_simple_test_input3() -> &'static str {
        "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"
    }

    #[test]
    fn day10_part1_simple() {
        assert_eq!(part1(&day10_parse(get_simple_test_input())), 1);
    }

    #[test]
    fn day10_part1_simple2() {
        assert_eq!(part1(&day10_parse(get_simple_test_input2())), 2);
    }

    #[test]
    fn day10_part1_simple3() {
        assert_eq!(part1(&day10_parse(get_simple_test_input3())), 3);
    }

    #[test]
    fn day10_part1() {
        assert_eq!(part1(&day10_parse(get_test_input())), 36);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(&day10_parse(get_test_input())), 81);
    }
}
