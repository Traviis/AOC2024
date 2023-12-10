#[allow(unused_imports)]
use colored::Colorize as _;
use std::collections::HashMap;

pub type Coordinate = (i64, i64);

pub type Map = HashMap<Coordinate, Segment>;

type InputType = (Coordinate, Map);
type OutputType = i64;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub enum Segment {
    Vertical,   // |
    Horizontal, // -
    NE,         // L
    NW,         // J
    SW,         // 7
    SE,         // F
    Start,      // S
    Empty,
}

#[aoc_generator(day10)]
fn day10_parse(input: &str) -> InputType {
    let mut map: Map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.trim();
            line.chars().enumerate().filter_map(move |(x, c)| {
                let segment = match c {
                    '|' => Segment::Vertical,
                    '-' => Segment::Horizontal,
                    'L' => Segment::NE,
                    'J' => Segment::NW,
                    '7' => Segment::SW,
                    'F' => Segment::SE,
                    'S' => Segment::Start,
                    '.' => Segment::Empty,
                    _ => panic!("Invalid character in input {}", c),
                };
                // if let Segment::Start = segment {
                //     *start_ref = (x as i64, y as i64);
                // }
                if let Segment::Empty = segment {
                    return None;
                }
                Some(((x as i64, y as i64), segment))
            })
        })
        .collect();

    //TODO: Yes, this is super lame to clone here, but I just want to not worry about all the references into it when I'm finding start
    let map_clone = map.clone();
    let (start, _) = map_clone
        //DOn't awnt to mess with closure captures above
        .iter()
        .find(|(_, segment)| {
            if let Segment::Start = segment {
                true
            } else {
                false
            }
        })
        .unwrap();

    //Let's also replace the start pipe with the proper segment
    let mut valid_next_segments_found = valid_next_segments(&map, None, start);

    //Cheat and sort this so I can cut down on my combinations

    //Pick the first one and to determine a direction arbitrarily, then see which piece fits

    let start_segment = valid_next_segments_found.pop().unwrap();
    //Find the start one from this neighbor

    let next_segment = valid_next_segments_found.pop().unwrap();

    let direction = determine_direction_from(Some(&start_segment), &start);

    let dx = next_segment.0 - start.0;
    let dy = next_segment.1 - start.1;

    match (dx, dy) {
        //I just moved east, so determine what valid pipes end in east and start from direction
        (1, 0) => match direction {
            Dir::North => map.insert(*start, Segment::NE),
            Dir::South => map.insert(*start, Segment::SE),
            Dir::East => map.insert(*start, Segment::Horizontal),
            Dir::West => map.insert(*start, Segment::Horizontal),
            Dir::Center => unreachable!(),
        },
        (-1, 0) => match direction {
            Dir::North => map.insert(*start, Segment::NW),
            Dir::South => map.insert(*start, Segment::SW),
            Dir::East => map.insert(*start, Segment::Horizontal),
            Dir::West => map.insert(*start, Segment::Horizontal),
            Dir::Center => unreachable!(),
        },
        (0, 1) => match direction {
            Dir::North => map.insert(*start, Segment::Vertical),
            Dir::South => map.insert(*start, Segment::Vertical),
            Dir::East => map.insert(*start, Segment::SE),
            Dir::West => map.insert(*start, Segment::SW),
            Dir::Center => unreachable!(),
        },
        (0, -1) => match direction {
            Dir::North => map.insert(*start, Segment::Vertical),
            Dir::South => map.insert(*start, Segment::Vertical),
            Dir::East => map.insert(*start, Segment::NE),
            Dir::West => map.insert(*start, Segment::NW),
            Dir::Center => unreachable!(),
        },
        _ => unreachable!(),
    };

    // Vertical,   // |
    // Horizontal, // -
    // NE,         // L
    // NW,         // J
    // SW,         // 7
    // SE,         // F
    // Start,      // S
    // Empty,

    #[cfg(test)]
    {
        dump_full_map(&map);
    }

    (*start, map)
}

#[derive(PartialEq, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
    Center,
}

fn determine_direction_from(previous: Option<&Coordinate>, current: &Coordinate) -> Dir {
    let (x, y) = current;
    match previous {
        Some((p_x, p_y)) => {
            let (dx, dy) = (x - p_x, y - p_y);
            match (dx, dy) {
                (0, 1) => Dir::North,  //I just came from the bottom, so I can't go back down
                (0, -1) => Dir::South, //I just came from the top, so I can't go back up
                (1, 0) => Dir::West,   //I just came from the left, so I can't go back left
                (-1, 0) => Dir::East,  //I just came from the right, so I can't go back right
                _ => panic!("Invalid previous coordinate"),
            }
        }
        None => Dir::Center,
    }
}

fn valid_next_segments(
    map: &Map,
    previous: Option<&Coordinate>,
    current: &Coordinate,
) -> Vec<Coordinate> {
    //Iterate through the cardinal directions, and find the next valid segments
    let mut valid_segments = Vec::new();
    let (x, y) = current;
    //My map coordinates are inverted , up is actually lower y values

    let invalid_next_path = determine_direction_from(previous, current);

    let current_segment = map.get(current).unwrap();

    match invalid_next_path {
        Dir::Center => (),
        Dir::North => {
            //This means I came from the north, so depending on what type I am, I go a certain direction
            match current_segment {
                Segment::Vertical => valid_segments.push((*x, *y + 1)),
                Segment::NE => valid_segments.push((*x + 1, *y)),
                Segment::NW => valid_segments.push((*x - 1, *y)),
                _ => (),
            }
        }
        Dir::South => match current_segment {
            Segment::Vertical => valid_segments.push((*x, *y - 1)),
            Segment::SE => valid_segments.push((*x + 1, *y)),
            Segment::SW => valid_segments.push((*x - 1, *y)),
            _ => (),
        },
        Dir::East => match current_segment {
            Segment::Horizontal => valid_segments.push((*x - 1, *y)),
            Segment::NE => valid_segments.push((*x, *y - 1)),
            Segment::SE => valid_segments.push((*x, *y + 1)),
            _ => (),
        },
        Dir::West => match current_segment {
            Segment::Horizontal => valid_segments.push((*x + 1, *y)),
            Segment::NW => valid_segments.push((*x, *y - 1)),
            Segment::SW => valid_segments.push((*x, *y + 1)),
            _ => (),
        },
    }

    if valid_segments.len() != 0 {
        return valid_segments;
    }

    //Check up
    let n_y = y - 1;
    if invalid_next_path != Dir::North {
        if let Some(segment) = map.get(&(*x, n_y)) {
            match segment {
                Segment::Vertical | Segment::SW | Segment::SE => valid_segments.push((*x, n_y)),
                _ => (),
            }
        }
    }

    //check down
    let s_y = y + 1;
    if invalid_next_path != Dir::South {
        if let Some(segment) = map.get(&(*x, s_y)) {
            match segment {
                Segment::Vertical | Segment::NW | Segment::NE => valid_segments.push((*x, s_y)),
                _ => (),
            }
        }
    }

    //check elft

    let w_x = x - 1;
    if invalid_next_path != Dir::West {
        if let Some(segment) = map.get(&(w_x, *y)) {
            match segment {
                Segment::Horizontal | Segment::NE | Segment::SE => valid_segments.push((w_x, *y)),

                _ => (),
            }
        }
    }

    //check right
    let e_x = x + 1;
    if invalid_next_path != Dir::East {
        if let Some(segment) = map.get(&(e_x, *y)) {
            match segment {
                Segment::Horizontal | Segment::NW | Segment::SW => valid_segments.push((e_x, *y)),
                _ => (),
            }
        }
    }

    valid_segments
}

#[allow(dead_code)]
fn dump_full_map(map: &Map) {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();
    dump_map(&Vec::new(), &Vec::new(), (0, 0), (*max_x, *max_y), map);
}

fn dump_map_containing_paths(path: &Vec<Coordinate>, pos_path: &Vec<Coordinate>, map: &Map) {
    let min_y = path.iter().map(|(_, y)| y).min().unwrap();
    let min_x = path.iter().map(|(x, _)| x).min().unwrap();
    let max_y = path.iter().map(|(_, y)| y).max().unwrap();
    let max_x = path.iter().map(|(x, _)| x).max().unwrap();

    dump_map(
        path,
        pos_path,
        (*min_x - 1, *min_y - 1),
        (*max_x + 1, *max_y + 1),
        map,
    );
}

fn dump_map(
    path: &Vec<Coordinate>,
    pos_path: &Vec<Coordinate>,
    top_left: Coordinate,
    bot_right: Coordinate,
    map: &Map,
) {
    let min_y = top_left.1;
    let min_x = top_left.0;
    let max_y = bot_right.1;
    let max_x = bot_right.0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let segment = map.get(&(x, y));
            let symbol = match segment {
                Some(Segment::Vertical) => "|",
                Some(Segment::Horizontal) => "-",
                Some(Segment::NE) => "L",
                Some(Segment::NW) => "J",
                Some(Segment::SW) => "7",
                Some(Segment::SE) => "F",
                Some(Segment::Start) => "S",
                None => ".",
                Some(Segment::Empty) => unreachable!(),
            };
            if path.contains(&(x, y)) && pos_path.contains(&(x, y)) {
                //This shouldn't actually happen

                print!("{}", symbol.yellow());
            } else if path.contains(&(x, y)) {
                print!("{}", symbol.red());
            } else if pos_path.contains(&(x, y)) {
                print!("{}", symbol.green());
            } else {
                print!("{}", symbol);
            }
        }
        println!();
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &InputType) -> OutputType {
    solver(input, false)
}

pub fn solver(input: &InputType, find_area: bool) -> OutputType {
    let (start, map) = input;
    //Just find the loop and return the length of the loop / 2 (for the furthest poin; or do djiikstra's and return the furthest point)
    /*
    #[cfg(test)]
    {
    dump_full_map(map);
    println!("Start {:?}", start);
    }
    */

    //If these are truly loops, we shouldnt' have to keep track of how many times we've visited, as it simply won't matter

    let mut distance_travelled = 0;

    let (mut cur_x, mut cur_y) = start;

    // First find what connects to the start, (just find the first valid one) and go from there
    let next_segments = valid_next_segments(map, None, &(cur_x, cur_y));

    // #[cfg(test)]
    // {
    //     println!("Next segments for start {:?}", next_segments);
    //     dump_map_containing_paths(
    //         &vec![*start],
    //         &next_segments,
    //         map,
    //     );
    // }

    //For visualization
    let mut path = Vec::new();
    path.push((cur_x, cur_y));

    let mut previous_coordinate = (cur_x, cur_y); //Start
    (cur_x, cur_y) = *next_segments.iter().next().unwrap();
    path.push((cur_x, cur_y));
    distance_travelled += 1;

    loop {
        let next_segments = valid_next_segments(map, Some(&previous_coordinate), &(cur_x, cur_y));
        if next_segments.len() != 1 {
            path.push((cur_x, cur_y));
            println!("next segments {:?}, {}", next_segments, distance_travelled);
            dump_map_containing_paths(&path, &next_segments, map);
            panic!();
        }
        /*
        #[cfg(test)]
        {
        println!("Next segments (singular) {:?}", next_segments);
        }
        */
        let (n_x, n_y) = next_segments.iter().next().unwrap();

        /*
        #[cfg(test)]
        dump_map_containing_paths(
        &path,
        &next_segments,
        map,
        );
        */

        distance_travelled += 1;
        if *n_x == start.0 && *n_y == start.1 {
            break;
        }
        previous_coordinate = (cur_x, cur_y);
        path.push((*n_x, *n_y));
        (cur_x, cur_y) = (*n_x, *n_y);
    }

    if !find_area {
        return distance_travelled / 2;
    }

    //If we are finding the area, we have the path, so scan over the map and count the ground tiles
    //that are within the loop, do this by scanning over and flipping a bit every time you
    //encounter the pipe edge

    //First, find the bounding box of the path
    let min_y = *path.iter().map(|(_, y)| y).min().unwrap();
    let min_x = *path.iter().map(|(x, _)| x).min().unwrap();
    let max_y = *path.iter().map(|(_, y)| y).max().unwrap();
    let max_x = *path.iter().map(|(x, _)| x).max().unwrap();

    let mut ground_tiles = 0;

    let mut loop_grounds = Vec::new();

    // Apparently, this is a knot theory problem, which... sucks
    // For part 2, I replaced the parts of the loop that had a connection to the row above by ! and
    //     the other parts of the loop by _. I removed all _ and counted the remaining parts of each
    //     line that had an odd number of ! before them
    // https://www.reddit.com/r/adventofcode/comments/18evyu9/comment/kcqkyrx/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

    for y in min_y..=max_y {
        let mut loop_count = 0;
        for x in min_x..=max_x {
            let segment = map.get(&(x, y));
            let above_connects = match segment {
                Some(Segment::Vertical) | Some(Segment::NE) | Some(Segment::NW) => true,
                _ => false,
            };

            //These vector membership checks are slow, I could replace these with sets, but whatever
            let in_loop = path.contains(&(x, y));

            if above_connects && in_loop {
                loop_count += 1;
            } else if in_loop {
                //do nothing
            } else if loop_count % 2 == 1 {
                ground_tiles += 1;
                loop_grounds.push((x, y));
            }
        } //end x
    } //end y

    #[cfg(test)]
    {
        dump_map_containing_paths(&path, &loop_grounds, map);
    }

    ground_tiles
}

#[aoc(day10, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solver(input, true)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        ".....
            .S-7.
            .|.|.
            .L-J.
            ....."
    }

    fn get_test_input_2() -> &'static str {
        "..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ..."
    }

    fn get_test_input_part2() -> &'static str {
        "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ..........."
    }

    fn get_test_input_part2_large() -> &'static str {
        ".F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ..."
    }

    fn get_test_input_part2_larger() -> &'static str {
        "FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L"
    }

    #[test]
    fn day10_part1() {
        assert_eq!(part1(&day10_parse(get_test_input())), 4);
        assert_eq!(part1(&day10_parse(get_test_input_2())), 8);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(part2(&day10_parse(get_test_input_part2())), 4);
        assert_eq!(part2(&day10_parse(get_test_input_part2_large())), 8);
        assert_eq!(part2(&day10_parse(get_test_input_part2_larger())), 10);
    }
}
