use std::collections::{BTreeSet, VecDeque};

// start, rocks, max_x, max_y
type InputType = ((i64, i64), BTreeSet<(i64, i64)>, i64, i64);
type OutputType = u64;

#[aoc_generator(day21)]
fn day21_parse(input: &str) -> InputType {
    let rocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i64, y as i64))
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
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .next();

    let max_x = input.lines().next().unwrap().len() as i64;
    let max_y = input.lines().count() as i64;

    (start.unwrap(), rocks, max_x - 1, max_y - 1)
}

fn dump_map(
    start: (i64, i64),
    rocks: &BTreeSet<(i64, i64)>,
    seen: &BTreeSet<(i64, i64)>,
    max_x: i64,
    max_y: i64,
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

fn fill(
    start: (i64, i64),
    rocks: &BTreeSet<(i64, i64)>,
    max_x: i64,
    max_y: i64,
    steps: i64,
) -> i64 {
    let mut visited = BTreeSet::new();
    visited.insert(start);

    let mut answer_set = BTreeSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((start, steps));

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
    }

    answer_set.len() as i64
}

fn part1_impl(input: &InputType, steps: u64) -> OutputType {
    let (start, rocks, max_x, max_y) = input;
    fill(*start, rocks, *max_x, *max_y, steps as i64) as u64
}

#[aoc(day21, part1)]
pub fn part1(input: &InputType) -> OutputType {
    part1_impl(input, 64)
}

#[aoc(day21, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let (start, rocks, max_x, max_y) = input;
    let max_x = *max_x;
    let max_y = *max_y;

    //I'm not even going to pretend that I know how I would have solved this without help
    // https://www.youtube.com/watch?v=9UOMZSL0JTg
    let steps = 26501365;
    let size = max_x + 1;
    assert_eq!(size / 2, start.0);
    assert_eq!(size / 2, start.1);
    assert_eq!(steps % size, size / 2);

    let grid_width = steps / size - 1;
    //How many grids do we have
    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    //How many reachable if we can completely reach the grid for even and odd ppints
    let odd_points = fill(start.clone(), rocks, max_x, max_y, size * 2 + 1);
    let even_points = fill(start.clone(), rocks, max_x, max_y, size * 2);

    //Let's do all the partial corners
    let corner_t = fill((size - 1, start.1), rocks, max_x, max_y, size - 1);
    let corner_r = fill((start.0, 0), rocks, max_x, max_y, size - 1);
    let corner_b = fill((0, start.1), rocks, max_x, max_y, size - 1);
    let corner_l = fill((start.0, size - 1), rocks, max_x, max_y, size - 1);

    //Now the small corner pieces
    let small_tr = fill((size - 1, 0), rocks, max_x, max_y, size / 2 - 1);
    let small_tl = fill((size - 1, size - 1), rocks, max_x, max_y, size / 2 - 1);
    let small_br = fill((0, 0), rocks, max_x, max_y, size / 2 - 1);
    let small_bl = fill((0, size - 1), rocks, max_x, max_y, size / 2 - 1);

    // Now the large edges (How are you supposed to figure this out?)
    let large_tr = fill((size - 1, 0), rocks, max_x, max_y, size * 3 / 2 - 1);
    let large_tl = fill((size - 1, size - 1), rocks, max_x, max_y, size * 3 / 2 - 1);
    let large_br = fill((0, 0), rocks, max_x, max_y, size * 3 / 2 - 1);
    let large_bl = fill((0, size - 1), rocks, max_x, max_y, size * 3 / 2 - 1);

    (odd * odd_points
        + even * even_points
        + corner_t
        + corner_r
        + corner_b
        + corner_l
        + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
        + grid_width * (large_tr + large_tl + large_br + large_bl)) as u64
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

    // #[test]
    // fn day21_part2() {
    //     assert_eq!(part2(&day21_parse(get_test_input())), 0);
    // }
}
