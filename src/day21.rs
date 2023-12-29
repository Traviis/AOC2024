use std::collections::{BTreeSet, VecDeque};

// start, rocks, max_x, max_y
type InputType = ((i32, i32), BTreeSet<(i32, i32)>, i32, i32);
type OutputType = u64;

#[aoc_generator(day21)]
fn day21_parse(input: &str) -> InputType {
    let rocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    //Yea, I'm going to duplicate this logic because I'm tired of trying to fight with rust
    let start = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                if c == 'S' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .next();

    let max_x = input.lines().next().unwrap().len() as i32;
    let max_y = input.lines().count() as i32;

    (start.unwrap(), rocks, max_x - 1, max_y - 1)
}

fn dump_map(
    start: (i32, i32),
    rocks: &BTreeSet<(i32, i32)>,
    seen: &BTreeSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            if rocks.contains(&(x, y)) {
                print!("#");
            } else if (x, y) == start {
                print!("S");
            } else if seen.contains(&(x, y)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1_impl(input: &InputType, steps: u64) -> OutputType {
    let (start, rocks, max_x, max_y) = input;
    let max_x = *max_x;
    let max_y = *max_y;

    let mut visited = BTreeSet::new();
    visited.insert(*start);

    let mut answer_set = BTreeSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((*start, steps));

    while let Some((pos, steps)) = queue.pop_front() {
        if steps % 2 == 0 {
            //You can jump back and forth between two points forever if you end on an even number
            answer_set.insert(pos);
        }

        if steps > 0 {
            for (dx, dy) in &[
                (pos.0, pos.1 + 1),
                (pos.0, pos.1 - 1),
                (pos.0 + 1, pos.1),
                (pos.0 - 1, pos.1),
            ] {
                //Don't go off the grid
                let dx = *dx;
                let dy = *dy;
                if dx < 0 || dy < 0 || dx > max_x || dy > max_y {
                    continue;
                }

                if rocks.contains(&(dx, dy)) || visited.contains(&(dx, dy)) {
                    continue;
                }

                visited.insert((dx, dy));
                queue.push_back(((dx, dy), steps - 1));
            }
        }
        #[cfg(test)]
        dump_map(*start, rocks, &visited, max_x, max_y);
    }

    answer_set.len() as u64
}

#[aoc(day21, part1)]
pub fn part1(input: &InputType) -> OutputType {
    part1_impl(input, 64)
}

#[aoc(day21, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ..........."
    }

    #[test]
    fn day21_part1() {
        assert_eq!(part1_impl(&day21_parse(get_test_input()), 6), 16);
    }

    #[test]
    fn day21_part2() {
        assert_eq!(part2(&day21_parse(get_test_input())), 0);
    }
}
