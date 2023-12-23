use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<Instruction>;
type OutputType = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Instruction {
    direction: Direction,
    steps: u64,
    color_code: String,
}

#[aoc_generator(day18)]
fn day18_parse(input: &str) -> InputType {
    let pat = Regex::new(r"([LURD]) (\d+) \((#[0-9a-f]{6})\)").unwrap();

    input
        .lines()
        .map(|line| {
            let pats = pat.captures(line).unwrap();

            let dir = match &pats[1] {
                "L" => Direction::Left,
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                _ => panic!("Invalid direction {}", &pats[0]),
            };
            let steps = pats[2].parse::<u64>().unwrap();
            let color_code = pats[3].to_string();
            Instruction {
                direction: dir,
                steps,
                color_code,
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Dug,
    DugAndPainted(String),
}

fn dump_map(map: &HashMap<(i64, i64), Tile>) {
    let min_x = *map.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *map.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    if max_x - min_x > 100 || max_y - min_y > 100 {
        println!("Map too big to print");
        return;
    }

    for y in (min_y..=max_y).rev() {
        print!("{:3} ", y);
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(Tile::Dug) => print!("#"),
                Some(Tile::DugAndPainted(_)) => print!("*"),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

#[aoc(day18, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //First use the instructions to dig a trench in the ground, this s hould be a complete circuit
    //and it also paints the edges while you go
    let mut map = HashMap::new();

    let mut cur_x = 0;
    let mut cur_y = 0;

    for inst in input {
        //TODO: Optimize?
        for _ in 0..inst.steps {
            match inst.direction {
                Direction::Up => cur_y += 1,
                Direction::Down => cur_y -= 1,
                Direction::Left => cur_x -= 1,
                Direction::Right => cur_x += 1,
            }
            map.entry((cur_x, cur_y))
                .or_insert(Tile::DugAndPainted(inst.color_code.clone()));
        }
    }

    #[cfg(test)]
    dump_map(&map);

    // Now that we have the border dug out, let's use the same algorithm as before to find the
    // inner parts that need to be filled in (as opposed to flood fill) (knot theory day10)

    let min_x = *map.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *map.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();

    let edge_map = map.clone();

    let mut area = edge_map.len() as i64;

    println!("Area: {}", area);

    for y in min_y..=max_y {
        let mut loop_count = 0;
        let mut last_x = None;
        let mut edges_in_row: Vec<_> = edge_map
            .iter()
            .filter(|((x, dy), _)| *dy == y)
            .map(|((x, _), _)| *x)
            .collect();

        edges_in_row.sort();
        //Remove consective numbers, that are part of the same horizontal edge, replace them with their min and max
        edges_in_row = edges_in_row
            .into_iter()
            .group_by(|&x| x)
            .into_iter()
            .flat_map(|(key, group)| {
                let count = group.count();
                match count {
                    1 => vec![key],
                    _ => vec![key, key + count as i64 - 1],
                }
            })
            .collect();

        // #[cfg(test)]
        // {
        //     println!("Edges in row: {} - {:?}", y, edges_in_row);
        // }
        //TODO: Might have to keep track of the previous Y range and check if edge is there or if
        //it's in a range of a tile that's dug
        for x in edges_in_row.iter() {
            //Instead of iterating over the whole map, just look at the edges and count to the next to subtracting the
            //number of tiles in between
            let segment = map.get(&(*x, y));
            let above = map.get(&(*x, y + 1));

            loop_count += 1;
            //Do they have to be part of the rope test?, we know that it was a perimeter that we
            //cut, so we can assume there are no holes in between
            // if above.is_some() {
            //     loop_count += 1;
            // }
            if loop_count % 2 == 0 && last_x.is_some() {
                let last_x_u = last_x.unwrap();

                let to_add = (x - last_x_u - 1) as i64;
                #[cfg(test)]
                println!("Adding {} to area for row {}", to_add, y);
                area += to_add;
                //every time you close a loop (loop_count is even, but not 0) and you have a last_x
                //then you can add the number of tiles between the last_x and the current x to the Area
                //and set last_x to None
                for d_x in (last_x_u + 1)..*x {
                    //For debug, let's fill in the map area
                    map.insert((d_x, y), Tile::Dug);
                }
                last_x = Some(*x);
                //last_x = None
            } else if last_x.is_none() {
                last_x = Some(*x);
            }
        }
    }
    //TODO: This is almost there, It's breaking on the case of **..***, it should be **##*** but I get Nothing.
    // It's a bug in how I'm handling loop_count

    /*
    for y in min_y..=max_y {
        let mut loop_count = 0;
        for x in min_x..=max_x {
            let segment = map.get(&(x, y));
            let above = map.get(&(x, y + 1));
            if above.is_some() && segment.is_some() {
                loop_count += 1;
            }
            if loop_count % 2 == 1 && segment.is_none() {
                map.insert((x, y), Tile::Dug);
            }
        }
    }
    */

    #[cfg(test)]
    dump_map(&map);

    area as u64
}

fn convert_inst(inst: &Instruction) -> Instruction {
    let raw_hex = &inst.color_code[1..6];
    let hex_dist = u64::from_str_radix(raw_hex, 16).unwrap();
    let dir = match &inst.color_code[6..] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("Invalid direction {}", &inst.color_code[5..]),
    };
    Instruction {
        direction: dir,
        steps: hex_dist,
        color_code: inst.color_code.clone(),
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let fixed_map = input
        .iter()
        .map(|inst| convert_inst(inst))
        .collect::<Vec<_>>();

    part1(&fixed_map)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)"
    }

    #[test]
    fn day18_part2_parse() {
        let bad_inst = Instruction {
            direction: Direction::Right,
            steps: 6,
            color_code: "#70c710".to_string(),
        };

        let converted = convert_inst(&bad_inst);

        assert_eq!(converted.direction, Direction::Right);
        assert_eq!(converted.steps, 461937);
    }

    #[test]
    fn day18_part1() {
        assert_eq!(part1(&day18_parse(get_test_input())), 62);
    }

    #[test]
    fn day18_part2() {
        assert_eq!(part2(&day18_parse(get_test_input())), 952408144115);
    }
}
