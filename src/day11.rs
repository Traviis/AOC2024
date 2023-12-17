use std::collections::{HashMap, HashSet};

type InputType = HashSet<(u64, u64)>;
type OutputType = u64;

fn expand_galaxy(input: &InputType, expansion: u64) -> InputType {
    //For every empty row and column, add a new empty row and column
    let mut new_input = input.clone();

    let max_x = *input.iter().map(|(x, _)| x).max().unwrap() as u64;
    let max_y = input.len() as u64;

    let mut expanded_rows = Vec::new();
    let mut expanded_columns = Vec::new();

    for y in 0..=max_y {
        let mut galaxy_in_row = false;
        for x in 0..=max_x {
            if let Some(_) = new_input.get(&(x, y as u64)) {
                galaxy_in_row = true;
                break;
            }
        }

        if !galaxy_in_row {
            expanded_rows.push(y);
        }
    }

    //Now expand columns
    for x in 0..=max_x {
        let mut galaxy_in_column = false;
        for y in 0..=max_y {
            if let Some(_) = new_input.get(&(x as u64, y)) {
                galaxy_in_column = true;
                break;
            }
        }
        if !galaxy_in_column {
            expanded_columns.push(x);
        }
    }

    let mut expanded_row_galaxies = HashMap::new();
    let mut expanded_column_galaxies = HashMap::new();

    for y in expanded_rows.iter() {
        //Expand the row by expansion if there is a galaxy in the row,
        // iterate over all galaxies and see if they fit this criteria
        let y = *y as usize;
        let valid_galaxies = input
            .iter()
            .filter(|(_, y2)| *y2 >= y as u64)
            .collect::<Vec<_>>();
        for (x, y) in valid_galaxies {
            if let Some(_) = new_input.get(&(*x, *y)) {
                *expanded_row_galaxies.entry((*x, *y)).or_insert(0) += expansion;
            }
        }
    }

    for x in expanded_columns.iter() {
        let x = *x as usize;
        let valid_galaxies = input
            .iter()
            .filter(|(x2, _)| *x2 >= x as u64)
            .collect::<Vec<_>>();
        for (x, y) in valid_galaxies {
            if let Some(_) = new_input.get(&(*x, *y)) {
                *expanded_column_galaxies.entry((*x, *y)).or_insert(0) += expansion;
            }
        }
    }

    #[cfg(test)]
    {
        println!("Expanded Rows: {:?}", expanded_rows);
        println!("Expanded Columns: {:?}", expanded_columns);
        println!("Expanded Row Galaxies: {:?}", expanded_row_galaxies);
        println!("Expanded Column Galaxies: {:?}", expanded_column_galaxies);
    }

    for (coord, expansion_val) in expanded_row_galaxies {
        let new_coord = (coord.0, (coord.1 + expansion_val) as u64);
        new_input.insert(new_coord);
        //Check if we have any galaxies in the expanded_column_galaxies because we just moved it
        if let Some(expansion_val) = expanded_column_galaxies.get(&coord) {
            expanded_column_galaxies.insert(new_coord, *expansion_val);
            expanded_column_galaxies.remove(&coord);
        }
        new_input.remove(&coord);
    }

    for (coord, expansion_val) in expanded_column_galaxies {
        new_input.insert(((coord.0 + expansion_val) as u64, coord.1));
        new_input.remove(&coord);
    }

    new_input
}

#[cfg(test)]
fn dump_galaxy(input: &InputType) {
    let max_x = input.iter().map(|(x, _)| x).max().unwrap();
    let max_y = input.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if let Some(_) = input.get(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc_generator(day11)]
fn day11_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.trim();
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as u64, y as u64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn manhatten_distance(a: (u64, u64), b: (u64, u64)) -> u64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}

#[aoc(day11, part1)]

pub fn part1(input: &InputType) -> OutputType {
    solver(input, 1)
}

pub fn solver(input: &InputType, expansion_rate: u64) -> OutputType {
    let input = expand_galaxy(input, expansion_rate);

    //could par_iter here, but it's not the slow part
    let mut seen_combo = HashSet::new();

    input
        .iter()
        .map(|(x, y)| {
            let mut dist = 0;
            for (x2, y2) in input.iter() {
                if x == x2 && y == y2 {
                    continue;
                }

                let mut sorted_combo = vec![(*x, *y), (*x2, *y2)];
                sorted_combo.sort();
                if seen_combo.contains(&sorted_combo) {
                    continue;
                }

                dist += manhatten_distance((*x, *y), (*x2, *y2));
                seen_combo.insert(sorted_combo);
            }
            dist as u64
        })
        .sum::<u64>()
}

#[aoc(day11, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solver(input, 1000000 - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    // #[test]
    // fn simple_galaxy_expansion() {
    // }

    #[test]
    fn galaxy_expansion() {
        let expanded_galaxy = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        let input = day11_parse(get_test_input());
        let expanded = expand_galaxy(&input, 1);
        let expanded_galaxy = day11_parse(expanded_galaxy);
        println!("Input");
        dump_galaxy(&input);
        println!("Expanded");
        dump_galaxy(&expanded);
        println!("Expanded Galaxy (target)");
        dump_galaxy(&expanded_galaxy);
        assert_eq!(expanded, expanded_galaxy);
    }

    #[test]
    fn day11_part1() {
        assert_eq!(part1(&day11_parse(get_test_input())), 374);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(solver(&day11_parse(get_test_input()), 9), 1030);
        assert_eq!(solver(&day11_parse(get_test_input()), 99), 8410);
    }
}
