use std::collections::HashMap;


type InputType = (HashMap<(i32, i32), Tile>, (usize, usize)>); //max_x, max_y
type OutputType = u64;

pub enum Tile {
    Forest,
    Path,
    SlopeEast, // >
    SlopeWest, // <
    SlopeNorth, // ^
    SlopeSouth, // v
}

#[aoc_generator(day23)]
fn day23_parse(input: &str) -> InputType {
    let map = input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            let tile = match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '>' => Tile::SlopeEast,
                '<' => Tile::SlopeWest,
                '^' => Tile::SlopeNorth,
                'v' => Tile::SlopeSouth,
                _ => panic!("Invalid tile"),
            };
            ((x as i32, y as i32), tile)
        })
    }).collect();

    let max_x = input.lines().next().unwrap().len();
    let max_y = input.lines().count();

    (map, max_x, max_y)
}

#[aoc(day23, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //In the examle and in the input, you generally only have one direction you can go, there is no
    //double wide paths, so we can probably constract the nodes and keep track of their cost
    todo!();
}

#[aoc(day23, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
    }

    #[test]
    fn day23_part1() {
        assert_eq!(part1(&day23_parse(get_test_input())), 0);
    }

    #[test]
    fn day23_part2() {
        assert_eq!(part2(&day23_parse(get_test_input())), 0);
    }
}
