use std::collections::HashMap;

use regex::Regex;

type InputType = Vec<Instruction>;
type OutputType = u64;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Instruction {
    direction: Direction,
    steps: u64,
    color_code: String
}

#[aoc_generator(day18)]
fn day18_parse(input: &str) -> InputType {

    let pat = Regex::new(r"([LURD]) (\d+) \((#[0-9a-f]{6})\)").unwrap();

    input.lines().map(|line| {
        let pats = pat.captures(line).unwrap();

        let dir = match &pats[1] {
            "L" => Direction::Left,
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            _ => panic!("Invalid direction {}",&pats[0])
        };
        let steps = pats[2].parse::<u64>().unwrap();
        let color_code = pats[3].to_string();
        Instruction { direction: dir, steps, color_code }
    }).collect()

}

enum Tile {
    Dug,
    DugAndPainted(String),
}

fn dump_map(map: &HashMap<(i64,i64),Tile>) {
    let min_x = *map.keys().map(|(x,_)| x).min().unwrap();
    let max_x = *map.keys().map(|(x,_)| x).max().unwrap();
    let min_y = *map.keys().map(|(_,y)| y).min().unwrap();
    let max_y = *map.keys().map(|(_,y)| y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            match map.get(&(x,y)) {
                Some(Tile::Dug) => print!("#"),
                Some(Tile::DugAndPainted(_)) => print!("*"),
                None => print!("."),
            }
        }
        println!();
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //First use the instructions to dig a trench in the ground, this s hould be a complete circuit
    //and it also paints the edges while you go
    let mut map = HashMap::new();

    let mut cur_x = 0;
    let mut cur_y = 0;

    for inst in input {
        for _ in 0..inst.steps {
            match inst.direction {
                Direction::Up => cur_y += 1,
                Direction::Down => cur_y -= 1,
                Direction::Left => cur_x -= 1,
                Direction::Right => cur_x += 1,
            }
            map.entry((cur_x,cur_y)).or_insert(Tile::DugAndPainted((inst.color_code.clone())));
        }
    }

    #[cfg(test)]
    dump_map(&map);

    // Now that we have the border dug out, let's use the same algorithm as before to find the
    // inner parts that need to be filled in (as opposed to flood fill) (knot theory)





    0

}

#[aoc(day18, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
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
    fn day18_part1() {
        assert_eq!(part1(&day18_parse(get_test_input())), 62);
    }

    #[test]
    fn day18_part2() {
        assert_eq!(part2(&day18_parse(get_test_input())), 0);
    }
}
