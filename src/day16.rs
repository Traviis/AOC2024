use std::collections::{HashMap, VecDeque};

type Coordinate = (i64, i64);
type InputType = HashMap<Coordinate, Mirror>;
type OutputType = u64;

use colored::Colorize as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mirror {
    Horizontal,
    Vertical,
    ForwardDiagonal,  // /
    BackwardDiagonal, // \
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn dir_from(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn dump_map(map: &InputType) {}

fn dump_visit_map(max_x: i64, max_y: i64, map: &HashMap<Coordinate, Vec<Direction>>, under_map: &InputType, highlight: Vec<Coordinate>) {
    //Print the map, ignoring the direction
    for y in 0..=max_y {
        for x in 0..=max_x {
            let coord = (x, y);
            let mirror = map.get(&coord);
            match mirror {
                Some(_) => {
                    print!("{}", if highlight.contains(&coord) { "#".red() } else { "#".normal() });
                }
                None => {
                    match under_map.get(&coord) {
                        Some(Mirror::Horizontal) => {
                            print!("{}", if highlight.contains(&coord) { "-".red() } else { "-".normal() });
                        }
                        Some(Mirror::Vertical) => {
                            print!("{}", if highlight.contains(&coord) { "|".red() } else { "|".normal() });

                        }
                        Some(Mirror::ForwardDiagonal) => {
                            print!("{}", if highlight.contains(&coord) { "/".red() } else { "/".normal() });
                        }
                        Some(Mirror::BackwardDiagonal) => {
                            print!("{}", if highlight.contains(&coord) { "\\".red() } else { "\\".normal() });
                        }
                        None => {
                            print!("{}", if highlight.contains(&coord) { ".".red() } else { ".".normal() });
                        }
                    }
                }
            }
        }
        println!();
    }
    println!();
}

#[aoc_generator(day16)]
fn day16_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                let coord = (x as i64, y as i64);
                let mirror = match c {
                    '-' => Some(Mirror::Horizontal),
                    '|' => Some(Mirror::Vertical),
                    '/' => Some(Mirror::ForwardDiagonal),
                    '\\' => Some(Mirror::BackwardDiagonal),
                    '.' => None,
                    _ => panic!("Invalid character ({})", c),
                };
                if mirror.is_some() {
                    return Some((coord, mirror.unwrap()));
                }
                None
            })
        })
        .collect()
}

#[aoc(day16, part1)]
pub fn part1(input: &InputType) -> OutputType {
    // Keep track of all tiles that have been visited and which directions they came from (if you
    // ever hit the same location at the same direction, the processing can end, because it will
    // have merged with the other light.

    let mut visited = HashMap::new();

    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();

    let mut queue = VecDeque::new();
    queue.push_back(((-1, 0), Direction::East)); // Start at the top left corner, facing east

    while let Some(((cur_x, cur_y), heading)) = queue.pop_front() {

        if cur_x >= 0 && cur_x <= max_x && cur_y >= 0 && cur_y <= max_y {
            visited
                .entry((cur_x,cur_y))
                .or_insert_with(Vec::new)
                .push(heading.dir_from());
        }
        // First see what the next tile we are going to will be
        let offset = match heading {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };

        let next_coord = (cur_x + offset.0, cur_y + offset.1);

        if next_coord.0 < 0 || next_coord.1 < 0 || next_coord.0 > max_x || next_coord.1 > max_y {
            // #[cfg(test)]
            // println!("Hit the edge of the map, so we can stop");
            // Hit the edge of the map, so we can stop
            continue;
        }

        // #[cfg(test)]
        // println!(
        //     "Starting at {:?} heading {:?} next_coord is {:?}",
        //     (cur_x, cur_y),
        //     dir,
        //     next_coord
        // );

        if visited
            .get(&next_coord)
            .map(|v: &Vec<Direction>| v.contains(&heading.dir_from()))
            .unwrap_or(false)
        {
            // #[cfg(test)]
            // println!("Already been here from this direction, so we can stop (merge)");
            // We have already been here from this direction, so we can stop
            continue;
        }


        let item_at_next_coord = input.get(&next_coord);
        // #[cfg(test)]
        // println!("Item at next coord is {:?}", item_at_next_coord);
        match item_at_next_coord {
            Some(Mirror::Horizontal) => {
                //If we are going east or west, we can just keep going
                if heading == Direction::East || heading == Direction::West {
                    queue.push_back((next_coord, heading));
                } else {
                    // If you are north or sourth, then create two beams going out both directions
                    // Since it splits, it doesn't matter if it came from the north or south it
                    // counts as having the same outcome
                    queue.push_back((next_coord, Direction::East));
                    queue.push_back((next_coord, Direction::West));
                }
            }
            Some(Mirror::Vertical) => {
                if heading == Direction::North || heading == Direction::South {
                    queue.push_back((next_coord, heading));
                } else {
                    queue.push_back((next_coord, Direction::North));
                    queue.push_back((next_coord, Direction::South));
                }
            }
            Some(Mirror::ForwardDiagonal) => {
                // "/"
                match heading {
                    Direction::North => {
                        queue.push_back((next_coord, Direction::East));
                    }
                    Direction::South => {
                        queue.push_back((next_coord, Direction::West));
                    }
                    Direction::East => {
                        queue.push_back((next_coord, Direction::North));
                    }
                    Direction::West => {
                        queue.push_back((next_coord, Direction::South));
                    }
                }
            }
            Some(Mirror::BackwardDiagonal) => {
                // "\"
                match heading {
                    Direction::North => {
                        queue.push_back((next_coord, Direction::West));
                    }
                    Direction::South => {
                        queue.push_back((next_coord, Direction::East));
                    }
                    Direction::East => {
                        queue.push_back((next_coord, Direction::South));
                    }
                    Direction::West => {
                        queue.push_back((next_coord, Direction::North));
                    }
                }
            }
            None => {
                queue.push_back((next_coord, heading)); //Keep going in the same direction
            }
        }

        // if visited.len() > 3  {
        //     println!("Visited {} tiles",visited.len());
        //     break
        // }
        #[cfg(test)]
        dump_visit_map(max_x, max_y, &visited, input, queue.clone().into_iter().map(|(c, _)| c).collect());
    }
    #[cfg(test)]
    dump_visit_map(max_x,max_y,&visited, input, Vec::new());

    visited.len() as u64
}

#[aoc(day16, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        ".|...\\....
            |.-.\\.....
            .....|-...
            ........|.
            ..........
            .........\\
            ..../.\\\\..
            .-.-/..|..
            .|....-|.\\
            ..//.|...."
    }

    #[test]
    fn day16_part1() {
        assert_eq!(part1(&day16_parse(get_test_input())), 46);
    }

    #[test]
    fn day16_part2() {
        assert_eq!(part2(&day16_parse(get_test_input())), 0);
    }
}
